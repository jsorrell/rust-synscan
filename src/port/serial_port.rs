use std::io;
use std::sync::Mutex;

/// Defines a serial port that the library can use
pub trait SerialPort: io::Read + io::Write {}

pub(crate) struct SynScanPort<T: SerialPort>(pub(crate) Mutex<T>);
