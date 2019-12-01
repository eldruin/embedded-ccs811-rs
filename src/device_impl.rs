use {Ccs811, SlaveAddr};

impl<I2C> Ccs811<I2C> {
    /// Create new instance of the CCS811 device.
    pub fn new(i2c: I2C, address: SlaveAddr) -> Self {
        Ccs811 {
            i2c,
            address: address.addr(),
        }
    }

    /// Destroy driver instance, return I²C bus instance.
    pub fn destroy(self) -> I2C {
        self.i2c
    }
}
