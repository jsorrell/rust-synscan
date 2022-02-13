use crate::port::commands::*;
use crate::util::*;
use crate::*;

impl<T: SerialPort> MotorController<T> {
    /// Reads the step period of the mount.
    /// This is the interval of clock cycles the mount will wait before attempting to move the stepper motor.
    /// This is used to determine the speed of the mount
    pub fn inquire_step_period(&mut self, channel: SingleChannel) -> SynScanResult<u32> {
        self.port.inquire_number(INQUIRE_STEP_PERIOD, channel)
    }

    /// Sets the step period of the mount.
    /// This is the interval of clock cycles the mount will wait before attempting to move the stepper motor.
    /// This is used to control the speed of the mount
    /// This will error if trying to change the period when moving in high speed mode
    pub fn set_step_period(&mut self, channel: impl Channel, period: u32) -> SynScanResult<()> {
        self.port
            .send_cmd_number(SET_STEP_PERIOD, channel, period, 6)
    }

    /// Returns the current high speed ratio of the mount depending on whether it is in fast or slow mode.
    pub fn determine_motion_rate_multiplier(
        &mut self,
        channel: SingleChannel,
    ) -> SynScanResult<f64> {
        if !self.inquire_status(channel)?.fast {
            return Ok(1.);
        }
        Ok(self.motor_parameters.high_speed_ratio[channel] as f64)
    }

    /// Calculates the current motion of the mount depending on the step period and whether it is in fast or slow mode.
    fn determine_motion_rate(
        &mut self,
        channel: SingleChannel,
        step_period: u32,
    ) -> SynScanResult<f64> {
        Ok(self.determine_motion_rate_multiplier(channel)?
            * (self.motor_parameters.timer_interrupt_freq as f64 / step_period as f64))
    }

    /// Calculates the set motion rate of the mount in counts per second
    /// This can be non-zero even if the mount is stopped
    pub fn inquire_motion_rate_counts(&mut self, channel: SingleChannel) -> SynScanResult<f64> {
        let step_period = self.inquire_step_period(channel)?;
        self.determine_motion_rate(channel, step_period)
    }

    /// Calculates the set motion rate of the mount in degrees per second
    /// This can be non-zero even if the mount is stopped
    pub fn inquire_motion_rate_degrees(&mut self, channel: SingleChannel) -> SynScanResult<f64> {
        let counts = self.inquire_motion_rate_counts(channel)?;
        Ok(self.motor_parameters.counts_to_degrees(channel, counts))
    }

    /// Sets the motion rate of the mount in counts per second.
    /// Doesn't start a stopped mount. Use start_motion as well.
    pub fn set_motion_rate_counts(
        &mut self,
        channel: SingleChannel,
        counts_per_sec: f64,
    ) -> SynScanResult<()> {
        let multiplier = self.determine_motion_rate_multiplier(channel)?;
        let target_step_period = (multiplier
            * (self.motor_parameters.timer_interrupt_freq as f64 / counts_per_sec))
            .round() as u32;
        self.set_step_period(channel, target_step_period)
    }

    /// Sets the motion rate of the mount in degrees per second.
    /// Doesn't start a stopped mount. Use start_motion as well.
    pub fn set_motion_rate_degrees(
        &mut self,
        channel: SingleChannel,
        degrees_per_sec: f64,
    ) -> SynScanResult<()> {
        self.set_motion_rate_counts(
            channel,
            self.motor_parameters
                .degrees_to_counts(channel, degrees_per_sec),
        )
    }

    /// Starts tracking or goto of a stopped mount.
    pub fn start_motion(&mut self, channel: impl Channel) -> SynScanResult<()> {
        self.port.send_cmd(START_MOTION, channel)
    }

    /// Stops motion of the mount.
    pub fn stop_motion(&mut self, channel: impl Channel) -> SynScanResult<()> {
        self.port.send_cmd(STOP_MOTION, channel)
    }

    /// Instantly stops motion of the mount.
    pub fn instant_stop(&mut self, channel: impl Channel) -> SynScanResult<()> {
        self.port.send_cmd(INSTANT_STOP, channel)
    }
}
