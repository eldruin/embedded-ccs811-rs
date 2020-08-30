use embedded_ccs811::{prelude::*, Ccs811Awake, MeasurementMode, ModeChangeError, SlaveAddr};
use linux_embedded_hal::I2cdev;
use nb::block;

fn main() {
    let dev = I2cdev::new("/dev/i2c-1").unwrap();
    let address = SlaveAddr::default();
    let sensor = Ccs811Awake::new(dev, address);
    match sensor.start_application() {
        Err(ModeChangeError { dev: _, error }) => {
            println!("Error during application start: {:?}", error);
        }
        Ok(mut sensor) => {
            sensor.set_mode(MeasurementMode::ConstantPower1s).unwrap();
            loop {
                let data = block!(sensor.data()).unwrap();
                println!("eCO2: {}, eTVOC: {}", data.eco2, data.etvoc);
            }
        }
    }
}
