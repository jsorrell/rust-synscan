use std::error::Error;
use std::{fmt, io};

pub type SynScanResult<T> = Result<T, SynScanError>;

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
    CommunicationError(io::Error),
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
            SynScanError::NoValidPECData => "No Valid PEC Data",
            SynScanError::CommunicationError(e) => return write!(f, "Communication Error: {}", e),
        };
        write!(f, "{}", description)
    }
}

impl Error for SynScanError {}

impl From<io::Error> for SynScanError {
    fn from(e: io::Error) -> Self {
        SynScanError::CommunicationError(e)
    }
}
