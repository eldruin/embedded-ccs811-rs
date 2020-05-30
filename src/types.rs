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

/// Errors reported by the device.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct DeviceErrors {
    /// I2C write to an invalid register reported by device.
    pub invalid_register_write: bool,
    /// I2C read from an invalid register reported by device.
    pub invalid_register_read: bool,
    /// Invalid measurement reported by device.
    pub invalid_measurement: bool,
    /// Sensor resistance measurement reached or exceeded the maximum range reported by device.
    pub max_resistance: bool,
    /// Heater current not in range reported by device.
    pub heater_fault: bool,
    /// Heater current not applied correctly reported by device.
    pub heater_supply: bool,
}

/// Error type for mode changes when using `Ccs811`.
///
/// This allows to retrieve the unchanged device in case of an error.
#[derive(Debug)]
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

/// Interrupt generation modes.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InterruptMode {
    /// Disable interrupt generation
    Disabled,
    /// Generate an interrupt every time there is new data ready.
    OnDataReady,
    /// Generate an interrupt if the measurement crosses a threshold by more
    /// than 50 ppm. (See `set_eco2_thresholds()`).
    OnThresholdCrossed,
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
}
