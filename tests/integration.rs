extern crate ccs811;
extern crate embedded_hal_mock as hal;
use hal::{
    i2c::Transaction as I2cTrans,
    pin::{Mock as PinMock, State as PinState, Transaction as PinTrans},
};

use ccs811::prelude::*;
use ccs811::Error;
mod common;
use common::{destroy, new, BitFlags as BF, Register, DEV_ADDR};

#[test]
fn can_create_and_destroy() {
    let nwake = PinMock::new(&[]);
    let sensor = new(&[], nwake);
    destroy(sensor);
}

#[test]
fn can_get_hw_id() {
    let nwake = PinMock::new(&[PinTrans::set(PinState::Low), PinTrans::set(PinState::High)]);
    let transactions = [
        I2cTrans::write_read(DEV_ADDR, vec![Register::HW_ID], vec![0x81]),
        I2cTrans::write_read(DEV_ADDR, vec![Register::STATUS], vec![0]),
    ];
    let mut sensor = new(&transactions, nwake);
    assert_eq!(0x81, sensor.hardware_id().unwrap());
    destroy(sensor);
}

#[test]
fn can_get_hw_version() {
    let nwake = PinMock::new(&[PinTrans::set(PinState::Low), PinTrans::set(PinState::High)]);
    let transactions = [
        I2cTrans::write_read(DEV_ADDR, vec![Register::HW_VERSION], vec![0x12]),
        I2cTrans::write_read(DEV_ADDR, vec![Register::STATUS], vec![0]),
    ];
    let mut sensor = new(&transactions, nwake);
    assert_eq!((1, 2), sensor.hardware_version().unwrap());
    destroy(sensor);
}

#[test]
fn can_get_fw_boot_version() {
    let nwake = PinMock::new(&[PinTrans::set(PinState::Low), PinTrans::set(PinState::High)]);
    let transactions = [
        I2cTrans::write_read(DEV_ADDR, vec![Register::FW_BOOT_VERSION], vec![0x12, 0x34]),
        I2cTrans::write_read(DEV_ADDR, vec![Register::STATUS], vec![0]),
    ];
    let mut sensor = new(&transactions, nwake);
    assert_eq!((1, 2, 0x34), sensor.firmware_bootloader_version().unwrap());
    destroy(sensor);
}

#[test]
fn can_start_app_mode() {
    let nwake = PinMock::new(&[PinTrans::set(PinState::Low), PinTrans::set(PinState::High)]);
    let transactions = [
        I2cTrans::write_read(DEV_ADDR, vec![Register::STATUS], vec![BF::APP_VALID]),
        I2cTrans::write(DEV_ADDR, vec![Register::APP_START]),
        I2cTrans::write_read(DEV_ADDR, vec![Register::STATUS], vec![0]),
    ];
    let sensor = new(&transactions, nwake);
    let sensor = sensor.app_start().ok().unwrap();
    destroy(sensor);
}

#[test]
fn cannot_start_app_mode_invalid_app() {
    let nwake = PinMock::new(&[PinTrans::set(PinState::Low), PinTrans::set(PinState::High)]);
    let transactions = [I2cTrans::write_read(
        DEV_ADDR,
        vec![Register::STATUS],
        vec![0],
    )];
    let sensor = new(&transactions, nwake);
    let result = sensor.app_start().err().unwrap();
    match result.error {
        Error::NoValidApp => (),
        _ => panic!("Invalid error"),
    }
    destroy(result.dev);
}
