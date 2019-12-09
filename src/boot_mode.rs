use crate::hal::digital::v2::OutputPin;
use crate::{
    hal, mode, Ccs811, Ccs811Awake, Ccs811BootMode, Ccs811Device, Error, ErrorAwake,
    ModeChangeError, Register,
};

impl<I2C, E> Ccs811BootMode for Ccs811Awake<I2C, mode::Boot>
where
    I2C: hal::blocking::i2c::Write<Error = E> + hal::blocking::i2c::WriteRead<Error = E>,
{
    type ModeChangeError = ModeChangeError<ErrorAwake<E>, Self>;
    type TargetType = Ccs811Awake<I2C, mode::App>;

    fn app_start(mut self) -> Result<Self::TargetType, Self::ModeChangeError> {
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
}

impl<I2C, CommE, PinE, NWAKE> Ccs811BootMode for Ccs811<I2C, NWAKE, mode::Boot>
where
    I2C: hal::blocking::i2c::Write<Error = CommE> + hal::blocking::i2c::WriteRead<Error = CommE>,
    NWAKE: OutputPin<Error = PinE>,
{
    type ModeChangeError = ModeChangeError<Error<CommE, PinE>, Self>;
    type TargetType = Ccs811<I2C, NWAKE, mode::App>;

    fn app_start(self) -> Result<Self::TargetType, Self::ModeChangeError> {
        self.wrap_mode_change(|s| s.app_start())
    }
}
