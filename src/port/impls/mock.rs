#![allow(unused)]

use crate::port::commands::*;
use crate::*;
use std::io;
use std::rc::Rc;
use std::sync::Mutex;

struct MockBufs {
    bytes_written: Vec<u8>,
    bytes_to_read: Vec<u8>,
}

/// A mock implementation of a SynScan port where the response is manually given and the written bytes can be read
#[derive(Clone)]
pub struct MockSynScanPort {
    bufs: Rc<Mutex<MockBufs>>,
}

impl MockSynScanPort {
    pub fn new() -> MockSynScanPort {
        MockSynScanPort {
            bufs: Rc::new(Mutex::new(MockBufs {
                bytes_written: Vec::with_capacity(40),
                bytes_to_read: Vec::with_capacity(40),
            })),
        }
    }

    pub fn add_response(&self, buf: &[u8]) {
        let mut bufs = self.bufs.lock().unwrap();
        bufs.bytes_to_read.extend_from_slice(buf)
    }

    pub fn add_ok(&self) {
        self.add_valid_response(&[])
    }

    pub fn add_valid_response(&self, buf: &[u8]) {
        let mut bufs = self.bufs.lock().unwrap();
        bufs.bytes_to_read.push(SUCCESS_BYTE);
        bufs.bytes_to_read.extend_from_slice(buf);
        bufs.bytes_to_read.push(TERMINATION_BYTE);
    }

    pub fn add_error_response(&self, error_byte: u8) {
        let mut bufs = self.bufs.lock().unwrap();
        bufs.bytes_to_read.push(ERROR_BYTE);
        bufs.bytes_to_read.push(error_byte);
        bufs.bytes_to_read.push(TERMINATION_BYTE);
    }

    // pub fn get_written_bytes(&mut self) -> &Vec<u8> {
    //     let bufs = self.bufs.lock().unwrap();
    //     &self.bytes_written
    // }

    pub fn check_correct(&self, command: u8, channel: impl Channel) {
        self.check_correct_query_written(command, channel, &[])
    }

    pub fn check_correct_query_written(&self, command: u8, channel: impl Channel, buf: &[u8]) {
        let mut bufs = self.bufs.lock().unwrap();
        let mut correct = Vec::with_capacity(buf.len() + 3);
        correct.push(QUERY_BYTE);
        correct.push(command);
        correct.push(channel.get_byte());
        correct.extend_from_slice(buf);
        correct.push(TERMINATION_BYTE);

        assert_eq!(&bufs.bytes_written, &correct);
        bufs.bytes_written.clear();
    }
}

impl io::Read for MockSynScanPort {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let mut bufs = self.bufs.lock().unwrap();

        if bufs.bytes_to_read.is_empty() {
            // Conveniently (for tester) give ack when empty
            bufs.bytes_to_read.push(SUCCESS_BYTE);
            bufs.bytes_to_read.push(TERMINATION_BYTE);
        }

        for i in 0..buf.len() {
            if bufs.bytes_to_read.is_empty() {
                return Ok(i);
            } else {
                buf[i] = bufs.bytes_to_read.remove(0);
            }
        }
        Ok(buf.len())
    }
}

impl io::Write for MockSynScanPort {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let mut bufs = self.bufs.lock().unwrap();
        for b in buf.iter() {
            bufs.bytes_written.push(*b);
        }

        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

impl Default for MockSynScanPort {
    fn default() -> Self {
        MockSynScanPort::new()
    }
}

impl SerialPort for MockSynScanPort {}
