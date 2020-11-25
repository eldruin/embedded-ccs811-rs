# Rust CCS811 Driver: Ultra-low Power Digital Gas Sensor for Monitoring Indoor Air Quality

[![crates.io](https://img.shields.io/crates/v/embedded-ccs811.svg)](https://crates.io/crates/embedded-ccs811)
[![Docs](https://docs.rs/embedded-ccs811/badge.svg)](https://docs.rs/embedded-ccs811)
[![Build Status](https://github.com/eldruin/embedded-ccs811-rs/workflows/Build/badge.svg)](https://github.com/eldruin/embedded-ccs811-rs/actions?query=workflow%3ABuild)
[![Coverage Status](https://coveralls.io/repos/github/eldruin/embedded-ccs811-rs/badge.svg?branch=master)](https://coveralls.io/github/eldruin/embedded-ccs811-rs?branch=master)

This is a platform agnostic Rust driver for the CCS811 ultra-low power
digital VOC sensor for monitoring indoor air quality (IAQ) using
the [`embedded-hal`] traits.

This driver allows you to:
- In application mode:
    - Set the measurement mode. See: `set_mode()`.
    - Check if there is new data ready. See: `has_data_ready()`.
    - Get the algoritm and raw result data. See: `data()`.
    - Get the raw data. See: `raw_data()`.
    - Get the current baseline. See: `baseline()`.
    - Set the baseline. See: `set_baseline()`.
    - Set the environment temperature and relative humidity. See: `set_environment()`.
    - Set the interrupt mode. See: `set_interrupt_mode()`.
    - Set the eCO2 thresholds for interrupts. See: `set_eco2_thresholds()`.
- In boot mode:
    - Start application. See: `start_application()`.
    - Reset, erase, download and verify new application. See: `update_application()`.
    - Erase application. See: `erase_application()`.
    - Verify application. See: `verify_application()`.
    - Download application. See: `download_application()`.
- In either mode:
    - Get the firmware mode. See: `firmware_mode()`.
    - Check whether a valid application is loaded. See: `has_valid_app()`.
    - Get the hardware ID. See: `hardware_id()`.
    - Get the hardware version. See: `hardware_version()`.
    - Get the firmware bootloader version. See: `firmware_bootloader_version()`.
    - Get the firmware application version. See: `firmware_application_version()`.
    - Do a software reset. See: `software_reset()`.

[Introductory blog post](https://blog.eldruin.com/ccs811-indoor-air-quality-sensor-driver-in-rust)

## The device

The CCS811 is an ultra-low power digital gas sensor solution which
integrates a metal oxide (MOX) gas sensor to detect a wide range of
Volatile Organic Compounds (VOCs) for indoor air quality monitoring
with a microcontroller unit (MCU), which includes an Analog-to-Digital
converter (ADC), and an I²C interface.

CCS811 is based on ams unique micro-hotplate technology which enables a
highly reliable solution for gas sensors, very fast cycle times and a 
significant reduction in average power consumption.

The integrated MCU manages the sensor driver modes and measurements.
The I²C digital interface significantly simplifies the hardware and
software design, enabling a faster time to market.

CCS811 supports intelligent algorithms to process raw sensor measurements
to output equivalent total VOC (eTVOC) and equivalent CO2 (eCO2) values,
where the main cause of VOCs is from humans.

CCS811 supports multiple measurement modes that have been optimized for
low-power consumption during an active sensor measurement and idle mode
extending battery life in portable applications.

Documentation:
- [Datasheet](https://www.sciosense.com/wp-content/uploads/2020/01/CCS811-Datasheet.pdf)
- [Programming and interfacing guide](https://www.sciosense.com/wp-content/uploads/2020/01/CCS811-Application-Note-Programming-and-interfacing-guide.pdf)

## Usage

To use this driver, import this crate and an `embedded_hal` implementation,
then instantiate the device.

Please find additional examples using hardware in this repository: [driver-examples]

[driver-examples]: https://github.com/eldruin/driver-examples

```rust
extern crate linux_embedded_hal as hal;
use embedded_ccs811::{prelude::*, Ccs811, MeasurementMode, SlaveAddr};
use nb::block;

fn main() {
    let dev = hal::I2cdev::new("/dev/i2c-1").unwrap();
    let nwake = hal::Pin::new(17);
    let delay = hal::Delay {};
    let address = SlaveAddr::default();
    let sensor = Ccs811::new(dev, address, nwake, delay);
    let mut sensor = sensor.start_application().ok().unwrap();
    sensor.set_mode(MeasurementMode::ConstantPower1s).unwrap();
    loop {
        let data = block!(sensor.data()).unwrap();
        println!("eCO2: {}, eTVOC: {}", data.eco2, data.etvoc);
    }
}
```

## Support

For questions, issues, feature requests, and other changes, please file an
[issue in the github project](https://github.com/eldruin/embedded-ccs811-rs/issues).

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

### Contributing

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.

[`embedded-hal`]: https://github.com/rust-embedded/embedded-hal
