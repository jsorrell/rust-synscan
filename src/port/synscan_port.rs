use crate::port::commands::*;
use crate::port::SynScanPort;
use crate::util::*;
use crate::*;
use std::{io, slice};
use std::sync::Mutex;

/// Converts bytes returned from the mount into the number it describes
pub(crate) fn bytes_to_number(data: Vec<u8>) -> SynScanResult<u32> {
    let mut i = data.len() + 1;
    let mut n_vec = vec![0; data.len()];
    for b in data {
        i = match i % 2 {
            0 => i + 1,
            _ => i - 3,
        };
        n_vec[i] = b;
    }

    let hex_string: String = n_vec.into_iter().map(|b| b as char).collect();
    match u32::from_str_radix(&hex_string, 16) {
        Ok(v) => Ok(v),
        Err(e) => Err(SynScanError::CommunicationError(io::Error::new(
            io::ErrorKind::InvalidData,
            e,
        ))),
    }
}

/// Converts a number into a bytes the mount can understand.
pub(crate) fn number_to_bytes(number: u32, num_bytes: usize) -> Vec<u8> {
    let hex_string = format!("{:X}", number);
    let hex_bytes = hex_string.into_bytes();
    let mut bytes = vec![b'0'; 6];

    let mut i = 1;
    for b in hex_bytes.into_iter().rev() {
        bytes[i] = b;
        i = match i % 2 {
            0 => i + 3,
            _ => i - 1,
        };
        if i >= num_bytes {
            break;
        }
    }
    bytes.truncate(num_bytes);
    bytes
}

impl<T: SerialPort> SynScanPort<T> {
    pub(crate) fn new(port: T) -> Self
    where
        T: SerialPort,
    {
        SynScanPort(Mutex::new(port))
    }

    fn read_byte(port: &mut impl SerialPort) -> io::Result<u8> {
        let mut b: u8 = 0;
        if 0 == port.read(slice::from_mut(&mut b))? {
            return Err(io::Error::from(io::ErrorKind::TimedOut));
        }
        Ok(b)
    }

    pub fn test(&self) -> SynScanResult<()> {
        // TODO does initialization do anything?
        self.send_cmd(INITIALIZATION_DONE, MultiChannel::Both)
    }

    fn resolve_controller_error(code: u8) -> SynScanError {
        match code as char {
            '0' => SynScanError::UnknownCommand,
            '1' => SynScanError::CommandLengthError,
            '2' => SynScanError::MotorNotStopped,
            '3' => SynScanError::InvalidCharacter,
            '4' => SynScanError::NotInitialized,
            '5' => SynScanError::DriverSleeping,
            '7' => SynScanError::PECTrainingRunning,
            '8' => SynScanError::NoValidPECData,
            _ => SynScanError::CommunicationError(io::Error::from(io::ErrorKind::InvalidData)),
        }
    }

    fn consume_term_char(port: &mut impl SerialPort) -> io::Result<()> {
        let termination_char = Self::read_byte(port)?;
        if termination_char != TERMINATION_BYTE {
            return Err(io::Error::from(io::ErrorKind::InvalidData));
        }
        Ok(())
    }

    fn read_response(port: &mut impl SerialPort) -> SynScanResult<Vec<u8>> {
        let first_byte = Self::read_byte(port)?;

        if first_byte == SUCCESS_BYTE {
            // Successful
            const MAX_VALID_RESPONSE: usize = 6;
            let mut buf = Vec::with_capacity(MAX_VALID_RESPONSE);
            let mut i = 0;
            loop {
                let byte = Self::read_byte(port)?;
                if byte == TERMINATION_BYTE {
                    break;
                } else if i < MAX_VALID_RESPONSE {
                    buf.push(byte);
                    i += 1;
                } else {
                    return Err(SynScanError::CommunicationError(io::Error::from(
                        io::ErrorKind::InvalidData,
                    )));
                }
            }
            Ok(buf)
        } else if first_byte == ERROR_BYTE {
            // Error Code
            let error_code = Self::read_byte(port)?;
            Self::consume_term_char(port)?;
            Err(Self::resolve_controller_error(error_code))
        } else {
            Err(SynScanError::CommunicationError(io::Error::from(
                io::ErrorKind::InvalidData,
            )))
        }
    }

    fn raw_send_cmd(
        &self,
        cmd: u8,
        channel: impl Channel,
        bytes: &[u8],
    ) -> SynScanResult<Vec<u8>> {
        let mut full_cmd = vec![QUERY_BYTE, cmd, channel.get_byte()];
        full_cmd.extend(bytes);
        full_cmd.push(TERMINATION_BYTE);

        let mut port_lock = self.0.lock().unwrap();

        match port_lock.write_all(full_cmd.as_slice()) {
            Ok(_) => Self::read_response(&mut *port_lock),
            Err(e) => Err(SynScanError::CommunicationError(e)),
        }
    }

    /// Inquires the mount for bytes
    pub fn inquire_bytes(&self, cmd: u8, channel: impl Channel) -> SynScanResult<Vec<u8>> {
        let response = self.raw_send_cmd(cmd, channel, &[])?;
        if response.is_empty() {
            Err(SynScanError::CommunicationError(io::Error::from(
                io::ErrorKind::InvalidData,
            )))
        } else {
            Ok(response)
        }
    }

    /// Inquires the mount for a number
    pub fn inquire_number(&self, cmd: u8, channel: impl Channel) -> SynScanResult<u32> {
        bytes_to_number(self.inquire_bytes(cmd, channel)?)
    }

    /// Sends a responseless command to the mount with bytes as the payload
    pub fn send_cmd_bytes(
        &self,
        cmd: u8,
        channel: impl Channel,
        bytes: &[u8],
    ) -> SynScanResult<()> {
        self.raw_send_cmd(cmd, channel, bytes)?;
        Ok(())
    }

    /// Sends a responseless command to the mount with a number as the payload
    pub fn send_cmd_number(
        &self,
        cmd: u8,
        channel: impl Channel,
        number: u32,
        num_bytes: usize,
    ) -> SynScanResult<()> {
        self.send_cmd_bytes(cmd, channel, number_to_bytes(number, num_bytes).as_slice())
    }

    /// Sends a responseless command to the mount with no payload
    pub fn send_cmd(&self, cmd: u8, channel: impl Channel) -> SynScanResult<()> {
        self.send_cmd_bytes(cmd, channel, &[])
    }
}
