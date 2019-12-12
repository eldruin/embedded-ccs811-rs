use ccs811::{mode, Ccs811, SlaveAddr};
use embedded_hal_mock::{
    delay::MockNoop as NoDelay,
    i2c::{Mock as I2cMock, Transaction as I2cTrans},
    pin::Mock as PinMock,
};

pub const DEV_ADDR: u8 = 0x5A;
pub struct Register {}
#[allow(unused)]
impl Register {
    pub const STATUS: u8 = 0x00;
    pub const MEAS_MODE: u8 = 0x01;
    pub const ALG_RESULT_DATA: u8 = 0x02;
    pub const RAW_DATA: u8 = 0x03;
    pub const ENV_DATA: u8 = 0x05;
    pub const THRESHOLDS: u8 = 0x10;
    pub const HW_ID: u8 = 0x20;
    pub const HW_VERSION: u8 = 0x21;
    pub const FW_BOOT_VERSION: u8 = 0x23;
    pub const FW_APP_VERSION: u8 = 0x24;
    pub const ERROR_ID: u8 = 0xE0;
    pub const APP_ERASE: u8 = 0xF1;
    pub const APP_VERIFY: u8 = 0xF3;
    pub const APP_START: u8 = 0xF4;
    pub const SW_RESET: u8 = 0xFF;
}

pub struct BitFlags {}
#[allow(unused)]
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

pub fn new(
    transactions: &[I2cTrans],
    pin: PinMock,
) -> Ccs811<I2cMock, PinMock, NoDelay, mode::Boot> {
    Ccs811::new(
        I2cMock::new(transactions),
        SlaveAddr::default(),
        pin,
        NoDelay::new(),
    )
}

pub fn destroy<MODE>(sensor: Ccs811<I2cMock, PinMock, NoDelay, MODE>) {
    let (mut i2c, mut pin, _delay) = sensor.destroy();
    i2c.done();
    pin.done();
}

#[macro_export]
macro_rules! read_status_test {
    ($name:ident, $method:ident, $expected:expr, $value: expr) => {
        #[test]
        fn $name() {
            let nwake =
                PinMock::new(&[PinTrans::set(PinState::Low), PinTrans::set(PinState::High)]);
            let transactions = [I2cTrans::write_read(
                DEV_ADDR,
                vec![Register::STATUS],
                vec![$value],
            )];
            let mut sensor = new(&transactions, nwake);
            assert_eq!($expected, sensor.$method().unwrap());
            destroy(sensor);
        }
    };
}

#[macro_export]
macro_rules! assert_error {
    ($result:expr, $error:ident::$variant:ident) => {
        match $result {
            Err($error::$variant) => (),
            _ => panic!("Error not returned."),
        }
    };
}
