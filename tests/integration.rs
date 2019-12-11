use ccs811::{prelude::*, FirmwareMode as FwMode};
use embedded_hal_mock::{
    i2c::Transaction as I2cTrans,
    pin::{Mock as PinMock, State as PinState, Transaction as PinTrans},
};
mod common;
use crate::common::{destroy, new, BitFlags as BF, Register, DEV_ADDR};

#[test]
fn can_create_and_destroy() {
    let nwake = PinMock::new(&[]);
    let sensor = new(&[], nwake);
    destroy(sensor);
}

macro_rules! get_test {
    ($name:ident, $method:ident, $reg:ident, $value:expr, $expected:expr) => {
        #[test]
        fn $name() {
            let nwake =
                PinMock::new(&[PinTrans::set(PinState::Low), PinTrans::set(PinState::High)]);
            let transactions = [
                I2cTrans::write_read(DEV_ADDR, vec![Register::$reg], $value),
                I2cTrans::write_read(DEV_ADDR, vec![Register::STATUS], vec![0]),
            ];
            let mut sensor = new(&transactions, nwake);
            assert_eq!($expected, sensor.$method().unwrap());
            destroy(sensor);
        }
    };
}

get_test!(can_get_hw_id, hardware_id, HW_ID, vec![0x81], 0x81);
get_test!(
    can_get_hw_version,
    hardware_version,
    HW_VERSION,
    vec![0x12],
    (1, 2)
);
get_test!(
    can_get_fw_boot_version,
    firmware_bootloader_version,
    FW_BOOT_VERSION,
    vec![0x12, 0x34],
    (1, 2, 0x34)
);
get_test!(
    can_get_fw_app_version,
    firmware_application_version,
    FW_APP_VERSION,
    vec![0x12, 0x34],
    (1, 2, 0x34)
);

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

read_status_test!(can_get_invalid_app, has_valid_app, false, 0);
read_status_test!(can_get_valid_app, has_valid_app, true, BF::APP_VALID);
read_status_test!(fw_mode_boot, firmware_mode, FwMode::Boot, 0);
read_status_test!(fw_mode_app, firmware_mode, FwMode::Application, BF::FW_MODE);

#[test]
fn can_do_software_reset() {
    let nwake = PinMock::new(&[PinTrans::set(PinState::Low), PinTrans::set(PinState::High)]);
    let transactions = [
        I2cTrans::write(DEV_ADDR, vec![Register::SW_RESET, 0x11, 0xE5, 0x72, 0x8A]),
        I2cTrans::write_read(DEV_ADDR, vec![Register::STATUS], vec![0]),
    ];
    let sensor = new(&transactions, nwake);
    let sensor = sensor.software_reset().ok().unwrap();
    destroy(sensor);
}
