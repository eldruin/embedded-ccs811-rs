use ccs811::{prelude::*, AlgorithmResult, Error, MeasurementMode};
use embedded_hal_mock::{
    i2c::Transaction as I2cTrans,
    pin::{Mock as PinMock, State as PinState, Transaction as PinTrans},
};
mod common;
use crate::common::{destroy, new, BitFlags as BF, Register, DEV_ADDR};
use nb::Error as NbError;

macro_rules! set_mode_test {
    ($name:ident, $mode:ident, $value:expr) => {
        #[test]
        fn $name() {
            let nwake =
                PinMock::new(&[PinTrans::set(PinState::Low), PinTrans::set(PinState::High)]);
            let transactions = [
                I2cTrans::write(DEV_ADDR, vec![Register::MEAS_MODE, $value]),
                I2cTrans::write_read(DEV_ADDR, vec![Register::STATUS], vec![0]),
            ];
            let mut sensor = new(&transactions, nwake);
            sensor.set_mode(MeasurementMode::$mode).unwrap();
            destroy(sensor);
        }
    };
}

set_mode_test!(can_set_mode_0, Idle, 0);
set_mode_test!(can_set_mode_1, ConstantPower1s, 1 << 4);
set_mode_test!(can_set_mode_2, PulseHeating10s, 2 << 4);
set_mode_test!(can_set_mode_3, LowPowerPulseHeating60s, 3 << 4);
set_mode_test!(can_set_mode_4, ConstantPower250ms, 4 << 4);

read_status_test!(has_data_ready, has_data_ready, true, BF::DATA_READY);
read_status_test!(has_no_data_ready, has_data_ready, false, 0);

#[test]
fn can_read_raw_data() {
    let nwake = PinMock::new(&[PinTrans::set(PinState::Low), PinTrans::set(PinState::High)]);
    let transactions = [
        I2cTrans::write_read(DEV_ADDR, vec![Register::RAW_DATA], vec![0x34, 0x52]),
        I2cTrans::write_read(DEV_ADDR, vec![Register::STATUS], vec![0]),
    ];
    let mut sensor = new(&transactions, nwake);
    assert_eq!((0x50 >> 2, 0x234), sensor.raw_data().unwrap());
    destroy(sensor);
}

#[test]
fn can_read_alg_result_data() {
    let nwake = PinMock::new(&[
        PinTrans::set(PinState::Low),
        PinTrans::set(PinState::High),
        PinTrans::set(PinState::Low),
        PinTrans::set(PinState::High),
    ]);
    let transactions = [
        I2cTrans::write_read(
            DEV_ADDR,
            vec![Register::ALG_RESULT_DATA],
            vec![0, 0, 0, 0, 0, 0, 0, 0],
        ),
        I2cTrans::write_read(
            DEV_ADDR,
            vec![Register::ALG_RESULT_DATA],
            vec![0x12, 0x34, 0x56, 0x78, BF::DATA_READY, 0, 0x91, 0x52],
        ),
    ];
    let mut sensor = new(&transactions, nwake);
    let expected = AlgorithmResult {
        eco2: 0x1234,
        etvoc: 0x5678,
        raw_current: 0x50 >> 2,
        raw_voltage: 0x291,
    };
    assert_error!(sensor.data(), NbError::WouldBlock);
    assert_eq!(expected, sensor.data().unwrap());
    destroy(sensor);
}

macro_rules! invalid_env_test {
    ($name:ident, $rh:expr, $temp:expr) => {
        #[test]
        fn $name() {
            let nwake =
                PinMock::new(&[PinTrans::set(PinState::Low), PinTrans::set(PinState::High)]);
            let mut sensor = new(&[], nwake);
            assert_error!(sensor.set_environment($rh, $temp), Error::InvalidInputData);
            destroy(sensor);
        }
    };
}

invalid_env_test!(cannot_set_negative_humidity, -1.0, 0.0);
invalid_env_test!(cannot_set_too_high_humidity, 100.1, 0.0);
invalid_env_test!(cannot_set_too_high_temp, 0.0, 255.0);

#[test]
fn can_set_environment_params() {
    let nwake = PinMock::new(&[PinTrans::set(PinState::Low), PinTrans::set(PinState::High)]);
    let transactions = [
        I2cTrans::write(DEV_ADDR, vec![Register::ENV_DATA, 0x60, 0x80, 0x64, 0x40]),
        I2cTrans::write_read(DEV_ADDR, vec![Register::STATUS], vec![0]),
    ];
    let mut sensor = new(&transactions, nwake);
    sensor.set_environment(48.25, 25.125).unwrap();
    destroy(sensor);
}
