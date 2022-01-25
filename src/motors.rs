pub mod motors {
    use crate::{SynscanDriverPort, SynScanError};

    impl SynscanDriverPort {
        fn get_pos_counts(&mut self, channel: u8) -> std::io::Result<Result<Vec<u8>, SynScanError>> {
            const COMMAND: u8 = b'j';
            self.send_cmd(COMMAND, channel, &[])
        }

        pub fn get_number_from_response(data: Vec<u8>) -> u32 {
            let mut i = data.len()+1;
            let mut n_vec = vec![0; data.len()];
            for b in data {
                i = match i % 2 {
                    0 => i + 1,
                    _ => i - 3,
                };
                n_vec[i] = b;
            }

            let hex_string: String = n_vec.into_iter().map(|b| b as char).collect();
            // Fixme
            u32::from_str_radix(&hex_string, 16).unwrap()
        }

        pub fn get_data_from_number(number: u32, num_bytes: usize) -> Vec<u8> {
            let hex_string = format!("{:X}", number);
            let hex_bytes = hex_string.into_bytes();
            let mut bytes = vec![b'0'; 6];

            let mut i = 1;
            for b in hex_bytes.into_iter().rev() {
                bytes[i] = b;
                i = match i % 2 {
                    0 => i+3,
                    _ => i-1,
                };
                if i >= num_bytes {
                    break;
                }
            }
            bytes.truncate(num_bytes);
            bytes
        }
    }

    #[cfg(test)]
    mod tests {
        use std::ptr::null;
        use crate::SynscanDriverPort;

        #[test]
        fn test_get_number_from_response() {
            assert_eq!(SynscanDriverPort::get_number_from_response(vec![b'3', b'1']), 0x31);
            assert_eq!(SynscanDriverPort::get_number_from_response(vec![b'3', b'1', b'7', b'f']), 0x7f31);
            assert_eq!(SynscanDriverPort::get_number_from_response(vec![b'5', b'6', b'3', b'4', b'1', b'2']), 0x123456);
        }
        #[test]
        fn test_get_data_from_number() {
            assert_eq!(SynscanDriverPort::get_data_from_number(22, 2), vec![b'1', b'6']);
            assert_eq!(SynscanDriverPort::get_data_from_number(22, 4), vec![b'1', b'6', b'0', b'0']);
            assert_eq!(SynscanDriverPort::get_data_from_number(22, 6), vec![b'1', b'6', b'0', b'0', b'0', b'0']);
            assert_eq!(SynscanDriverPort::get_data_from_number(123456, 6), vec![b'4', b'0', b'E', b'2', b'0', b'1']);
        }
    }
}
