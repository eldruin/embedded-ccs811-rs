use core::convert::From;

/// All possible errors generated when using the `Ccs811` type.
#[derive(Debug)]
pub enum Error<CommE, PinE> {
    /// I²C bus error
    I2C(CommE),
    /// nWAKE pin set error
    Pin(PinE),
    /// Errors reported by device
    ///
    /// This can contain several errors at the same time.
    /// You can index this list by `DeviceError` to see if an specific error variant
    /// has been reported. See the documentation for usage examples.
    Device(DeviceErrors),
    /// No valid application loaded
    NoValidApp,
    /// Invalid input data provided to function
    InvalidInputData,
}

/// All possible errors when using an the `Ccs811Awake` type.
#[derive(Debug)]
pub enum ErrorAwake<E> {
    /// I²C bus error
    I2C(E),
    /// Errors reported by device
    ///
    /// This can contain several errors at the same time.
    /// You can index this list by `DeviceError` to see if an specific error variant
    /// has been reported. See the documentation for usage examples.
    Device(DeviceErrors),
    /// No valid application loaded
    NoValidApp,
    /// Invalid input data provided to function
    InvalidInputData,
}

impl<CommE, PinE> From<ErrorAwake<CommE>> for Error<CommE, PinE> {
    fn from(error: ErrorAwake<CommE>) -> Self {
        match error {
            ErrorAwake::I2C(e) => Error::I2C(e),
            ErrorAwake::Device(e) => Error::Device(e),
            ErrorAwake::NoValidApp => Error::NoValidApp,
            ErrorAwake::InvalidInputData => Error::InvalidInputData,
        }
    }
}

/// Array of possible errors since multiple sources are possible.
///
/// You can index by `DeviceError` to check for each error variant.
/// They are encoded as a bitmask.
#[derive(Debug)]
pub struct DeviceErrors(pub(crate) [bool; 6]);

use core::ops::{Index, IndexMut};

impl Index<DeviceError> for DeviceErrors {
    type Output = bool;

    fn index(&self, idx: DeviceError) -> &Self::Output {
        match idx {
            DeviceError::InvalidRegisterWrite => &self.0[0],
            DeviceError::InvalidRegisterRead => &self.0[1],
            DeviceError::InvalidMeasurement => &self.0[2],
            DeviceError::MaxResistence => &self.0[3],
            DeviceError::HeaterFault => &self.0[4],
            DeviceError::HeaterSupply => &self.0[5],
        }
    }
}

impl IndexMut<DeviceError> for DeviceErrors {
    fn index_mut(&mut self, idx: DeviceError) -> &mut Self::Output {
        match idx {
            DeviceError::InvalidRegisterWrite => &mut self.0[0],
            DeviceError::InvalidRegisterRead => &mut self.0[1],
            DeviceError::InvalidMeasurement => &mut self.0[2],
            DeviceError::MaxResistence => &mut self.0[3],
            DeviceError::HeaterFault => &mut self.0[4],
            DeviceError::HeaterSupply => &mut self.0[5],
        }
    }
}

/// Errors reported by the device
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DeviceError {
    /// I2C write to an invalid register reported by device.
    InvalidRegisterWrite,
    /// I2C read from an invalid register reported by device.
    InvalidRegisterRead,
    /// Invalid measurement reported by device.
    InvalidMeasurement,
    /// Sensor resistance measurement reached or exceeded the maximum range reported by device.
    MaxResistence,
    /// Heater current not in range reported by device.
    HeaterFault,
    /// Heater current not applied correctly reported by device.
    HeaterSupply,
}

/// Error type for mode changes when using `Ccs811`.
///
/// This allows to retrieve the unchanged device in case of an error.
pub struct ModeChangeError<E, DEV> {
    /// Unchanged device.
    pub dev: DEV,
    /// Error occurred.
    pub error: E,
}

impl<E, DEV> ModeChangeError<E, DEV> {
    pub(crate) fn new(dev: DEV, error: E) -> Self {
        ModeChangeError { dev, error }
    }
}

/// Measurement modes.
///
/// NOTE: When changing to a new mode with a lower sample rate,
/// place the device in `Idle` mode for at least 10 minutes before
/// enabling the new mode.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MeasurementMode {
    /// Idle. Measurements are disabled. (Mode 0)
    Idle,
    /// Constant power mode. IAQ measurement every second. (Mode 1)
    ConstantPower1s,
    /// Pulse heating mode. IAQ measurement every 10 seconds. (Mode 2)
    PulseHeating10s,
    /// Low power pulse heating mode. IAQ measurement every 60 seconds. (Mode 3)
    LowPowerPulseHeating60s,
    /// Constant power mode. IAQ measurement every 250ms. (Mode 4)
    ConstantPower250ms,
}

/// Firmware mode
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FirmwareMode {
    /// Boot mode. New firmware can be loaded.
    Boot,
    /// Application mode. CCS811 can take measurements
    Application,
}

/// Algorithm result
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AlgorithmResult {
    /// eCO2 result in ppm
    pub eco2: u16,
    /// eTVOC result in ppb
    pub etvoc: u16,
    /// Raw sensor current in uA
    pub raw_current: u8,
    /// Raw sensor voltage (1023 = 1.65V)
    pub raw_voltage: u16,
}

/// Possible slave addresses
#[derive(Debug, Clone, Copy)]
pub enum SlaveAddr {
    /// Default slave address
    Default,
    /// Alternative slave address providing bit value for the ADDR pin
    Alternative(bool),
}

impl Default for SlaveAddr {
    /// Default slave address
    fn default() -> Self {
        SlaveAddr::Default
    }
}

impl SlaveAddr {
    pub(crate) fn addr(self) -> u8 {
        match self {
            SlaveAddr::Default => 0x5A,
            SlaveAddr::Alternative(false) => 0x5A,
            SlaveAddr::Alternative(true) => 0x5B,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_get_default_address() {
        let addr = SlaveAddr::default();
        assert_eq!(0x5A, addr.addr());
    }

    #[test]
    fn can_generate_alternative_addresses() {
        assert_eq!(0x5A, SlaveAddr::Alternative(false).addr());
        assert_eq!(0x5B, SlaveAddr::Alternative(true).addr());
    }

    #[test]
    fn can_index_device_errors() {
        let errors = DeviceErrors([true; 6]);
        assert!(errors[DeviceError::InvalidRegisterWrite]);
        assert!(errors[DeviceError::InvalidRegisterRead]);
        assert!(errors[DeviceError::InvalidMeasurement]);
        assert!(errors[DeviceError::MaxResistence]);
        assert!(errors[DeviceError::HeaterFault]);
        assert!(errors[DeviceError::HeaterSupply]);
    }
}
