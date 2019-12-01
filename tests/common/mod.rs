use ccs811::{Ccs811, SlaveAddr};
use hal::i2c::{Mock as I2cMock, Transaction as I2cTrans};

pub const DEV_ADDR: u8 = 0x90;

pub fn new(transactions: &[I2cTrans]) -> Ccs811<I2cMock> {
    Ccs811::new(I2cMock::new(transactions), SlaveAddr::default())
}

pub fn destroy(sensor: Ccs811<I2cMock>) {
    sensor.destroy().done();
}
