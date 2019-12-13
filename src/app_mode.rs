use crate::hal::{blocking::delay::DelayUs, digital::v2::OutputPin};
use crate::{
    hal, mode, register_access::get_errors, AlgorithmResult, BitFlags, Ccs811, Ccs811AppMode,
    Ccs811Awake, Error, ErrorAwake, InterruptMode, MeasurementMode, Register,
};

impl<I2C, E> Ccs811AppMode for Ccs811Awake<I2C, mode::App>
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
        Ok(handle_raw_data(data[0], data[1]))
    }

    fn data(&mut self) -> nb::Result<AlgorithmResult, Self::Error> {
        let mut data = [0; 8];
        self.i2c
            .write_read(self.address, &[Register::ALG_RESULT_DATA], &mut data)
            .map_err(ErrorAwake::I2C)?;
        let status = data[4];
        if (status & BitFlags::ERROR) != 0 {
            get_errors(data[5]).map_err(ErrorAwake::Device)?;
        } else if (status & BitFlags::DATA_READY) == 0 {
            return Err(nb::Error::WouldBlock);
        }
        let raw = handle_raw_data(data[6], data[7]);
        Ok(AlgorithmResult {
            eco2: (u16::from(data[0]) << 8) | u16::from(data[1]),
            etvoc: (u16::from(data[2]) << 8) | u16::from(data[3]),
            raw_current: raw.0,
            raw_voltage: raw.1,
        })
    }

    fn set_environment(
        &mut self,
        humidity_percentage: f32,
        temperature_celsius: f32,
    ) -> Result<(), Self::Error> {
        if humidity_percentage < 0.0
            || humidity_percentage > 100.0
            || temperature_celsius > 254.998_05
        {
            return Err(ErrorAwake::InvalidInputData);
        }
        let raw_humidity = get_raw_humidity(humidity_percentage);
        let raw_temp = get_raw_temperature(temperature_celsius);
        let raw = [
            Register::ENV_DATA,
            raw_humidity.0,
            raw_humidity.1,
            raw_temp.0,
            raw_temp.1,
        ];
        self.i2c
            .write(self.address, &raw)
            .map_err(ErrorAwake::I2C)?;
        self.check_status_error()
    }

    fn baseline(&mut self) -> Result<[u8; 2], Self::Error> {
        self.read_register_2bytes(Register::BASELINE)
    }

    fn set_baseline(&mut self, baseline: [u8; 2]) -> Result<(), Self::Error> {
        self.i2c
            .write(
                self.address,
                &[Register::BASELINE, baseline[0], baseline[1]],
            )
            .map_err(ErrorAwake::I2C)?;
        self.check_status_error()
    }

    fn set_eco2_thresholds(
        &mut self,
        low_to_medium: u16,
        medium_to_high: u16,
    ) -> Result<(), Self::Error> {
        self.i2c
            .write(
                self.address,
                &[
                    Register::THRESHOLDS,
                    (low_to_medium >> 8) as u8,
                    low_to_medium as u8,
                    (medium_to_high >> 8) as u8,
                    medium_to_high as u8,
                ],
            )
            .map_err(ErrorAwake::I2C)?;
        self.check_status_error()
    }

    fn set_interrupt_mode(&mut self, mode: InterruptMode) -> Result<(), Self::Error> {
        let int_mask = match mode {
            InterruptMode::Disabled => 0,
            InterruptMode::OnDataReady => BitFlags::INTERRUPT,
            InterruptMode::OnThresholdCrossed => BitFlags::INTERRUPT | BitFlags::THRESH,
        };
        let meas_mode = (self.meas_mode_reg & (0b111 << 4)) | int_mask;
        self.write_register_1byte(Register::MEAS_MODE, meas_mode)?;
        self.meas_mode_reg = meas_mode;
        Ok(())
    }
}

fn get_raw_humidity(humidity_percentage: f32) -> (u8, u8) {
    get_raw_environment_data(humidity_percentage)
}

fn get_raw_temperature(temperature_celsius: f32) -> (u8, u8) {
    let value = temperature_celsius + 25.0;
    if value < 0.0 {
        (0, 0)
    } else {
        get_raw_environment_data(value)
    }
}

fn get_raw_environment_data(value: f32) -> (u8, u8) {
    let main = (value as u8) << 1;
    let rest = value - f32::from(value as u8);
    let rest = (rest * 512.0) as u16;
    (main | (((rest & (1 << 8)) >> 8) as u8), rest as u8)
}

fn handle_raw_data(data0: u8, data1: u8) -> (u8, u16) {
    (
        (data1 >> 2) as u8,
        u16::from(data0) | (u16::from(data1 & 0x3) << 8),
    )
}

impl<I2C, CommE, PinE, NWAKE, WAKEDELAY> Ccs811AppMode for Ccs811<I2C, NWAKE, WAKEDELAY, mode::App>
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

    fn data(&mut self) -> nb::Result<AlgorithmResult, Self::Error> {
        self.on_awaken_nb(|s| s.dev.data())
    }

    fn baseline(&mut self) -> Result<[u8; 2], Self::Error> {
        self.on_awaken(|s| s.dev.baseline())
    }

    fn set_baseline(&mut self, baseline: [u8; 2]) -> Result<(), Self::Error> {
        self.on_awaken(|s| s.dev.set_baseline(baseline))
    }

    fn set_environment(
        &mut self,
        humidity_percentage: f32,
        temperature_celsius: f32,
    ) -> Result<(), Self::Error> {
        self.on_awaken(|s| {
            s.dev
                .set_environment(humidity_percentage, temperature_celsius)
        })
    }

    fn set_eco2_thresholds(
        &mut self,
        low_to_medium: u16,
        medium_to_high: u16,
    ) -> Result<(), Self::Error> {
        self.on_awaken(|s| s.dev.set_eco2_thresholds(low_to_medium, medium_to_high))
    }

    fn set_interrupt_mode(&mut self, mode: InterruptMode) -> Result<(), Self::Error> {
        self.on_awaken(|s| s.dev.set_interrupt_mode(mode))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert_humidity() {
        assert_eq!((0, 0), get_raw_humidity(0.0));
        assert_eq!((0x64, 0), get_raw_humidity(50.0));
        assert_eq!((0x61, 0), get_raw_humidity(48.5));
        assert_eq!((0x60, 0x80), get_raw_humidity(48.25));
        assert_eq!((0x60, 0x40), get_raw_humidity(48.125));
        assert_eq!((0x60, 0x20), get_raw_humidity(48.0625));
        assert_eq!((0x60, 0x10), get_raw_humidity(48.03125));
        assert_eq!((0x60, 0x08), get_raw_humidity(48.015_625));
        assert_eq!((0x60, 0x04), get_raw_humidity(48.007_813));
        assert_eq!((0x60, 0x02), get_raw_humidity(48.003_906));
        assert_eq!((0x60, 0x01), get_raw_humidity(48.001_953));
        assert_eq!((0x61, 0xFF), get_raw_humidity(48.998_047));
    }

    #[test]
    fn convert_temperature() {
        assert_eq!((0, 0), get_raw_temperature(-25.5));
        assert_eq!((0, 0), get_raw_temperature(-25.0));
        assert_eq!((0x64, 0), get_raw_temperature(25.0));
        assert_eq!((0x61, 0), get_raw_temperature(23.5));
    }
}
