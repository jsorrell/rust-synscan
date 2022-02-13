use super::synscan_port::{bytes_to_number, number_to_bytes};
use crate::port::mock::MockSynScanPort;
use crate::*;

impl MockSynScanPort {
    pub(crate) fn add_valid_number(&self, number: u32, num_bytes: usize) {
        self.add_valid_response(number_to_bytes(number, num_bytes).as_slice())
    }

    pub(crate) fn check_correct_number_written(
        &self,
        command: u8,
        channel: impl Channel,
        number: u32,
        num_bytes: usize,
    ) {
        self.check_correct_query_written(
            command,
            channel,
            number_to_bytes(number, num_bytes).as_slice(),
        )
    }
}

#[test]
fn test_get_number_from_response() {
    assert_eq!(bytes_to_number(vec![b'3', b'1']).unwrap(), 0x31);
    assert_eq!(
        bytes_to_number(vec![b'3', b'1', b'7', b'F']).unwrap(),
        0x7f31
    );
    assert_eq!(
        bytes_to_number(vec![b'7', b'2', b'9', b'D', b'B', b'E']).unwrap(),
        0xBE9D72
    );
    assert_eq!(
        bytes_to_number(vec![b'5', b'6', b'3', b'4', b'1', b'2']).unwrap(),
        0x123456
    );
}

#[test]
fn test_get_data_from_number() {
    assert_eq!(number_to_bytes(22, 2), vec![b'1', b'6']);
    assert_eq!(number_to_bytes(22, 4), vec![b'1', b'6', b'0', b'0']);
    assert_eq!(
        number_to_bytes(22, 6),
        vec![b'1', b'6', b'0', b'0', b'0', b'0']
    );
    assert_eq!(
        number_to_bytes(123456, 6),
        vec![b'4', b'0', b'E', b'2', b'0', b'1']
    );
}
