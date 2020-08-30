use crate::hal::{blocking::delay::DelayUs, digital::v2::OutputPin};
use crate::{
    hal, mode, ActionInProgress, BitFlags, Ccs811, Ccs811Awake, Ccs811Device, Error, ErrorAwake,
    FirmwareMode, ModeChangeError, Register, SlaveAddr,
};
use core::marker::PhantomData;

impl<I2C, NWAKE, WAKEDELAY> Ccs811<I2C, NWAKE, WAKEDELAY, mode::Boot> {
    /// Create new instance of the CCS811 device.
    ///
    /// See `Ccs811Awake` for the case where the nWAKE pin is not used.
    pub fn new(i2c: I2C, address: SlaveAddr, n_wake_pin: NWAKE, wake_delay: WAKEDELAY) -> Self {
        Self::create(i2c, address.addr(), n_wake_pin, wake_delay)
    }
}

impl<I2C, NWAKE, WAKEDELAY, MODE> Ccs811<I2C, NWAKE, WAKEDELAY, MODE> {
    pub(crate) fn create(i2c: I2C, address: u8, n_wake_pin: NWAKE, wake_delay: WAKEDELAY) -> Self {
        Self::from_awake_dev(Ccs811Awake::create(i2c, address), n_wake_pin, wake_delay)
    }

    pub(crate) fn from_awake_dev(
        dev: Ccs811Awake<I2C, MODE>,
        n_wake_pin: NWAKE,
        wake_delay: WAKEDELAY,
    ) -> Self {
        Ccs811 {
            dev,
            n_wake_pin,
            wake_delay,
            _mode: PhantomData,
        }
    }
}

impl<I2C> Ccs811Awake<I2C, mode::Boot> {
    /// Create new instance of an already awake CCS811 device.
    pub fn new(i2c: I2C, address: SlaveAddr) -> Self {
        Self::create(i2c, address.addr())
    }
}

impl<I2C, MODE> Ccs811Awake<I2C, MODE> {
    pub(crate) fn create(i2c: I2C, address: u8) -> Self {
        Ccs811Awake {
            i2c,
            address,
            meas_mode_reg: 0,
            in_progress: ActionInProgress::None,
            _mode: PhantomData,
        }
    }
}

impl<I2C, E, MODE> Ccs811Awake<I2C, MODE>
where
    I2C: hal::blocking::i2c::Write<Error = E>,
{
    /// Destroy driver instance, return I²C bus instance.
    pub fn destroy(self) -> I2C {
        self.i2c
    }
}
impl<I2C, E, MODE> Ccs811Awake<I2C, MODE>
where
    I2C: hal::blocking::i2c::Write<Error = E> + hal::blocking::i2c::WriteRead<Error = E>,
{
    pub(crate) fn write_sw_reset(&mut self) -> Result<(), ErrorAwake<E>> {
        self.i2c
            .write(self.address, &[Register::SW_RESET, 0x11, 0xE5, 0x72, 0x8A])
            .map_err(ErrorAwake::I2C)
    }
}

impl<I2C, CommE, PinE, NWAKE, WAKEDELAY, MODE> Ccs811<I2C, NWAKE, WAKEDELAY, MODE>
where
    I2C: hal::blocking::i2c::Write<Error = CommE>,
    NWAKE: OutputPin<Error = PinE>,
    WAKEDELAY: DelayUs<u8>,
{
    /// Destroy driver instance, return I²C bus, nWAKE pin
    /// and wake delay instances.
    pub fn destroy(self) -> (I2C, NWAKE, WAKEDELAY) {
        (self.dev.destroy(), self.n_wake_pin, self.wake_delay)
    }

    pub(crate) fn on_awaken<T, F>(&mut self, f: F) -> Result<T, Error<CommE, PinE>>
    where
        F: FnOnce(&mut Self) -> Result<T, ErrorAwake<CommE>>,
    {
        self.n_wake_pin.set_low().map_err(Error::Pin)?;
        self.wake_delay.delay_us(50);
        let result = match f(self) {
            Ok(v) => Ok(v),
            Err(e) => Err(e.into()),
        };
        self.n_wake_pin.set_high().map_err(Error::Pin)?;
        self.wake_delay.delay_us(20);
        result
    }

    pub(crate) fn on_awaken_nb<T, F>(&mut self, f: F) -> nb::Result<T, Error<CommE, PinE>>
    where
        F: FnOnce(&mut Self) -> nb::Result<T, ErrorAwake<CommE>>,
    {
        self.n_wake_pin
            .set_low()
            .map_err(Error::Pin)
            .map_err(nb::Error::Other)?;
        self.wake_delay.delay_us(50);
        let result = match f(self) {
            Ok(v) => Ok(v),
            Err(nb::Error::Other(e)) => Err(nb::Error::Other(e.into())),
            Err(nb::Error::WouldBlock) => Err(nb::Error::WouldBlock),
        };
        self.n_wake_pin
            .set_high()
            .map_err(Error::Pin)
            .map_err(nb::Error::Other)?;
        self.wake_delay.delay_us(20);
        result
    }

    // Note: defining a type for the result would require inherent
    // associated items: https://github.com/rust-lang/rust/issues/8995
    // Note 2: is_verifying is always false after a mode change
    #[allow(clippy::type_complexity)]
    pub(crate) fn wrap_mode_change<TMODE, F>(
        mut self,
        f: F,
    ) -> Result<Ccs811<I2C, NWAKE, WAKEDELAY, TMODE>, ModeChangeError<Error<CommE, PinE>, Self>>
    where
        F: FnOnce(
            Ccs811Awake<I2C, MODE>,
        ) -> Result<
            Ccs811Awake<I2C, TMODE>,
            ModeChangeError<ErrorAwake<CommE>, Ccs811Awake<I2C, MODE>>,
        >,
    {
        if let Err(e) = self.n_wake_pin.set_low() {
            return Err(ModeChangeError::new(self, Error::Pin(e)));
        }
        self.wake_delay.delay_us(50);
        let Ccs811 {
            dev,
            mut n_wake_pin,
            mut wake_delay,
            ..
        } = self;
        let result = f(dev);
        if let Err(e) = n_wake_pin.set_high() {
            return match result {
                Ok(Ccs811Awake { i2c, address, .. }) => Err(ModeChangeError {
                    dev: Ccs811::create(i2c, address, n_wake_pin, wake_delay),
                    error: Error::Pin(e),
                }),
                Err(ModeChangeError { dev, error }) => Err(ModeChangeError {
                    dev: Ccs811::from_awake_dev(dev, n_wake_pin, wake_delay),
                    error: error.into(),
                }),
            };
        }
        wake_delay.delay_us(20);
        match result {
            Ok(dev) => Ok(Ccs811::from_awake_dev(dev, n_wake_pin, wake_delay)),
            Err(ModeChangeError { dev, error }) => Err(ModeChangeError {
                dev: Ccs811::from_awake_dev(dev, n_wake_pin, wake_delay),
                error: error.into(),
            }),
        }
    }
}

