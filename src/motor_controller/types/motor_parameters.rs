use crate::port::commands::*;
use crate::port::SynScanPort;
use crate::util::*;
use crate::SingleChannel;
use crate::*;

/// Constant motor parameters given by the mount
#[derive(Copy, Clone, Debug)]
pub struct MotorParameters {
    pub counts_per_revolution: BiChannelValue<u32>,
    pub timer_interrupt_freq: u32,
    //todo motor_board_version: (u32, u32),
    pub high_speed_ratio: BiChannelValue<u32>,
}

impl MotorParameters {
    pub fn counts_to_degrees(&self, channel: SingleChannel, counts: f64) -> f64 {
        let counts_per_rev = self.counts_per_revolution[channel];
        counts as f64 / counts_per_rev as f64 * 360.
    }

    pub fn degrees_to_counts(&self, channel: SingleChannel, degrees: f64) -> f64 {
        let counts_per_rev = self.counts_per_revolution[channel];
        (degrees / 360.) * counts_per_rev as f64
    }
}

impl<T: SerialPort> SynScanPort<T> {
    pub fn get_motor_parameters(&self) -> SynScanResult<MotorParameters> {
        Ok(MotorParameters {
            counts_per_revolution: SingleChannel::VALUES
                .into_iter()
                .map(|c| self.inquire_number(INQUIRE_COUNTS_PER_REVOLUTION, c))
                .collect::<SynScanResult<BiChannelValue<u32>>>()?,
            timer_interrupt_freq: self
                .inquire_number(INQUIRE_TIMER_INTERRUPT_FREQUENCY, SingleChannel::Channel1)?,
            high_speed_ratio: SingleChannel::VALUES
                .into_iter()
                .map(|c| self.inquire_number(INQUIRE_HIGH_SPEED_RATIO, c))
                .collect::<SynScanResult<BiChannelValue<u32>>>()?,
        })
    }
}
