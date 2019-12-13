//! This is a platform agnostic Rust driver for the CCS811 ultra-low power
//! digital VOC sensor for monitoring indoor air quality (IAQ) using
//! the [`embedded-hal`] traits.
//!
//! [`embedded-hal`]: https://github.com/rust-embedded/embedded-hal
//!
//! This driver allows you to:
//! - In application mode:
//!     - Set the measurement mode. See: [`set_mode()`].
//!     - Check if there is new data ready. See: [`has_data_ready()`].
//!     - Get the algoritm and raw result data. See: [`data()`].
//!     - Get the raw data. See: [`raw_data()`].
//!     - Get the current baseline. See: [`baseline()`].
//!     - Set the baseline. See: [`set_baseline()`].
//!     - Set the environment temperature and relative humidity. See: [`set_environment()`].
//!     - Set the interrupt mode. See: [`set_interrupt_mode()`].
//!     - Set the eCO2 thresholds for interrupts. See: [`set_eco2_thresholds()`].
//! - In boot mode:
//!     - Start application. See: [`start_application()`].
//!     - Reset, erase, download and verify new application. See: [`update_application()`].
//!     - Erase application. See: [`erase_application()`].
//!     - Verify application. See: [`verify_application()`].
//!     - Download application. See: [`download_application()`].
//! - In either mode:
//!     - Get the firmware mode. See: [`firmware_mode()`].
//!     - Check whether a valid application is loaded. See: [`has_valid_app()`].
//!     - Get the hardware ID. See: [`hardware_id()`].
//!     - Get the hardware version. See: [`hardware_version()`].
//!     - Get the firmware bootloader version. See: [`firmware_bootloader_version()`].
//!     - Get the firmware application version. See: [`firmware_application_version()`].
//!     - Do a software reset. See: [`software_reset()`].
//!
//! [`set_mode()`]: trait.Ccs811AppMode.html#tymethod.set_mode
//! [`has_data_ready()`]: trait.Ccs811AppMode.html#tymethod.has_data_ready
//! [`data()`]: trait.Ccs811AppMode.html#tymethod.data
//! [`raw_data()`]: trait.Ccs811AppMode.html#tymethod.raw_data
//! [`baseline()`]: trait.Ccs811AppMode.html#tymethod.baseline
//! [`set_baseline()`]: trait.Ccs811AppMode.html#tymethod.set_baseline
//! [`set_environment()`]: trait.Ccs811AppMode.html#tymethod.set_environment
//! [`set_interrupt_mode()`]: trait.Ccs811AppMode.html#tymethod.set_interrupt_mode
//! [`set_eco2_thresholds()`]: trait.Ccs811AppMode.html#tymethod.set_eco2_thresholds
//! [`start_application()`]: trait.Ccs811BootMode.html#tymethod.start_application
//! [`update_application()`]: trait.Ccs811BootMode.html#tymethod.update_application
//! [`erase_application()`]: trait.Ccs811BootMode.html#tymethod.erase_application
//! [`verify_application()`]: trait.Ccs811BootMode.html#tymethod.verify_application
//! [`download_application()`]: trait.Ccs811BootMode.html#tymethod.download_application
//! [`firmware_mode()`]: trait.Ccs811Device.html#tymethod.firmware_mode
//! [`has_valid_app()`]: trait.Ccs811Device.html#tymethod.has_valid_app
//! [`hardware_id()`]: trait.Ccs811Device.html#tymethod.hardware_id
//! [`hardware_version()`]: trait.Ccs811Device.html#tymethod.hardware_version
//! [`firmware_bootloader_version()`]: trait.Ccs811Device.html#tymethod.firmware_bootloader_version
//! [`firmware_application_version()`]: trait.Ccs811Device.html#tymethod.firmware_application_version
//! [`software_reset()`]: trait.Ccs811Device.html#tymethod.software_reset
//!
//! <!-- TODO
//! [Introductory blog post](TODO)
//! -->
//!
//! ## The device
//!
//! The CCS811 is an ultra-low power digital gas sensor solution which
//! integrates a metal oxide (MOX) gas sensor to detect a wide range of
//! Volatile Organic Compounds (VOCs) for indoor air quality monitoring
//! with a microcontroller unit (MCU), which includes an Analog-to-Digital
//! converter (ADC), and an I²C interface.
//!
//! CCS811 is based on ams unique micro-hotplate technology which enables a
//! highly reliable solution for gas sensors, very fast cycle times and a
//! significant reduction in average power consumption.
//!
//! The integrated MCU manages the sensor driver modes and measurements.
//! The I²C digital interface significantly simplifies the hardware and
//! software design, enabling a faster time to market.
//!
//! CCS811 supports intelligent algorithms to process raw sensor measurements
//! to output equivalent total VOC (eTVOC) and equivalent CO2 (eCO2) values,
//! where the main cause of VOCs is from humans.
//!
//! CCS811 supports multiple measurement modes that have been optimized for
//! low-power consumption during an active sensor measurement and idle mode
//! extending battery life in portable applications.
//!
//! Documentation:
//! - [Datasheet](https://ams.com/documents/20143/36005/CCS811_DS000459_7-00.pdf)
//! - [Programming and interfacing guide](https://ams.com/documents/20143/36005/CCS811_AN000369_2-00.pdf)
//!
//! ## Usage examples (see also examples folder)
//!
//! To use this driver, import this crate and an `embedded_hal` implementation,
//! then instantiate the appropriate device.
//!
//! The CCS811 can be placed in sleep and woken up only for communication.
//! This driver provides two structures: `Ccs811Awake` and `Ccs811` depeding
//! on the waking state.
//!
//! The `Ccs811Awake` assumes an awake device and handles only the I2C communication.
//! This can be used when the waking up and sleep of the device is handled
//! manually.
//! Additionally a wrapper `Ccs811` is provided, which handles waking up
//! the device before each operation and putting it to sleep afterwards.
//!
//! Please find additional examples using hardware in this repository: [driver-examples]
//!
//! [driver-examples]: https://github.com/eldruin/driver-examples
//!
//! ### Start the application and take measurements
//!
//! ```no_run
//! extern crate linux_embedded_hal as hal;
//! use ccs811::{prelude::*, Ccs811, SlaveAddr, MeasurementMode};
//! use nb::block;
//!
//! # fn main() {
//! let dev = hal::I2cdev::new("/dev/i2c-1").unwrap();
//! let nwake = hal::Pin::new(17);
//! let delay = hal::Delay {};
//! let address = SlaveAddr::default();
//! let sensor = Ccs811::new(dev, address, nwake, delay);
//! let mut sensor = sensor.start_application().ok().unwrap();
//! sensor.set_mode(MeasurementMode::ConstantPower1s).unwrap();
//! loop {
//!     let data = block!(sensor.data()).unwrap();
//!     println!("eCO2: {}, eTVOC: {}", data.eco2, data.etvoc);
//! }
//! # }
//! ```
//!
//! ### Save and restore the baseline
//!
//! ```no_run
//! extern crate linux_embedded_hal as hal;
//! use ccs811::{prelude::*, Ccs811Awake, SlaveAddr};
//!
//! # fn main() {
//! let dev = hal::I2cdev::new("/dev/i2c-1").unwrap();
//! let address = SlaveAddr::default();
//! let sensor = Ccs811Awake::new(dev, address);
//! let mut sensor = sensor.start_application().ok().unwrap();
//! let baseline = sensor.baseline().unwrap();
//! // ...
//! sensor.set_baseline(baseline).unwrap();
//! # }
//! ```
//!
//! ### Set the environment temperature and relative humidity
//!
//! ```no_run
//! extern crate linux_embedded_hal as hal;
//! use ccs811::{prelude::*, Ccs811Awake, SlaveAddr};
//!
//! # fn main() {
//! let dev = hal::I2cdev::new("/dev/i2c-1").unwrap();
//! let address = SlaveAddr::default();
//! let sensor = Ccs811Awake::new(dev, address);
//! let mut sensor = sensor.start_application().ok().unwrap();
//! let temp_c = 25;
//! let rel_humidity = 50.0;
//! sensor.set_environment(rel_humidity, temp_c).unwrap();
//! # }
//! ```
//!
//! ### Set the eCO2 thresholds and configure interrupts
//!
//! Only generate an interrupt when the thresholds are crossed.
//!
//! ```no_run
//! extern crate linux_embedded_hal as hal;
//! use ccs811::{prelude::*, Ccs811Awake, SlaveAddr, InterruptMode, MeasurementMode};
//!
//! # fn main() {
//! let dev = hal::I2cdev::new("/dev/i2c-1").unwrap();
//! let address = SlaveAddr::default();
//! let sensor = Ccs811Awake::new(dev, address);
//! let mut sensor = sensor.start_application().ok().unwrap();
//! sensor.set_eco2_thresholds(1500, 2500).unwrap();
//! sensor.set_interrupt_mode(InterruptMode::OnThresholdCrossed).unwrap();
//! sensor.set_mode(MeasurementMode::ConstantPower1s).unwrap();
//! # }
//! ```
//!
//! ### Get hardware and firmware information
//!
//! ```no_run
//! extern crate linux_embedded_hal as hal;
//! use ccs811::{prelude::*, Ccs811Awake, SlaveAddr};
//!
//! # fn main() {
//! let dev = hal::I2cdev::new("/dev/i2c-1").unwrap();
//! let address = SlaveAddr::default();
//! let mut sensor = Ccs811Awake::new(dev, address);
//! let hw_id = sensor.hardware_id().unwrap();
//! let hw_ver = sensor.hardware_version().unwrap();
//! let fw_boot_ver = sensor.firmware_bootloader_version().unwrap();
//! let fw_app_ver = sensor.firmware_application_version().unwrap();
//! println!(
//!     "HW ID: {}, HW version: {:#?}, FW bootloader version: {:#?}, FW app version: {:#?}",
//!     hw_id, hw_ver, fw_boot_ver, fw_app_ver
//! );
//! # }
//! ```

