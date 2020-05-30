use crate::hal::{
    blocking::delay::{DelayMs, DelayUs},
    digital::v2::OutputPin,
};
use crate::{
    hal, mode, ActionInProgress, BitFlags, Ccs811, Ccs811Awake, Ccs811BootMode, Ccs811Device,
    Error, ErrorAwake, ModeChangeError, Register,
};

impl<I2C, E> Ccs811BootMode for Ccs811Awake<I2C, mode::Boot>
where
    I2C: hal::blocking::i2c::Write<Error = E> + hal::blocking::i2c::WriteRead<Error = E>,
{
    type Error = ErrorAwake<E>;
    type ModeChangeError = ModeChangeError<Self::Error, Self>;
    type TargetType = Ccs811Awake<I2C, mode::App>;

    fn start_application(mut self) -> Result<Self::TargetType, Self::ModeChangeError> {
        match self.has_valid_app() {
            Err(e) => Err(ModeChangeError::new(self, e)),
            Ok(is_valid) => {
                if !is_valid {
                    Err(ModeChangeError::new(self, ErrorAwake::NoValidApp))
                } else {
                    match self.write_register_no_data(Register::APP_START) {
                        Err(e) => Err(ModeChangeError::new(self, e)),
                        Ok(_) => Ok(Ccs811Awake::create(self.i2c, self.address)),
                    }
                }
            }
        }
    }

    fn verify_application(&mut self) -> nb::Result<(), Self::Error> {
        let status = self.read_status().map_err(nb::Error::Other)?;
        let verified = (status & BitFlags::APP_VERIFY) != 0;
        if !verified {
            if self.in_progress == ActionInProgress::Verification {
                Err(nb::Error::WouldBlock)
            } else {
                let result = self
                    .i2c
                    .write(self.address, &[Register::APP_VERIFY])
                    .map_err(ErrorAwake::I2C);
                match result {
                    Ok(_) => {
                        self.in_progress = ActionInProgress::Verification;
                        Err(nb::Error::WouldBlock)
                    }
                    Err(e) => Err(nb::Error::Other(e)),
                }
            }
        } else {
            self.in_progress = ActionInProgress::None;
            Ok(())
        }
    }

    fn erase_application(&mut self) -> nb::Result<(), Self::Error> {
        let status = self.read_status().map_err(nb::Error::Other)?;
        let erased = (status & BitFlags::APP_ERASE) != 0;
        if !erased {
            if self.in_progress == ActionInProgress::Erase {
                Err(nb::Error::WouldBlock)
            } else {
                let result = self
                    .i2c
                    .write(self.address, &[Register::APP_ERASE, 0xE7, 0xA7, 0xE6, 0x09])
                    .map_err(ErrorAwake::I2C);
                match result {
                    Ok(_) => {
                        self.in_progress = ActionInProgress::Erase;
                        Err(nb::Error::WouldBlock)
                    }
                    Err(e) => Err(nb::Error::Other(e)),
                }
            }
        } else {
            self.in_progress = ActionInProgress::None;
            Ok(())
        }
    }

    fn download_application<D: DelayMs<u16>>(
        &mut self,
        bin: &[u8],
        delay: &mut D,
    ) -> Result<(), Self::Error> {
        if bin.len() % 8 != 0 {
            return Err(ErrorAwake::InvalidInputData);
        }
        let mut data = [0; 9];
        data[0] = Register::REG_BOOT_APP;
        for chunk in bin.chunks(8) {
            data[1..].copy_from_slice(chunk);
            self.i2c
                .write(self.address, &data)
                .map_err(ErrorAwake::I2C)?;
            delay.delay_ms(50);
        }
        self.check_status_error()
    }

    fn update_application<D: DelayMs<u16>>(
        &mut self,
        bin: &[u8],
        delay: &mut D,
    ) -> Result<(), Self::Error> {
        self.write_sw_reset()?;
        delay.delay_ms(50); // Theoretically 2ms is enough
        match self.erase_application() {
            Err(nb::Error::WouldBlock) => delay.delay_ms(500),
            Err(nb::Error::Other(e)) => return Err(e),
            Ok(_) => (),
        }
        loop {
            match self.erase_application() {
                Err(nb::Error::WouldBlock) => (),
                Err(nb::Error::Other(e)) => return Err(e),
                Ok(_) => break,
            }
        }
        self.download_application(bin, delay)?;
        match self.verify_application() {
            Err(nb::Error::WouldBlock) => delay.delay_ms(70),
            Err(nb::Error::Other(e)) => return Err(e),
            Ok(_) => (),
        }
        loop {
            match self.verify_application() {
                Err(nb::Error::WouldBlock) => (),
                Err(nb::Error::Other(e)) => return Err(e),
                Ok(_) => break,
            }
        }
        Ok(())
    }

    // Note: is_verifying is false after a reset
    fn software_reset(&mut self) -> Result<(), Self::Error> {
        self.write_sw_reset()
    }
}

impl<I2C, CommE, PinE, NWAKE, WAKEDELAY> Ccs811BootMode
    for Ccs811<I2C, NWAKE, WAKEDELAY, mode::Boot>
where
    I2C: hal::blocking::i2c::Write<Error = CommE> + hal::blocking::i2c::WriteRead<Error = CommE>,
    NWAKE: OutputPin<Error = PinE>,
    WAKEDELAY: DelayUs<u8>,
{
    type Error = Error<CommE, PinE>;
    type ModeChangeError = ModeChangeError<Self::Error, Self>;
    type TargetType = Ccs811<I2C, NWAKE, WAKEDELAY, mode::App>;

    fn start_application(self) -> Result<Self::TargetType, Self::ModeChangeError> {
        self.wrap_mode_change(|s| s.start_application())
    }

    fn verify_application(&mut self) -> nb::Result<(), Self::Error> {
        self.on_awaken_nb(|s| s.dev.verify_application())
    }

    fn erase_application(&mut self) -> nb::Result<(), Self::Error> {
        self.on_awaken_nb(|s| s.dev.erase_application())
    }

    fn download_application<D: DelayMs<u16>>(
        &mut self,
        bin: &[u8],
        delay: &mut D,
    ) -> Result<(), Self::Error> {
        self.on_awaken(|s| s.dev.download_application(bin, delay))
    }

    fn update_application<D: DelayMs<u16>>(
        &mut self,
        bin: &[u8],
        delay: &mut D,
    ) -> Result<(), Self::Error> {
        self.on_awaken(|s| s.dev.update_application(bin, delay))
    }

    fn software_reset(&mut self) -> Result<(), Self::Error> {
        self.on_awaken(|s| s.dev.software_reset())
    }
}
