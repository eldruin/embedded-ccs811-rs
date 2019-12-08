use core::marker::PhantomData;
use hal::digital::v2::OutputPin;
use {
    hal, mode, BitFlags, Ccs811, Ccs811Awake, Ccs811Device, Error, ErrorAwake, Register, SlaveAddr,
};

impl<I2C, NWAKE> Ccs811<I2C, NWAKE, mode::Boot> {
    /// Create new instance of the CCS811 device.
    ///
    /// See `Ccs811Awake` for the case where the nWAKE pin is not used.
    pub fn new(i2c: I2C, n_wake_pin: NWAKE, address: SlaveAddr) -> Self {
        Ccs811 {
            dev: Ccs811Awake::new(i2c, address),
            n_wake_pin,
            _mode: PhantomData,
        }
    }
}

impl<I2C> Ccs811Awake<I2C, mode::Boot> {
    /// Create new instance of an already awake CCS811 device.
    pub fn new(i2c: I2C, address: SlaveAddr) -> Self {
        Ccs811Awake {
            i2c,
            address: address.addr(),
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

impl<I2C, CommE, PinE, NWAKE, MODE> Ccs811<I2C, NWAKE, MODE>
where
    I2C: hal::blocking::i2c::Write<Error = CommE>,
    NWAKE: OutputPin<Error = PinE>,
{
    /// Destroy driver instance, return I²C bus instance and nWAKE pin.
    pub fn destroy(self) -> (I2C, NWAKE) {
        (self.dev.destroy(), self.n_wake_pin)
    }

    fn on_awaken<T, F>(&mut self, f: F) -> Result<T, Error<CommE, PinE>>
    where
        F: FnOnce(&mut Self) -> Result<T, ErrorAwake<CommE>>,
    {
        self.n_wake_pin.set_low().map_err(Error::Pin)?;
        let result = match f(self) {
            Ok(v) => Ok(v),
            Err(e) => Err(e.into()),
        };
        self.n_wake_pin.set_high().map_err(Error::Pin)?;
        result
    }
}

impl<I2C, E, MODE> Ccs811Device for Ccs811Awake<I2C, MODE>
where
    I2C: hal::blocking::i2c::Write<Error = E> + hal::blocking::i2c::WriteRead<Error = E>,
{
    type Error = ErrorAwake<E>;

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
}

impl<I2C, CommE, PinE, NWAKE, MODE> Ccs811Device for Ccs811<I2C, NWAKE, MODE>
where
    I2C: hal::blocking::i2c::Write<Error = CommE> + hal::blocking::i2c::WriteRead<Error = CommE>,
    NWAKE: OutputPin<Error = PinE>,
{
    type Error = Error<CommE, PinE>;

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
}
