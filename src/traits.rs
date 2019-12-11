use crate::{private, MeasurementMode};
use nb;

/// General CCS811 methods
pub trait Ccs811Device: private::Sealed {
    /// Error type
    type Error;
    /// Boot/App mode change error
    type ModeChangeError;
    /// Boot mode type
    type BootModeType;

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
}
