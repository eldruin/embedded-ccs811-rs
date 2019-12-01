/// All possible errors in this crate
#[derive(Debug)]
pub enum Error<E> {
    /// I²C bus error
    I2C(E),
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
            SlaveAddr::Default => 0x90,
            SlaveAddr::Alternative(true) => 0x91,
            SlaveAddr::Alternative(false) => 0x90,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::SlaveAddr;

    #[test]
    fn can_get_default_address() {
        let addr = SlaveAddr::default();
        assert_eq!(0x90, addr.addr());
    }

    #[test]
    fn can_generate_alternative_addresses() {
        assert_eq!(0x90, SlaveAddr::Alternative(false).addr());
        assert_eq!(0x91, SlaveAddr::Alternative(true).addr());
    }
}
