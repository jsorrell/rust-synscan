mod comm {
    use std::{io, slice};
    use std::io::{BufRead, BufReader, Read, Write};
    use std::io::ErrorKind::{InvalidData, TimedOut};
    use crate::{SynscanDriverPort, SynScanError};


    impl SynscanDriverPort {
        fn check_amount_read(&self, size: usize) -> io::Result<()> {
            if size == 0 {
                return Err(io::Error::from(TimedOut));
            }
            Ok(())
        }

        fn check_termination_character(&mut self) -> io::Result<()> {
            let mut termination_char = 0u8;
            let bytes_read = self.serial_port.read(slice::from_mut(&mut termination_char))?;
            self.check_amount_read(bytes_read)?;
            if termination_char != b'\r' {
                return Err(io::Error::from(InvalidData));
            }
            Ok(())
        }


        fn read_response(&mut self) -> io::Result<Result<Vec<u8>, SynScanError>> {
            let mut byte = 0u8;
            let bytes_read = self.serial_port.read(slice::from_mut(&mut byte))?;
            self.check_amount_read(bytes_read)?;

            return if byte == b'=' {
                // Successful
                let mut buf = Vec::with_capacity(7);
                let reader = BufReader::new(&mut self.serial_port);
                let bytes_read = reader.take(7).read_until(b'\r', &mut buf)?;
                self.check_amount_read(bytes_read)?;
                if *buf.last().expect("Missing termination character") != b'\r' {
                    return Err(io::Error::from(InvalidData));
                }
                buf.truncate(bytes_read - 1);
                Ok(Ok(buf))
            } else if byte == b'!' {
                // Error Code
                let bytes_read = self.serial_port.read(slice::from_mut(&mut byte))?;
                self.check_amount_read(bytes_read)?;
                self.check_termination_character()?;
                Ok(Err(SynScanError::get(byte)))
            } else {
                Err(io::Error::from(InvalidData))
            }
        }

        fn send_raw_cmd(&mut self, full_cmd: &[u8]) -> io::Result<Result<Vec<u8>, SynScanError>> {
            self.serial_port.write_all(full_cmd)?;
            self.read_response()
        }

        pub fn send_cmd(&mut self, cmd: u8, channel: u8, data: &[u8]) -> io::Result<Result<Vec<u8>, SynScanError>> {
            let mut full_cmd = Vec::with_capacity(data.len() + 4);
            full_cmd.push(b':');
            full_cmd.push(cmd);
            full_cmd.push(channel);
            full_cmd.extend(data);
            full_cmd.push(b'\r');
            self.send_raw_cmd(full_cmd.as_slice())
        }

        pub fn send_dataless_cmd(&mut self, cmd: u8, channel: u8) -> io::Result<Result<Vec<u8>, SynScanError>> {
            self.send_cmd(cmd, channel, &[])
        }

        pub fn test_comm(&mut self) -> bool {
            let result = self.send_cmd(b'F', b'3', &[]);
            match result {
                Ok(Ok(_)) => true,
                _ => false
            }
        }
    }

}
