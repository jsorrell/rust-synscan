use crate::motor_controller::*;
use crate::port::commands::{INQUIRE_POSITION, SET_POSITION};
use crate::util::*;

impl<T: SerialPort> MotorController<T> {
    /// Gets the position of the mount in number of steps relative to initialization.
    pub fn inquire_pos(&mut self, channel: SingleChannel) -> SynScanResult<i32> {
        let counts = self.port.inquire_number(INQUIRE_POSITION, channel)?;
        // Data is offset by 0x800000 according to spec
        Ok(counts as i32 - 0x800000)
    }

    /// Gets the position of the mount in number of degrees relative to initialization.
    pub fn inquire_pos_degrees(&mut self, channel: SingleChannel) -> SynScanResult<f64> {
        let counts = self.inquire_pos(channel)?;
        Ok(self
            .motor_parameters
            .counts_to_degrees(channel, counts as f64))
    }

    /// Sets the position of the mount in number of steps.
    /// Only changes encoder number -- doesn't move mount.
    pub fn set_pos(&mut self, channel: impl Channel, counts: i32) -> SynScanResult<()> {
        // Data is offset by 0x800000 according to spec
        self.port
            .send_cmd_number(SET_POSITION, channel, (counts + 0x800000) as u32, 6)
    }

    /// Sets the position of the mount in number of degrees.
    /// Only changes encoder number -- doesn't move mount.
    pub fn set_pos_degrees(&mut self, channel: SingleChannel, degrees: f64) -> SynScanResult<()> {
        let counts = self
            .motor_parameters
            .degrees_to_counts(channel, degrees)
            .round() as i32;
        self.set_pos(channel, counts)
    }
}