impl<I2C, E, MODE> Ccs811Device for Ccs811Awake<I2C, MODE>
where
    I2C: hal::blocking::i2c::Write<Error = E> + hal::blocking::i2c::WriteRead<Error = E>,
{
    type Error = ErrorAwake<E>;

    fn firmware_mode(&mut self) -> Result<FirmwareMode, Self::Error> {
        let status = self.read_status()?;
        let mode = if (status & BitFlags::FW_MODE) != 0 {
            FirmwareMode::Application
        } else {
            FirmwareMode::Boot
        };
        Ok(mode)
    }

    fn has_valid_app(&mut self) -> Result<bool, Self::Error> {
        let status = self.read_status()?;
        Ok((status & BitFlags::APP_VALID) != 0)
    }

    fn hardware_id(&mut self) -> Result<u8, Self::Error> {
        self.read_register_1byte(Register::HW_ID)
    }

    fn hardware_version(&mut self) -> Result<(u8, u8), Self::Error> {
        let version = self.read_register_1byte(Register::HW_VERSION)?;
        Ok(((version & 0xF0) >> 4, version & 0xF))
    }

    fn firmware_bootloader_version(&mut self) -> Result<(u8, u8, u8), Self::Error> {
        let version = self.read_register_2bytes(Register::FW_BOOT_VERSION)?;
        Ok(((version[0] & 0xF0) >> 4, version[0] & 0xF, version[1]))
    }

    fn firmware_application_version(&mut self) -> Result<(u8, u8, u8), Self::Error> {
        let version = self.read_register_2bytes(Register::FW_APP_VERSION)?;
        Ok(((version[0] & 0xF0) >> 4, version[0] & 0xF, version[1]))
    }
}

impl<I2C, CommE, PinE, NWAKE, WAKEDELAY, MODE> Ccs811Device for Ccs811<I2C, NWAKE, WAKEDELAY, MODE>
where
    I2C: hal::blocking::i2c::Write<Error = CommE> + hal::blocking::i2c::WriteRead<Error = CommE>,
    NWAKE: OutputPin<Error = PinE>,
    WAKEDELAY: DelayUs<u8>,
{
    type Error = Error<CommE, PinE>;

    fn firmware_mode(&mut self) -> Result<FirmwareMode, Self::Error> {
        self.on_awaken(|s| s.dev.firmware_mode())
    }

    fn has_valid_app(&mut self) -> Result<bool, Self::Error> {
        self.on_awaken(|s| s.dev.has_valid_app())
    }

    fn hardware_id(&mut self) -> Result<u8, Self::Error> {
        self.on_awaken(|s| s.dev.hardware_id())
    }

    fn hardware_version(&mut self) -> Result<(u8, u8), Self::Error> {
        self.on_awaken(|s| s.dev.hardware_version())
    }

    fn firmware_bootloader_version(&mut self) -> Result<(u8, u8, u8), Self::Error> {
        self.on_awaken(|s| s.dev.firmware_bootloader_version())
    }

    fn firmware_application_version(&mut self) -> Result<(u8, u8, u8), Self::Error> {
        self.on_awaken(|s| s.dev.firmware_application_version())
    }
}
