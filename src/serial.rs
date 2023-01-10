use crate::enums::{JrkG2Command, VarOffset};
use crate::jrk::JrkG2;
use embedded_hal::serial;
use nb::block;

/// Implement the `JrkG2` trait for Serial
pub struct Serial<Bus> {
    serial: Bus,
}

impl<Bus> Serial<Bus>
where
    Bus: serial::Write<u8> + serial::Read<u8>,
{
    pub const fn new(serial: Bus) -> Self {
        Self { serial }
    }
}

impl<Bus> JrkG2<<Bus as serial::Read<u8>>::Error> for Serial<Bus>
where
    Bus: serial::Read<u8> + serial::Write<u8>,
{
    fn write(&mut self, data: &[u8]) -> Result<(), <Bus as serial::Read<u8>>::Error> {
        for &b in data.iter() {
            block!(self.serial.write(b)).ok(); // Infallible
        }
        Ok(())
    }
    fn read(&mut self, cmd: VarOffset) -> Result<u16, <Bus as serial::Read<u8>>::Error> {
        self.write(&[JrkG2Command::GetVariable16 as u8 | (cmd as u8 + 1)])?;
        let l = block!(self.serial.read())?;
        let h = block!(self.serial.read())?;
        Ok(u16::from_le_bytes([l, h]))
    }
}
