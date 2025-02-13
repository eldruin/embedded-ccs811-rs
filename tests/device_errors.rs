use embedded_ccs811::{prelude::*, Error};
use embedded_hal_mock::eh1::{
    digital::{Mock as PinMock, State as PinState, Transaction as PinTrans},
    i2c::Transaction as I2cTrans,
};
mod common;
use crate::common::{destroy, new, BitFlags as BF, Register, DEV_ADDR};

macro_rules! expect_err {
    ($name:ident, $error_id:expr, $invalid_write:expr, $invalid_read:expr, $invalid_meas:expr,
    $max_resistance:expr, $heater_fault:expr, $heater_supply:expr) => {
        #[test]
        fn $name() {
            let nwake =
                PinMock::new(&[PinTrans::set(PinState::Low), PinTrans::set(PinState::High)]);
            let transactions = [
                I2cTrans::write_read(DEV_ADDR, vec![Register::HW_ID], vec![0x81]),
                I2cTrans::write_read(DEV_ADDR, vec![Register::STATUS], vec![BF::ERROR]),
                I2cTrans::write_read(DEV_ADDR, vec![Register::ERROR_ID], vec![$error_id]),
            ];
            let mut sensor = new(&transactions, nwake);
            match sensor.hardware_id() {
                Err(Error::Device(errors)) => {
                    assert_eq!($invalid_write, errors.invalid_register_write);
                    assert_eq!($invalid_read, errors.invalid_register_read);
                    assert_eq!($invalid_meas, errors.invalid_measurement);
                    assert_eq!($max_resistance, errors.max_resistance);
                    assert_eq!($heater_fault, errors.heater_fault);
                    assert_eq!($heater_supply, errors.heater_supply);
                }
                _ => panic!("Wrong result"),
            }
            destroy(sensor);
        }
    };
}

expect_err!(
    invalid_write,
    BF::WRITE_REG_INVALID,
    true,
    false,
    false,
    false,
    false,
    false
);

expect_err!(
    invalid_read,
    BF::READ_REG_INVALID,
    false,
    true,
    false,
    false,
    false,
    false
);

expect_err!(
    invalid_measurement,
    BF::MEASMODE_INVALID,
    false,
    false,
    true,
    false,
    false,
    false
);

expect_err!(
    max_resistence,
    BF::MAX_RESISTANCE,
    false,
    false,
    false,
    true,
    false,
    false
);

expect_err!(
    heater_fault,
    BF::HEATER_FAULT,
    false,
    false,
    false,
    false,
    true,
    false
);

expect_err!(
    heater_supply,
    BF::HEATER_SUPPLY,
    false,
    false,
    false,
    false,
    false,
    true
);

expect_err!(
    heater_supply_and_heater_fault,
    BF::HEATER_SUPPLY | BF::HEATER_FAULT,
    false,
    false,
    false,
    false,
    true,
    true
);

expect_err!(
    all,
    BF::WRITE_REG_INVALID
        | BF::READ_REG_INVALID
        | BF::MEASMODE_INVALID
        | BF::MAX_RESISTANCE
        | BF::HEATER_SUPPLY
        | BF::HEATER_FAULT,
    true,
    true,
    true,
    true,
    true,
    true
);
