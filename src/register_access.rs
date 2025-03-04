use crate::{hal, Ccs811Awake, DeviceErrors, ErrorAwake};

pub(crate) struct Register {}
impl Register {
    pub const STATUS: u8 = 0x00;
    pub const MEAS_MODE: u8 = 0x01;
    pub const ALG_RESULT_DATA: u8 = 0x02;
    pub const RAW_DATA: u8 = 0x03;
    pub const ENV_DATA: u8 = 0x05;
    pub const THRESHOLDS: u8 = 0x10;
    pub const BASELINE: u8 = 0x11;
    pub const HW_ID: u8 = 0x20;
    pub const HW_VERSION: u8 = 0x21;
    pub const FW_BOOT_VERSION: u8 = 0x23;
    pub const FW_APP_VERSION: u8 = 0x24;
    pub const ERROR_ID: u8 = 0xE0;
    pub const APP_ERASE: u8 = 0xF1;
    pub const REG_BOOT_APP: u8 = 0xF2;
    pub const APP_VERIFY: u8 = 0xF3;
    pub const APP_START: u8 = 0xF4;
    pub const SW_RESET: u8 = 0xFF;
}

pub(crate) struct BitFlags {}
impl BitFlags {
    pub const DATA_READY: u8 = 1 << 3;
    pub const APP_VALID: u8 = 1 << 4;
    pub const APP_VERIFY: u8 = 1 << 5;
    pub const APP_ERASE: u8 = 1 << 6;
    pub const FW_MODE: u8 = 1 << 7;
    pub const ERROR: u8 = 1;
    pub const WRITE_REG_INVALID: u8 = 1;
    pub const READ_REG_INVALID: u8 = 1 << 1;
    pub const MEASMODE_INVALID: u8 = 1 << 2;
    pub const MAX_RESISTANCE: u8 = 1 << 3;
    pub const HEATER_FAULT: u8 = 1 << 4;
    pub const HEATER_SUPPLY: u8 = 1 << 5;
    pub const INTERRUPT: u8 = 1 << 3;
    pub const THRESH: u8 = 1 << 2;
}

impl<I2C, E, MODE> Ccs811Awake<I2C, MODE>
where
    I2C: hal::i2c::I2c<Error = E>,
{
    pub(crate) fn check_status_error(&mut self) -> Result<(), ErrorAwake<E>> {
        self.read_status().map(drop)
    }

    pub(crate) fn read_status(&mut self) -> Result<u8, ErrorAwake<E>> {
        let mut data = [0];
        self.i2c
            .write_read(self.address, &[Register::STATUS], &mut data)
            .map_err(ErrorAwake::I2C)?;
        let status = data[0];
        if (status & BitFlags::ERROR) != 0 {
            self.i2c
                .write_read(self.address, &[Register::ERROR_ID], &mut data)
                .map_err(ErrorAwake::I2C)?;
            get_errors(data[0]).map_err(ErrorAwake::Device)?;
        }
        Ok(status)
    }

    pub(crate) fn read_register_1byte(&mut self, register: u8) -> Result<u8, ErrorAwake<E>> {
        let mut data = [0];
        self.read_register(register, &mut data).and(Ok(data[0]))
    }

    pub(crate) fn read_register_2bytes(&mut self, register: u8) -> Result<[u8; 2], ErrorAwake<E>> {
        let mut data = [0; 2];
        self.read_register(register, &mut data).and(Ok(data))
    }

    pub(crate) fn read_register(
        &mut self,
        register: u8,
        data: &mut [u8],
    ) -> Result<(), ErrorAwake<E>> {
        self.i2c
            .write_read(self.address, &[register], data)
            .map_err(ErrorAwake::I2C)?;
        self.check_status_error()
    }

    pub(crate) fn write_register_no_data(&mut self, register: u8) -> Result<(), ErrorAwake<E>> {
        self.i2c
            .write(self.address, &[register])
            .map_err(ErrorAwake::I2C)?;
        self.check_status_error()
    }

    pub(crate) fn write_register_1byte(
        &mut self,
        register: u8,
        data: u8,
    ) -> Result<(), ErrorAwake<E>> {
        self.i2c
            .write(self.address, &[register, data])
            .map_err(ErrorAwake::I2C)?;
        self.check_status_error()
    }
}

pub(crate) fn get_errors(error_id: u8) -> Result<(), DeviceErrors> {
    let mut has_error = false;
    let mut errors = DeviceErrors::default();
    if (error_id & BitFlags::WRITE_REG_INVALID) != 0 {
        errors.invalid_register_write = true;
        has_error = true;
    }
    if (error_id & BitFlags::READ_REG_INVALID) != 0 {
        errors.invalid_register_read = true;
        has_error = true;
    }
    if (error_id & BitFlags::MEASMODE_INVALID) != 0 {
        errors.invalid_measurement = true;
        has_error = true;
    }
    if (error_id & BitFlags::MAX_RESISTANCE) != 0 {
        errors.max_resistance = true;
        has_error = true;
    }
    if (error_id & BitFlags::HEATER_FAULT) != 0 {
        errors.heater_fault = true;
        has_error = true;
    }
    if (error_id & BitFlags::HEATER_SUPPLY) != 0 {
        errors.heater_supply = true;
        has_error = true;
    }
    if has_error {
        Err(errors)
    } else {
        Ok(())
    }
}
