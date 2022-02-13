use crate::port::commands::*;
use crate::util::*;
use crate::*;

impl<T: SerialPort> MotorController<T> {
    /// Sets the goto target in encoder counts relative to where the mount was initialized
    /// Positive counts are clockwise
    pub fn set_goto_target_counts(
        &mut self,
        channel: impl Channel,
        counts: i32,
    ) -> SynScanResult<()> {
        self.port
            .send_cmd_number(SET_GOTO_TARGET, channel, (counts + 0x800000) as u32, 6)
    }

    /// Sets the goto target in degrees relative to where the mount was initialized
    /// Positive degrees are clockwise
    pub fn set_goto_target(&mut self, channel: SingleChannel, degrees: f64) -> SynScanResult<()> {
        let counts = self
            .motor_parameters
            .degrees_to_counts(channel, degrees)
            .round() as i32;
        self.set_goto_target_counts(channel, counts)
    }

    /// Reads the goto target in counts relative to where the mount was initialized
    /// Positive counts are clockwise
    pub fn inquire_goto_target(&mut self, channel: SingleChannel) -> SynScanResult<i32> {
        let counts = self
            .port
            .inquire_number(INQUIRE_GOTO_TARGET_POSITION, channel)?;
        Ok(counts as i32 - 0x800000)
    }

    /// Reads the goto target in degrees relative to where the mount was initialized
    /// Positive degrees are clockwise
    pub fn inquire_goto_target_degrees(&mut self, channel: SingleChannel) -> SynScanResult<f64> {
        let counts = self.inquire_goto_target(channel)?;
        Ok(self
            .motor_parameters
            .counts_to_degrees(channel, counts as f64))
    }
}
