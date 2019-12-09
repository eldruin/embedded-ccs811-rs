use crate::hal::digital::v2::OutputPin;
use crate::{
    hal, mode, ActionInProgress, BitFlags, Ccs811, Ccs811Awake, Ccs811BootMode, Ccs811Device,
    Error, ErrorAwake, ModeChangeError, Register,
};
use nb;

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
}

impl<I2C, CommE, PinE, NWAKE> Ccs811BootMode for Ccs811<I2C, NWAKE, mode::Boot>
where
    I2C: hal::blocking::i2c::Write<Error = CommE> + hal::blocking::i2c::WriteRead<Error = CommE>,
    NWAKE: OutputPin<Error = PinE>,
{
    type Error = Error<CommE, PinE>;
    type ModeChangeError = ModeChangeError<Self::Error, Self>;
    type TargetType = Ccs811<I2C, NWAKE, mode::App>;

    fn start_application(self) -> Result<Self::TargetType, Self::ModeChangeError> {
        self.wrap_mode_change(|s| s.start_application())
    }

    fn verify_application(&mut self) -> nb::Result<(), Self::Error> {
        self.on_awaken_nb(|s| s.dev.verify_application())
    }

    fn erase_application(&mut self) -> nb::Result<(), Self::Error> {
        self.on_awaken_nb(|s| s.dev.erase_application())
    }
}
