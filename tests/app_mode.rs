use ccs811::{prelude::*, MeasurementMode};
use embedded_hal_mock::{
    i2c::Transaction as I2cTrans,
    pin::{Mock as PinMock, State as PinState, Transaction as PinTrans},
};
mod common;
use crate::common::{destroy, new, Register, DEV_ADDR};

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
