use crate::port::commands::*;
use crate::util::*;
use crate::Direction::*;
use crate::DriveMode::*;
use crate::*;
use std::io;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct MotorStatus {
    pub mode: DriveMode,
    pub direction: Direction,
    pub fast: bool,
    pub running: bool,
    pub blocked: bool,
    pub inited: bool,
    pub level_switch: bool,
}

impl<T: SerialPort> MotorController<T> {
    /// Returns a [MotorStatus] describing the mount status
    pub fn inquire_status(&self, channel: SingleChannel) -> SynScanResult<MotorStatus> {
        let data = self.port.inquire_bytes(INQUIRE_STATUS, channel)?;
        let bytes = data
            .into_iter()
            .map(|b| {
                if (b'0'..=b'9').contains(&b) {
                    Ok(b - b'0')
                } else if (b'A'..=b'F').contains(&b) {
                    Ok(b - b'A')
                } else {
                    Err(SynScanError::CommunicationError(io::Error::from(
                        io::ErrorKind::InvalidData,
                    )))
                }
            })
            .collect::<SynScanResult<Vec<u8>>>()?;

        if bytes.len() != 3 {
            return Err(SynScanError::CommunicationError(io::Error::from(
                io::ErrorKind::InvalidData,
            )));
        }

        Ok(MotorStatus {
            mode: if bytes[0] & 0x1 != 0 {
                DriveMode::Tracking
            } else {
                DriveMode::Goto
            },
            direction: if bytes[0] & 0x2 != 0 {
                Direction::CounterClockwise
            } else {
                Direction::Clockwise
            },
            fast: bytes[0] & 0x4 != 0,
            running: bytes[1] & 0x1 != 0,
            blocked: bytes[1] & 0x2 != 0,
            inited: bytes[2] & 0x1 != 0,
            level_switch: bytes[2] & 0x2 != 0,
        })
    }

    /// Sets the motion mode to either fast or slow GOTO mode
    /// Errors if called when the mount is not stopped
    pub fn set_goto_motion_mode(&self, channel: impl Channel, fast: bool) -> SynScanResult<()> {
        self.set_motion_mode(channel, Goto, fast, Clockwise) // direction doesn't do anything
    }

    /// Sets the motion mode to either fast or slow Tracking mode in the given direction
    /// Errors if called when the mount is not stopped
    pub fn set_tracking_motion_mode(
        &self,
        channel: impl Channel,
        fast: bool,
        direction: Direction,
    ) -> SynScanResult<()> {
        self.set_motion_mode(channel, Tracking, fast, direction)
    }

    /// Sets the motion mode
    /// Errors if called when the mount is not stopped
    pub fn set_motion_mode(
        &self,
        channel: impl Channel,
        mode: DriveMode,
        fast: bool,
        direction: Direction,
    ) -> SynScanResult<()> {
        // TODO What do all the superfluous speed bits mean?
        let (mut byte0, mut byte1) = (0, 0);
        if mode == DriveMode::Tracking {
            byte0 |= 0x1;
        }

        if fast == (mode == DriveMode::Tracking) {
            byte0 |= 0x2;
        }

        if direction == Direction::CounterClockwise {
            byte1 |= 0x1;
        }

        byte0 = if byte0 <= 9 {
            byte0 + b'0'
        } else {
            byte0 + b'A'
        };
        byte1 = if byte1 <= 9 {
            byte1 + b'0'
        } else {
            byte1 + b'A'
        };

        self.port
            .send_cmd_bytes(SET_MOTION_MODE, channel, &[byte0, byte1])
    }
}
