use crate::{private, AlgorithmResult, FirmwareMode, InterruptMode, MeasurementMode};
use embedded_hal::blocking::delay::DelayMs;
use nb;

/// General CCS811 methods
pub trait Ccs811Device: private::Sealed {
    /// Error type
    type Error;
    /// Boot/App mode change error
    type ModeChangeError;
    /// Boot mode type
    type BootModeType;

    /// Get the firmware mode.
    fn firmware_mode(&mut self) -> Result<FirmwareMode, Self::Error>;

    /// Check if a valid application firmware is loaded.
    fn has_valid_app(&mut self) -> Result<bool, Self::Error>;

    /// Get the hardware ID (0x81 for the CCS81x family of devices)
    fn hardware_id(&mut self) -> Result<u8, Self::Error>;

    /// Get the hardware version (major, minor) ((1,X) for the CCS81x family of devices)
    fn hardware_version(&mut self) -> Result<(u8, u8), Self::Error>;

    /// Get the firmware bootloader verion (major, minor, trivial)
    fn firmware_bootloader_version(&mut self) -> Result<(u8, u8, u8), Self::Error>;

    /// Get the firmware application verion (major, minor, trivial)
    fn firmware_application_version(&mut self) -> Result<(u8, u8, u8), Self::Error>;

    /// Restart the device in boot mode.
    ///
    /// 2ms should be waited before doing any other operation.
    fn software_reset(self) -> Result<Self::BootModeType, Self::ModeChangeError>;
}

/// Methods available when on application mode
pub trait Ccs811AppMode: private::Sealed {
    /// Error type
    type Error;

    /// Set the measurement mode
    ///
    /// NOTE: When changing to a new mode with a lower sample rate,
    /// place the device in `Idle` mode for at least 10 minutes before
    /// enabling the new mode.
    fn set_mode(&mut self, mode: MeasurementMode) -> Result<(), Self::Error>;

    /// Check if there is a new data sample ready.
    fn has_data_ready(&mut self) -> Result<bool, Self::Error>;

    /// Get the raw sensor data.
    ///
    /// Returns a tuple containing the current and voltage through the sensor in
    /// the format: (current, voltage).
    /// The current is a value between 0uA and 63uA.
    /// The voltage contains the value as computed in the ADC. (1023 = 1.65V)
    fn raw_data(&mut self) -> Result<(u8, u16), Self::Error>;

    /// Get the algorithm results data.
    ///
    /// Returns a tuple containing the current and voltage through the sensor in
    /// the format: (current, voltage).
    /// The current is a value between 0uA and 63uA.
    /// The voltage contains the value as computed in the ADC. (1023 = 1.65V)
    fn data(&mut self) -> nb::Result<AlgorithmResult, Self::Error>;

    /// Set the environment temperature and relative humidity.
    ///
    /// The humidity must be provided as percentage: [0.0..100.0].
    /// The temperature must be provided in Celsius. (Theoretical max: 254.99805ºC)
    fn set_environment(
        &mut self,
        humidity_percentage: f32,
        temperature_celsius: f32,
    ) -> Result<(), Self::Error>;

    /// Set the eCO2 threshold values for interrupt generation (in ppm).
    ///
    /// An interrupt will be asserted if the value moved from the current
    /// range by 50 ppm.
    fn set_eco2_thresholds(
        &mut self,
        low_to_medium: u16,
        medium_to_high: u16,
    ) -> Result<(), Self::Error>;

    /// Configure the interrupt generation.
    fn set_interrupt_mode(&mut self, mode: InterruptMode) -> Result<(), Self::Error>;
}

/// Methods available when on boot mode
pub trait Ccs811BootMode: private::Sealed {
    /// Error type
    type Error;
    /// Boot/App mode change error
    type ModeChangeError;
    /// Application mode type
    type TargetType;

    /// Start application mode
    ///
    /// NOTE: after this call 1ms must be waited before sending application commands.
    fn start_application(self) -> Result<Self::TargetType, Self::ModeChangeError>;

    /// Verify application.
    ///
    /// NOTE: After the first call, 70ms must be waited before calling again to
    /// poll until completion.
    fn verify_application(&mut self) -> nb::Result<(), Self::Error>;

    /// Erase application.
    ///
    /// NOTE: After the first call, 500ms must be waited before calling again to
    /// poll until completion.
    fn erase_application(&mut self) -> nb::Result<(), Self::Error>;

    /// Download new application.
    ///
    /// Returns `Error::InvalidInputData` if the input binary lengh is not multiple of 8.
    /// This takes at least 50ms * (bin_size/8).
    fn download_application<D: DelayMs<u16>>(
        &mut self,
        bin: &[u8],
        delay: &mut D,
    ) -> Result<(), Self::Error>;

}
