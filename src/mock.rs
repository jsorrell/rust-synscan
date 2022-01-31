use serialport::{ClearBuffer, DataBits, FlowControl, Parity, SerialPort, StopBits};
use std::io::{Read, Write};
use std::time::Duration;

pub struct MockSynScanPort {
    bytes_written: Vec<u8>,
    bytes_to_read: Vec<u8>,
}

impl MockSynScanPort {
    pub fn new() -> MockSynScanPort {
        MockSynScanPort {
            bytes_written: Vec::with_capacity(40),
            bytes_to_read: Vec::with_capacity(40),
        }
    }

    pub fn add_response(&mut self, buf: &[u8]) {
        self.bytes_to_read.extend_from_slice(buf)
    }

    pub fn add_valid_response(&mut self, buf: &[u8]) {
        self.bytes_to_read.push(b'=');
        self.add_response(buf);
        self.bytes_to_read.push(b'\r');
    }

    pub fn add_error_response(&mut self, error_byte: u8) {
        self.bytes_to_read.push(b'!');
        self.bytes_to_read.push(error_byte);
        self.bytes_to_read.push(b'\r');
    }

    pub fn get_written_bytes(&mut self) -> &Vec<u8> {
        &self.bytes_written
    }
}

impl Read for &mut MockSynScanPort {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        for i in 0..buf.len() {
            if self.bytes_to_read.is_empty() {
                return Ok(i);
            } else {
                buf[i] = self.bytes_to_read.remove(0);
            }
        }
        Ok(buf.len())
    }
}

impl Write for &mut MockSynScanPort {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        for b in buf.into_iter() {
            self.bytes_written.push(*b);
        }

        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

impl SerialPort for &mut MockSynScanPort {
    fn name(&self) -> Option<String> {
        unimplemented!()
    }

    fn baud_rate(&self) -> serialport::Result<u32> {
        unimplemented!()
    }

    fn data_bits(&self) -> serialport::Result<DataBits> {
        unimplemented!()
    }

    fn flow_control(&self) -> serialport::Result<FlowControl> {
        unimplemented!()
    }

    fn parity(&self) -> serialport::Result<Parity> {
        unimplemented!()
    }

    fn stop_bits(&self) -> serialport::Result<StopBits> {
        unimplemented!()
    }

    fn timeout(&self) -> Duration {
        unimplemented!()
    }

    fn set_baud_rate(&mut self, _baud_rate: u32) -> serialport::Result<()> {
        unimplemented!()
    }

    fn set_data_bits(&mut self, _data_bits: DataBits) -> serialport::Result<()> {
        unimplemented!()
    }

    fn set_flow_control(&mut self, _flow_control: FlowControl) -> serialport::Result<()> {
        unimplemented!()
    }

    fn set_parity(&mut self, _parity: Parity) -> serialport::Result<()> {
        unimplemented!()
    }

    fn set_stop_bits(&mut self, _stop_bits: StopBits) -> serialport::Result<()> {
        unimplemented!()
    }

    fn set_timeout(&mut self, _timeout: Duration) -> serialport::Result<()> {
        unimplemented!()
    }

    fn write_request_to_send(&mut self, _level: bool) -> serialport::Result<()> {
        unimplemented!()
    }

    fn write_data_terminal_ready(&mut self, _level: bool) -> serialport::Result<()> {
        unimplemented!()
    }

    fn read_clear_to_send(&mut self) -> serialport::Result<bool> {
        unimplemented!()
    }

    fn read_data_set_ready(&mut self) -> serialport::Result<bool> {
        unimplemented!()
    }

    fn read_ring_indicator(&mut self) -> serialport::Result<bool> {
        unimplemented!()
    }

    fn read_carrier_detect(&mut self) -> serialport::Result<bool> {
        unimplemented!()
    }

    fn bytes_to_read(&self) -> serialport::Result<u32> {
        unimplemented!()
    }

    fn bytes_to_write(&self) -> serialport::Result<u32> {
        unimplemented!()
    }

    fn clear(&self, _buffer_to_clear: ClearBuffer) -> serialport::Result<()> {
        unimplemented!()
    }

    fn try_clone(&self) -> serialport::Result<Box<dyn SerialPort>> {
        unimplemented!()
    }

    fn set_break(&self) -> serialport::Result<()> {
        unimplemented!()
    }

    fn clear_break(&self) -> serialport::Result<()> {
        unimplemented!()
    }
}
