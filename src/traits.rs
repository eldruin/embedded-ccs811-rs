use private;

/// General CCS811 methods
pub trait Ccs811Device: private::Sealed {
    /// Error type
    type Error;

    /// Check if a valid application firmware is loaded.
    fn has_valid_app(&mut self) -> Result<bool, Self::Error>;

    /// Get the hardware ID (0x81 for the CCS81x family of devices)
    fn hardware_id(&mut self) -> Result<u8, Self::Error>;

    /// Get the hardware version (major, minor) ((1,X) for the CCS81x family of devices)
    fn hardware_version(&mut self) -> Result<(u8, u8), Self::Error>;

    /// Get the firmware bootloader verion (major, minor, trivial)
    fn firmware_bootloader_version(&mut self) -> Result<(u8, u8, u8), Self::Error>;
}

/// Methods available when on application mode
pub trait Ccs811AppMode: private::Sealed {}

/// Methods available when on boot mode
pub trait Ccs811BootMode: private::Sealed {
    /// Boot/App mode change error
    type ModeChangeError;
    /// Application mode type
    type AppModeType;

    /// Start App mode
    fn app_start(self) -> Result<Self::AppModeType, Self::ModeChangeError>;
}
