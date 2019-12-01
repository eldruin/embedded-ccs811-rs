# Rust CCS811 Driver: Ultra-low Power Digital Gas Sensor for Monitoring Indoor Air Quality

<!-- TODO
[![crates.io](https://img.shields.io/crates/v/ccs811.svg)](https://crates.io/crates/ccs811)
[![Docs](https://docs.rs/ccs811/badge.svg)](https://docs.rs/ccs811)
-->
[![Build Status](https://travis-ci.org/eldruin/ccs811-rs.svg?branch=master)](https://travis-ci.org/eldruin/ccs811-rs)
[![Coverage Status](https://coveralls.io/repos/github/eldruin/ccs811-rs/badge.svg?branch=master)](https://coveralls.io/github/eldruin/ccs811-rs?branch=master)

This is a platform agnostic Rust driver for the CCS811 ultra-low power digital
gas sensor for monitoring indoor air quality using the [`embedded-hal`] traits.

<!--TODO
This driver allows you to:
-->
<!-- TODO
[Introductory blog post]()
-->

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
- [Datasheet](https://ams.com/documents/20143/36005/CCS811_DS000459_7-00.pdf)
- [Programming and interfacing guide](https://ams.com/documents/20143/36005/CCS811_AN000369_2-00.pdf)

<!--TODO
## Usage

To use this driver, import this crate and an `embedded_hal` implementation,
then instantiate the device.

Please find additional examples using hardware in this repository: [driver-examples]

[driver-examples]: https://github.com/eldruin/driver-examples

```rust
```
-->

## Support

For questions, issues, feature requests, and other changes, please file an
[issue in the github project](https://github.com/eldruin/ccs811-rs/issues).

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
