use std::io;

/// Defines a serial port that the library can use
pub trait SerialPort: io::Read + io::Write {}

pub(crate) struct SynScanPort<T: SerialPort>(pub(crate) T);

impl<T: SerialPort> io::Write for SynScanPort<T> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.0.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.0.flush()
    }
}

impl<T: SerialPort> io::Read for SynScanPort<T> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.0.read(buf)
    }
}
