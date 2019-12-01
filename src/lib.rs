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

mod device_impl;
mod types;
pub use types::{Error, SlaveAddr};

/// CCS811 device driver
#[derive(Debug)]
pub struct Ccs811<I2C> {
    /// The concrete I²C device implementation.
    i2c: I2C,
    address: u8,
}
