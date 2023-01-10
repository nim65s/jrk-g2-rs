use crate::enums::{JrkG2Command, VarOffset};
use crate::jrk::JrkG2;
use embedded_hal::blocking::i2c;

/// Implement the `JrkG2` trait for I2C
pub struct I2c<Bus> {
    device: u8,
    i2c: Bus,
}

impl<Bus, I2cError> I2c<Bus>
where
    Bus: i2c::Write<Error = I2cError> + i2c::Read<Error = I2cError>,
{
    pub const fn new(i2c: Bus) -> Self {
        Self { device: 0x0B, i2c }
    }
    /// The controller have a default 0x0B I2C address, but this can be manually changed in the
    /// configuration utility.
    pub fn set_device(&mut self, device: u8) {
        self.device = device;
    }
}

impl<Bus, I2cError> JrkG2<I2cError> for I2c<Bus>
where
    Bus: i2c::Write<Error = I2cError> + i2c::Read<Error = I2cError>,
{
    fn write(&mut self, data: &[u8]) -> Result<(), I2cError> {
        self.i2c.write(self.device, data)
    }
    fn read(&mut self, cmd: VarOffset) -> Result<u16, I2cError> {
        let mut buf: [u8; 2] = [0, 0];
        self.write(&[JrkG2Command::GetVariable16 as u8 | (cmd as u8 + 1)])?;
        self.i2c.read(self.device, &mut buf)?;
        Ok(u16::from_le_bytes(buf))
    }
}
