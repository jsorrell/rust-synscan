use crate::result::SynScanResult;
use crate::util::*;
use crate::{MotorController, MotorParameters, SynScanError};
use std::fmt::{Display, Formatter};
use std::io::ErrorKind;
use std::{fmt, io};

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Direction {
    Clockwise,
    CounterClockwise,
}

impl Direction {
    pub fn opposite(&self) -> Self {
        match self {
            Direction::Clockwise => Direction::CounterClockwise,
            Direction::CounterClockwise => Direction::Clockwise,
        }
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Direction::Clockwise => write!(f, "Clockwise"),
            Direction::CounterClockwise => write!(f, "CounterClockwise"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum DriveMode {
    Goto,
    Tracking,
}

impl Display for DriveMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            DriveMode::Goto => write!(f, "Goto"),
            DriveMode::Tracking => write!(f, "Tracking"),
        }
    }
}

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

impl Display for MotorStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}Inited\n", if self.inited { "" } else { "Not " })?;
        write!(f, "{}", if self.blocked { "Blocked!!!\n" } else { "" })?;
        write!(
            f,
            "{} {} {} ({})\n",
            self.mode,
            self.direction,
            if self.running { "Running" } else { "Stopped" },
            if self.fast { "Fast" } else { "Slow" }
        )?;
        write!(f, "Level Switch: {}", self.level_switch)
    }
}

impl<'a> MotorController<'a> {
    pub fn get_motor_parameters(&self) -> &MotorParameters {
        return &self.motor_parameters;
    }

    // Number of steps from turning on
    pub fn get_pos_counts(&mut self, channel: &SingleChannel) -> SynScanResult<i32> {
        const COMMAND: u8 = b'j';
        let counts = self.port.inquire_number(COMMAND, channel)?;
        Ok(counts as i32 - 0x800000)
    }

    pub fn test_com(&mut self) -> SynScanResult<()> {
        self.port.send_cmd(b'F', &MultiChannel::Both)
    }

    // Number of degrees from turning on
    pub fn get_pos(&mut self, channel: &SingleChannel) -> SynScanResult<f64> {
        let counts = self.get_pos_counts(channel)?;
        Ok(self
            .motor_parameters
            .counts_to_degrees(*channel, counts as f64))
    }

    pub fn get_step_period(&mut self, channel: &SingleChannel) -> SynScanResult<u32> {
        const COMMAND: u8 = b'i';
        self.port.inquire_number(COMMAND, channel)
    }

