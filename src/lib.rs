extern crate core;

mod comm;
mod motors;

use std::error::Error;
use std::ffi::OsStr;
use std::{fmt, io};
use std::time::Duration;
use serial::{BaudRate, SerialPort, SystemPort};


struct SynscanDriverPort {
    serial_port: SystemPort,
    counts_per_revolution: (u32, u32),
}

impl SynscanDriverPort {
    fn new<U: AsRef<OsStr> + ?Sized>(port_name: &U, baud_rate: BaudRate, timeout: Duration) -> io::Result<Self> {
        let mut port = serial::open(port_name)?;
        port.reconfigure(&|settings| {
            settings.set_baud_rate(baud_rate)?;
            Ok(())
        })?;
        port.set_timeout(timeout)?;

        let mut sdp = Self {
            serial_port: port,
            counts_per_revolution: (0, 0)
        };

        // FIXME
        let counts1 = sdp.send_cmd(b'a', b'1', &[])?.unwrap();
        let counts2 = sdp.send_cmd(b'a', b'2', &[])?.unwrap();

        // sdp.counts_per_revolution = (counts1, counts2);

        Ok(sdp)
    }
}



#[derive(Debug)]
pub enum SynScanError {
    UnknownCommand,
    CommandLengthError,
    MotorNotStopped,
    InvalidCharacter,
    NotInitialized,
    DriverSleeping,
    PECTrainingRunning,
    NoValidPECData,
}

impl SynScanError {
    fn get(code: u8) -> SynScanError {
        return match code as char {
            '0' => SynScanError::UnknownCommand,
            '1' => SynScanError::CommandLengthError,
            '2' => SynScanError::MotorNotStopped,
            '3' => SynScanError::InvalidCharacter,
            '4' => SynScanError::NotInitialized,
            '5' => SynScanError::DriverSleeping,
            '7' => SynScanError::PECTrainingRunning,
            '8' => SynScanError::NoValidPECData,
            _ => todo!()
        }
    }
}

impl fmt::Display for SynScanError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let description = match self {
            SynScanError::UnknownCommand => "Unknown Command",
            SynScanError::CommandLengthError => "Command Length Invalid",
            SynScanError::MotorNotStopped => "Motor Must be Stopped",
            SynScanError::InvalidCharacter => "Invalid Character",
            SynScanError::NotInitialized => "Controller Not Initialized",
            SynScanError::DriverSleeping => "Driver is Sleeping",
            SynScanError::PECTrainingRunning => "PEC Training is Running",
            SynScanError::NoValidPECData => "No Valid PEC Data"
        };
        write!(f, "{}", description)
    }
}

impl Error for SynScanError {}



// #[cfg(test)]
// mod tests {
//     use std::time::Duration;
//     use crate::SynscanDriverPort;
//     #[test]
//     fn it_works() {
//         let mut port = SynscanDriverPort::new("/dev/ttyUSB0", serial::Baud115200, Duration::from_millis(1000)).unwrap();
//         println!("{:?}", port.send_dataless_cmd(b'h', b'1'));
//
//         let result = 2 + 2;
//         assert_eq!(result, 4);
//     }
// }
