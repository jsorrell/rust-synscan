use crate::util::{SynScanError, SynScanResult};
use crate::{MotorController, SerialPort};
use std::io;
use std::time::Duration;

pub type SPSerialPort = Box<dyn serialport::SerialPort>;

#[cfg(feature = "serialport")]
impl SerialPort for SPSerialPort {}

#[cfg(feature = "serialport")]
impl MotorController<SPSerialPort> {
    /// Gets a new MotorController using the serialport implentation of a serial port
    pub fn new_serialport(
        path: impl Into<String>,
        baud_rate: u32,
        timeout: Duration,
    ) -> SynScanResult<MotorController<SPSerialPort>> {
        let port = serialport::new(path.into(), baud_rate)
            .timeout(timeout)
            .open()?;
        Self::new(port)
    }
}

#[cfg(feature = "serialport")]
impl From<serialport::Error> for SynScanError {
    fn from(e: serialport::Error) -> Self {
        SynScanError::from(io::Error::from(e))
    }
}
