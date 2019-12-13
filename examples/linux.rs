extern crate linux_embedded_hal as hal;
use ccs811::{prelude::*, Ccs811, SlaveAddr};
use nb::block;

fn main() {
    let dev = hal::I2cdev::new("/dev/i2c-1").unwrap();
    let nwake = hal::Pin::new(17);
    let address = SlaveAddr::default();
    let delay = hal::Delay {};
    let sensor = Ccs811::new(dev, address, nwake, delay);
    let mut sensor = sensor.start_application().ok().unwrap();
    loop {
        let data = block!(sensor.data()).unwrap();
        println!("eCO2: {}, eTVOC: {}", data.eco2, data.etvoc);
    }
}