#![deny(unsafe_code, missing_docs)]
#![no_std]

extern crate embedded_hal as hal;
use core::marker::PhantomData;

mod common_impl;
pub mod prelude;
mod register_access;
use crate::register_access::{BitFlags, Register};
mod app_mode;
mod boot_mode;
mod traits;
pub use crate::traits::{Ccs811AppMode, Ccs811BootMode, Ccs811Device};
mod types;
pub use crate::types::{
    AlgorithmResult, DeviceError, DeviceErrors, Error, ErrorAwake, FirmwareMode, InterruptMode,
    MeasurementMode, ModeChangeError, SlaveAddr,
};

/// CCS811 device driver
///
/// Convenience wrapper arount `Ccs811Awake` which handles waking up the device on each operation.
#[derive(Debug)]
pub struct Ccs811<I2C, NWAKE, WAKEDELAY, MODE> {
    dev: Ccs811Awake<I2C, MODE>,
    n_wake_pin: NWAKE,
    wake_delay: WAKEDELAY,
    _mode: PhantomData<MODE>,
}

/// Already awake CCS811 device driver
///
/// This can be used when the nWAKE pin is connected directly to GND or when
/// handling the device waking manually instead of using the `Ccs811` wrapper type.
#[derive(Debug)]
pub struct Ccs811Awake<I2C, MODE> {
    /// The concrete I²C device implementation.
    i2c: I2C,
    address: u8,
    meas_mode_reg: u8,
    in_progress: ActionInProgress,
    _mode: PhantomData<MODE>,
}

#[derive(Debug, PartialEq)]
enum ActionInProgress {
    None,
    Verification,
    Erase,
}

/// Mode marker
pub mod mode {
    /// Boot mode
    pub struct Boot(());
    /// App mode
    pub struct App(());
}

mod private {
    use super::{mode, Ccs811, Ccs811Awake};
    pub trait Sealed {}

    impl Sealed for mode::Boot {}
    impl Sealed for mode::App {}
    impl<I2C, NWAKE, WAKEDELAY, MODE> Sealed for Ccs811<I2C, NWAKE, WAKEDELAY, MODE> {}
    impl<I2C, MODE> Sealed for Ccs811Awake<I2C, MODE> {}
}
