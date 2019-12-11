//! This is a platform agnostic Rust driver for the CCS811 high-accuracy
//! ambient light sensor using the [`embedded-hal`] traits.
//!
//! [`embedded-hal`]: https://github.com/rust-embedded/embedded-hal
//!
//! <!--TODO
//! This driver allows you to:
//! -->
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

#![deny(unsafe_code, missing_docs)]
#![no_std]

extern crate embedded_hal as hal;
use core::marker::PhantomData;

mod common_impl;
pub mod prelude;
mod register_access;
use crate::register_access::{BitFlags, Register};
mod boot_mode;
mod traits;
pub use crate::traits::{Ccs811AppMode, Ccs811BootMode, Ccs811Device};
mod types;
pub use crate::types::{DeviceError, DeviceErrors, Error, ErrorAwake, ModeChangeError, SlaveAddr};

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