    pub fn get_status(&mut self, channel: &SingleChannel) -> SynScanResult<MotorStatus> {
        const COMMAND: u8 = b'f';
        let data = self.port.inquire_bytes(COMMAND, channel)?;
        let bytes = data
            .into_iter()
            .map(|b| {
                if b'0' <= b && b <= b'9' {
                    Ok(b - b'0')
                } else if b'A' <= b && b <= b'F' {
                    Ok(b - b'A')
                } else {
                    Err(SynScanError::CommunicationError(io::Error::from(
                        ErrorKind::InvalidData,
                    )))
                }
            })
            .collect::<SynScanResult<Vec<u8>>>()?;

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

    pub fn is_running(&mut self, channel: &SingleChannel) -> SynScanResult<bool> {
        let status = self.get_status(channel)?;
        Ok(status.running)
    }

    pub fn set_motion_mode(
        &mut self,
        channel: &dyn Channel,
        mode: DriveMode,
        fast: bool,
        direction: Direction,
    ) -> SynScanResult<()> {
        const COMMAND: u8 = b'G';
        // TODO What do all the superfluous speed bits mean?
        let (mut byte0, mut byte1) = (0, 0);
        if mode == DriveMode::Tracking {
            byte0 = byte0 | 0x1;
        }

        if fast == (mode == DriveMode::Tracking) {
            byte0 = byte0 | 0x2;
        }

        if direction == Direction::CounterClockwise {
            byte1 = byte1 | 0x1;
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

        self.port.send_cmd_bytes(COMMAND, channel, &[byte0, byte1])
    }

    pub fn set_step_period(&mut self, channel: &dyn Channel, period: u32) -> SynScanResult<()> {
        const COMMAND: u8 = b'I';
        self.port.send_cmd_number(COMMAND, channel, period, 6)
    }

    pub fn set_autoguide_speed(
        &mut self,
        channel: &dyn Channel,
        speed: AutoGuideSpeed,
    ) -> SynScanResult<()> {
        const COMMAND: u8 = b'P';
        self.port
            .send_cmd_bytes(COMMAND, channel, vec![speed.comm_byte()].as_slice())
    }

    pub fn set_pos_counts(&mut self, channel: &dyn Channel, counts: i32) -> SynScanResult<()> {
        const COMMAND: u8 = b'E';
        // Data is offset by 0x800000 according to spec
        self.port
            .send_cmd_number(COMMAND, channel, (counts + 0x800000) as u32, 6)
    }

    pub fn set_pos(&mut self, channel: &SingleChannel, degrees: f64) -> SynScanResult<()> {
        let counts = self
            .motor_parameters
            .degrees_to_counts(*channel, degrees)
            .round() as i32;
        self.set_pos_counts(channel, counts)
    }

    pub fn set_goto_target_counts(
        &mut self,
        channel: &dyn Channel,
        counts: i32,
    ) -> SynScanResult<()> {
        const COMMAND: u8 = b'S';
        self.port
            .send_cmd_number(COMMAND, channel, (counts + 0x800000) as u32, 6)
    }

    pub fn set_goto_target(&mut self, channel: &SingleChannel, degrees: f64) -> SynScanResult<()> {
        let counts = self
            .motor_parameters
            .degrees_to_counts(*channel, degrees)
            .round() as i32;
        self.set_goto_target_counts(channel, counts)
    }

    pub fn get_goto_target_counts(&mut self, channel: &SingleChannel) -> SynScanResult<i32> {
        const COMMAND: u8 = b'h';
        let counts = self.port.inquire_number(COMMAND, channel)?;
        Ok(counts as i32 - 0x800000)
    }

    pub fn get_goto_target(&mut self, channel: &SingleChannel) -> SynScanResult<f64> {
        let counts = self.get_goto_target_counts(channel)?;
        Ok(self
            .motor_parameters
            .counts_to_degrees(*channel, counts as f64))
    }

    pub fn start_motion(&mut self, channel: &dyn Channel) -> SynScanResult<()> {
        const COMMAND: u8 = b'J';
        self.port.send_cmd(COMMAND, channel)
    }

    pub fn stop_motion(&mut self, channel: &dyn Channel, instant: bool) -> SynScanResult<()> {
        let cmd = if instant { b'L' } else { b'K' };
        self.port.send_cmd(cmd, channel)
    }

    fn offline_get_motion_rate_counts(
        &mut self,
        channel: &SingleChannel,
        step_period: u32,
        fast: bool,
    ) -> f64 {
        let multiplier = if fast {
            self.motor_parameters.high_speed_ratio[*channel] as f64
        } else {
            1f64
        };
        multiplier * (self.motor_parameters.timer_interrupt_freq as f64 / step_period as f64)
    }

    pub fn get_motion_rate_counts(&mut self, channel: &SingleChannel) -> SynScanResult<f64> {
        let step_period = self.get_step_period(channel)?;
        let fast = self.get_status(channel)?.fast;
        Ok(self.offline_get_motion_rate_counts(channel, step_period, fast))
    }

    pub fn get_motion_rate_degrees(&mut self, channel: &SingleChannel) -> SynScanResult<f64> {
        let counts = self.get_motion_rate_counts(channel)?;
        Ok(self.motor_parameters.counts_to_degrees(*channel, counts))
    }

    pub fn set_motion_rate_counts(
        &mut self,
        channel: &SingleChannel,
        counts_per_sec: f64,
        fast: bool,
    ) -> SynScanResult<()> {
        let multiplier = if fast {
            self.motor_parameters.high_speed_ratio[*channel] as f64
        } else {
            1.
        };
        let target_step_period = ((multiplier as f64)
            * (self.motor_parameters.timer_interrupt_freq as f64 / counts_per_sec))
            .round() as u32;
        self.set_step_period(channel, target_step_period)
    }

    pub fn set_motion_rate_degrees(
        &mut self,
        channel: &SingleChannel,
        degrees_per_sec: f64,
        fast: bool,
    ) -> SynScanResult<()> {
        self.set_motion_rate_counts(
            channel,
            self.motor_parameters
                .degrees_to_counts(*channel, degrees_per_sec),
            fast,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mock::MockSynScanPort;
    use crate::motors::Direction::{Clockwise, CounterClockwise};
    use crate::MotorControllerPort;

    fn get_mc(mock: &mut MockSynScanPort, params: Option<MotorParameters>) -> MotorController {
        let params = params.unwrap_or(MotorParameters {
            counts_per_revolution: BiChannelValue::new(169499, 180),
            timer_interrupt_freq: 1000,
            high_speed_ratio: BiChannelValue::new(16, 2),
        });

        MotorController {
            port: MotorControllerPort {
                serial_port: Box::new(mock),
            },
            motor_parameters: params,
        }
    }

    #[test]
    fn test_set_pos() {
        let mut mock = MockSynScanPort::new();
        mock.add_valid_response(&[]);
        mock.add_valid_response(&[]);
        let mut mc = get_mc(&mut mock, None);
        mc.set_pos_counts(&MultiChannel::Both, -1234).unwrap();
        mc.set_pos(&SingleChannel::Channel2, 90.).unwrap();
        drop(mc);
        let mut correct_bytes = vec![b':', b'E', b'3'];
        correct_bytes.append(&mut MotorControllerPort::number_to_bytes(
            0x800000 - 1234,
            6,
        ));
        correct_bytes.append(&mut vec![b'\r', b':', b'E', b'2']);
        correct_bytes.append(&mut MotorControllerPort::number_to_bytes(0x800000 + 45, 6));
        correct_bytes.append(&mut vec![b'\r']);

        assert_eq!(
            mock.get_written_bytes().as_slice(),
            correct_bytes.as_slice()
        );
    }

    #[test]
    fn test_set_motion_mode() {
        let mut mock = MockSynScanPort::new();
        mock.add_valid_response(&[]);
        mock.add_valid_response(&[]);
        let mut mc = get_mc(&mut mock, None);
        mc.set_motion_mode(&SingleChannel::Channel1, DriveMode::Goto, true, Clockwise)
            .unwrap();
        mc.set_motion_mode(
            &SingleChannel::Channel2,
            DriveMode::Tracking,
            true,
            CounterClockwise,
        )
        .unwrap();
        drop(mc);

        let mut correct_bytes = vec![b':', b'G', b'1', b'0', b'0', b'\r'];
        correct_bytes.append(&mut vec![b':', b'G', b'2', b'3', b'1', b'\r']);

        assert_eq!(
            mock.get_written_bytes().as_slice(),
            correct_bytes.as_slice()
        );
    }

    #[test]
    fn test_set_goto_target() {
        let mut mock = MockSynScanPort::new();
        mock.add_valid_response(&[]);
        mock.add_valid_response(&[]);
        let mut mc = get_mc(&mut mock, None);
        mc.set_goto_target_counts(&MultiChannel::Both, -1234)
            .unwrap();
        mc.set_goto_target(&SingleChannel::Channel2, 90.).unwrap();
        drop(mc);
        let mut correct_bytes = vec![b':', b'S', b'3'];
        correct_bytes.append(&mut MotorControllerPort::number_to_bytes(
            0x800000 - 1234,
            6,
        ));
        correct_bytes.append(&mut vec![b'\r', b':', b'S', b'2']);
        correct_bytes.append(&mut MotorControllerPort::number_to_bytes(0x800000 + 45, 6));
        correct_bytes.append(&mut vec![b'\r']);

        assert_eq!(
            mock.get_written_bytes().as_slice(),
            correct_bytes.as_slice()
        );
    }

    #[test]
    fn test_set_step_period() {
        let mut mock = MockSynScanPort::new();
        mock.add_valid_response(&[]);
        mock.add_valid_response(&[]);
        mock.add_valid_response(&[]);
        mock.add_valid_response(&[]);
        let mut mc = get_mc(&mut mock, None);
        mc.set_step_period(&MultiChannel::Both, 999).unwrap();
        mc.set_motion_rate_counts(&SingleChannel::Channel2, 1., false)
            .unwrap();
        mc.set_motion_rate_counts(&SingleChannel::Channel2, 2., true)
            .unwrap();
        mc.set_motion_rate_degrees(&SingleChannel::Channel2, 4., false)
            .unwrap();
        drop(mc);
        let mut correct_bytes = vec![b':', b'I', b'3'];
        correct_bytes.append(&mut MotorControllerPort::number_to_bytes(999, 6));
        correct_bytes.append(&mut vec![b'\r', b':', b'I', b'2']);
        correct_bytes.append(&mut MotorControllerPort::number_to_bytes(1000, 6));
        correct_bytes.append(&mut vec![b'\r', b':', b'I', b'2']);
        correct_bytes.append(&mut MotorControllerPort::number_to_bytes(1000, 6));
        correct_bytes.append(&mut vec![b'\r', b':', b'I', b'2']);
        correct_bytes.append(&mut MotorControllerPort::number_to_bytes(500, 6));
        correct_bytes.append(&mut vec![b'\r']);

        assert_eq!(
            mock.get_written_bytes().as_slice(),
            correct_bytes.as_slice()
        );
    }

    #[test]
    fn test_start_stop_motion() {
        let mut mock = MockSynScanPort::new();
        mock.add_valid_response(&[]);
        mock.add_valid_response(&[]);
        mock.add_valid_response(&[]);
        let mut mc = get_mc(&mut mock, None);
        mc.start_motion(&SingleChannel::Channel1).unwrap();
        mc.stop_motion(&MultiChannel::Both, false).unwrap();
        mc.stop_motion(&SingleChannel::Channel2, true).unwrap();
        drop(mc);
        let correct_bytes = vec![
            b':', b'J', b'1', b'\r', b':', b'K', b'3', b'\r', b':', b'L', b'2', b'\r',
        ];
        assert_eq!(
            mock.get_written_bytes().as_slice(),
            correct_bytes.as_slice()
        );
    }

    #[test]
    fn test_set_autoguide_speed() {
        let mut mock = MockSynScanPort::new();
        mock.add_valid_response(&[]);
        let mut mc = get_mc(&mut mock, None);
        mc.set_autoguide_speed(&SingleChannel::Channel1, AutoGuideSpeed::ThreeQuarters)
            .unwrap();
        drop(mc);
        let correct_bytes = vec![b':', b'P', b'1', b'1', b'\r'];
        assert_eq!(
            mock.get_written_bytes().as_slice(),
            correct_bytes.as_slice()
        );
    }

    #[test]
    fn test_get_goto_target() {
        let mut mock = MockSynScanPort::new();
        mock.add_valid_response(MotorControllerPort::number_to_bytes(0x800000 + 25, 6).as_slice());
        mock.add_valid_response(MotorControllerPort::number_to_bytes(0x800000 - 45, 6).as_slice());
        let mut mc = get_mc(&mut mock, None);
        assert_eq!(
            mc.get_goto_target_counts(&SingleChannel::Channel1).unwrap(),
            25
        );
        assert_eq!(mc.get_goto_target(&SingleChannel::Channel2).unwrap(), -90.);
        drop(mc);
        assert_eq!(
            mock.get_written_bytes().as_slice(),
            vec![b':', b'h', b'1', b'\r', b':', b'h', b'2', b'\r'].as_slice()
        );
    }

    #[test]
    fn test_get_step_period() {
        let mut mock = MockSynScanPort::new();
        mock.add_valid_response(MotorControllerPort::number_to_bytes(800, 6).as_slice());
        let mut mc = get_mc(&mut mock, None);
        assert_eq!(mc.get_step_period(&SingleChannel::Channel2).unwrap(), 800);
        drop(mc);
        assert_eq!(
            mock.get_written_bytes().as_slice(),
            vec![b':', b'i', b'2', b'\r'].as_slice()
        );
    }

    #[test]
    fn test_get_pos() {
        let mut mock = MockSynScanPort::new();
        mock.add_valid_response(MotorControllerPort::number_to_bytes(0x800000 + 25, 6).as_slice());
        mock.add_valid_response(MotorControllerPort::number_to_bytes(0x800000 - 45, 6).as_slice());
        let mut mc = get_mc(&mut mock, None);
        assert_eq!(mc.get_pos_counts(&SingleChannel::Channel1).unwrap(), 25);
        assert_eq!(mc.get_pos(&SingleChannel::Channel2).unwrap(), -90.);
        drop(mc);
        assert_eq!(
            mock.get_written_bytes().as_slice(),
            vec![b':', b'j', b'1', b'\r', b':', b'j', b'2', b'\r'].as_slice()
        );
    }

    #[test]
    fn test_get_status() {
        let mut mock = MockSynScanPort::new();
        mock.add_valid_response(&mut vec![b'7', b'1', b'1']);
        mock.add_valid_response(&mut vec![b'0', b'2', b'3']);
        let mut mc = get_mc(&mut mock, None);
        assert_eq!(
            mc.get_status(&SingleChannel::Channel1).unwrap(),
            MotorStatus {
                mode: DriveMode::Tracking,
                direction: Direction::CounterClockwise,
                fast: true,
                running: true,
                blocked: false,
                inited: true,
                level_switch: false
            }
        );
        assert_eq!(
            mc.get_status(&SingleChannel::Channel2).unwrap(),
            MotorStatus {
                mode: DriveMode::Goto,
                direction: Direction::Clockwise,
                fast: false,
                running: false,
                blocked: true,
                inited: true,
                level_switch: true
            }
        );
        drop(mc);
        assert_eq!(
            mock.get_written_bytes().as_slice(),
            vec![b':', b'f', b'1', b'\r', b':', b'f', b'2', b'\r'].as_slice()
        );
    }
}
