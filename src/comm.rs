mod comm {
    use crate::result::SynScanResult;
    use crate::util::*;
    use crate::{MotorControllerPort, SynScanError};
    use std::io::ErrorKind::{InvalidData, TimedOut};
    use std::io::{ErrorKind, Read, Write};
    use std::{io, slice};

    impl SynScanError {
        fn get_controller_error(code: u8) -> SynScanError {
            return match code as char {
                '0' => SynScanError::UnknownCommand,
                '1' => SynScanError::CommandLengthError,
                '2' => SynScanError::MotorNotStopped,
                '3' => SynScanError::InvalidCharacter,
                '4' => SynScanError::NotInitialized,
                '5' => SynScanError::DriverSleeping,
                '7' => SynScanError::PECTrainingRunning,
                '8' => SynScanError::NoValidPECData,
                _ => SynScanError::CommunicationError(io::Error::from(InvalidData)),
            };
        }
    }

    impl<'a> MotorControllerPort<'a> {
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
                    ErrorKind::InvalidData,
                    e,
                ))),
            }
        }

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

        fn check_amount_read(&self, size: usize) -> io::Result<()> {
            if size != 0 {
                Ok(())
            } else {
                Err(io::Error::from(TimedOut))
            }
        }

        fn consume_term_char(&mut self) -> io::Result<()> {
            let mut termination_char = 0u8;
            let bytes_read = self
                .serial_port
                .read(slice::from_mut(&mut termination_char))?;
            self.check_amount_read(bytes_read)?;
            if termination_char != b'\r' {
                return Err(io::Error::from(InvalidData));
            }
            Ok(())
        }

        fn read_response(&mut self) -> SynScanResult<Vec<u8>> {
            // Read first byte
            let mut byte = 0u8;
            let bytes_read = self.serial_port.read(slice::from_mut(&mut byte))?;
            self.check_amount_read(bytes_read)?;

            return if byte == b'=' {
                // Successful
                const MAX_VALID_RESPONSE: usize = 6;
                let mut buf = Vec::with_capacity(MAX_VALID_RESPONSE);
                let mut i = 0;
                loop {
                    let mut byte = 0u8;
                    let bytes_read = self.serial_port.read(slice::from_mut(&mut byte))?;
                    self.check_amount_read(bytes_read)?;
                    if byte == b'\r' {
                        break;
                    } else {
                        if i < MAX_VALID_RESPONSE {
                            buf.push(byte);
                            i += 1;
                        } else {
                            return Err(SynScanError::CommunicationError(io::Error::from(
                                InvalidData,
                            )));
                        }
                    }
                }

                Ok(buf)
            } else if byte == b'!' {
                // Error Code
                let bytes_read = self.serial_port.read(slice::from_mut(&mut byte))?;
                self.check_amount_read(bytes_read)?;
                self.consume_term_char()?;
                Err(SynScanError::get_controller_error(byte))
            } else {
                Err(SynScanError::CommunicationError(io::Error::from(
                    InvalidData,
                )))
            };
        }

        fn raw_send_cmd(
            &mut self,
            cmd: u8,
            channel: &dyn Channel,
            bytes: &[u8],
        ) -> SynScanResult<Vec<u8>> {
            let mut full_cmd = Vec::with_capacity(bytes.len() + 4);
            full_cmd.push(b':');
            full_cmd.push(cmd);
            full_cmd.push(channel.get_byte());
            full_cmd.extend(bytes);
            full_cmd.push(b'\r');
            match self.serial_port.write_all(full_cmd.as_slice()) {
                Ok(_) => self.read_response(),
                Err(e) => Err(SynScanError::CommunicationError(e)),
            }
        }

        pub fn inquire_bytes(&mut self, cmd: u8, channel: &dyn Channel) -> SynScanResult<Vec<u8>> {
            let response = self.raw_send_cmd(cmd, channel, &[])?;
            if response.is_empty() {
                Err(SynScanError::CommunicationError(io::Error::from(
                    InvalidData,
                )))
            } else {
                Ok(response)
            }
        }

        pub fn inquire_number(&mut self, cmd: u8, channel: &dyn Channel) -> SynScanResult<u32> {
            Self::bytes_to_number(self.inquire_bytes(cmd, channel)?)
        }

        pub fn send_cmd_bytes(
            &mut self,
            cmd: u8,
            channel: &dyn Channel,
            bytes: &[u8],
        ) -> SynScanResult<()> {
            self.raw_send_cmd(cmd, channel, bytes)?;
            Ok(())
        }

        pub fn send_cmd_number(
            &mut self,
            cmd: u8,
            channel: &dyn Channel,
            number: u32,
            num_bytes: usize,
        ) -> SynScanResult<()> {
            self.send_cmd_bytes(
                cmd,
                channel,
                Self::number_to_bytes(number, num_bytes).as_slice(),
            )
        }

        pub fn send_cmd(&mut self, cmd: u8, channel: &dyn Channel) -> SynScanResult<()> {
            self.send_cmd_bytes(cmd, channel, &[])
        }
    }

    #[cfg(test)]
    mod tests {
        use crate::MotorControllerPort;

        #[test]
        fn test_get_number_from_response() {
            assert_eq!(
                MotorControllerPort::bytes_to_number(vec![b'3', b'1']).unwrap(),
                0x31
            );
            assert_eq!(
                MotorControllerPort::bytes_to_number(vec![b'3', b'1', b'7', b'F']).unwrap(),
                0x7f31
            );
            assert_eq!(
                MotorControllerPort::bytes_to_number(vec![b'7', b'2', b'9', b'D', b'B', b'E'])
                    .unwrap(),
                0xBE9D72
            );
            assert_eq!(
                MotorControllerPort::bytes_to_number(vec![b'5', b'6', b'3', b'4', b'1', b'2'])
                    .unwrap(),
                0x123456
            );
        }
        #[test]
        fn test_get_data_from_number() {
            assert_eq!(
                MotorControllerPort::number_to_bytes(22, 2),
                vec![b'1', b'6']
            );
            assert_eq!(
                MotorControllerPort::number_to_bytes(22, 4),
                vec![b'1', b'6', b'0', b'0']
            );
            assert_eq!(
                MotorControllerPort::number_to_bytes(22, 6),
                vec![b'1', b'6', b'0', b'0', b'0', b'0']
            );
            assert_eq!(
                MotorControllerPort::number_to_bytes(123456, 6),
                vec![b'4', b'0', b'E', b'2', b'0', b'1']
            );
        }
    }
}
