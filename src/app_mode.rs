use crate::hal::{blocking::delay::DelayUs, digital::v2::OutputPin};
use crate::{
    hal, mode, BitFlags, Ccs811, Ccs811AppMode, Ccs811Awake, Error, ErrorAwake, MeasurementMode,
    Register,
};

impl<I2C, E> Ccs811AppMode for Ccs811Awake<I2C, mode::Boot>
where
    I2C: hal::blocking::i2c::Write<Error = E> + hal::blocking::i2c::WriteRead<Error = E>,
{
    type Error = ErrorAwake<E>;
    fn set_mode(&mut self, mode: MeasurementMode) -> Result<(), Self::Error> {
        let idle_mode = self.meas_mode_reg & 0b0000_1100;
        let meas_mode = match mode {
            MeasurementMode::Idle => idle_mode,
            MeasurementMode::ConstantPower1s => idle_mode | 1 << 4,
            MeasurementMode::PulseHeating10s => idle_mode | 2 << 4,
            MeasurementMode::LowPowerPulseHeating60s => idle_mode | 3 << 4,
            MeasurementMode::ConstantPower250ms => idle_mode | 4 << 4,
        };
        self.write_register_1byte(Register::MEAS_MODE, meas_mode)?;
        self.meas_mode_reg = meas_mode;
        Ok(())
    }

    fn has_data_ready(&mut self) -> Result<bool, Self::Error> {
        let status = self.read_status()?;
        Ok((status & BitFlags::DATA_READY) != 0)
    }

    fn raw_data(&mut self) -> Result<(u8, u16), Self::Error> {
        let data = self.read_register_2bytes(Register::RAW_DATA)?;
        Ok((
            (data[1] >> 2) as u8,
            u16::from(data[0]) | (u16::from(data[1] & 0x3) << 8),
        ))
    }
}

impl<I2C, CommE, PinE, NWAKE, WAKEDELAY> Ccs811AppMode for Ccs811<I2C, NWAKE, WAKEDELAY, mode::Boot>
where
    I2C: hal::blocking::i2c::Write<Error = CommE> + hal::blocking::i2c::WriteRead<Error = CommE>,
    NWAKE: OutputPin<Error = PinE>,
    WAKEDELAY: DelayUs<u8>,
{
    type Error = Error<CommE, PinE>;

    fn set_mode(&mut self, mode: MeasurementMode) -> Result<(), Self::Error> {
        self.on_awaken(|s| s.dev.set_mode(mode))
    }

    fn has_data_ready(&mut self) -> Result<bool, Self::Error> {
        self.on_awaken(|s| s.dev.has_data_ready())
    }

    fn raw_data(&mut self) -> Result<(u8, u16), Self::Error> {
        self.on_awaken(|s| s.dev.raw_data())
    }
}
