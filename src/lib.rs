extern crate core;

mod comm;
pub mod mock;
pub mod motors;
pub mod result;
pub mod util;

use crate::result::{SynScanError, SynScanResult};
use crate::util::*;
use serialport;
use serialport::SerialPort;
use std::time::Duration;

/// Port for serial communication to SynScan Motor Controller
struct MotorControllerPort<'a> {
    serial_port: Box<dyn SerialPort + 'a>,
}

#[derive(Copy, Clone, Hash, Debug)]
pub struct MotorParameters {
    counts_per_revolution: BiChannelValue<u32>,
    timer_interrupt_freq: u32,
    //todo motor_board_version: (u32, u32),
    high_speed_ratio: BiChannelValue<u32>,
}

impl MotorParameters {
    pub fn get_counts_per_revolution(&self, channel: SingleChannel) -> u32 {
        self.counts_per_revolution[channel]
    }

    pub fn get_timer_interrupt_frequency(&self) -> u32 {
        self.timer_interrupt_freq
    }

    pub fn get_high_speed_ratio(&self, channel: SingleChannel) -> u32 {
        self.high_speed_ratio[channel]
    }

    pub fn counts_to_degrees(&self, channel: SingleChannel, counts: f64) -> f64 {
        let counts_per_rev = self.get_counts_per_revolution(channel);
        counts as f64 / counts_per_rev as f64 * 360f64
    }

    pub fn degrees_to_counts(&self, channel: SingleChannel, degrees: f64) -> f64 {
        let counts_per_rev = self.get_counts_per_revolution(channel);
        (degrees / 360f64) * counts_per_rev as f64
    }
}

pub struct MotorController<'a> {
    port: MotorControllerPort<'a>,
    motor_parameters: MotorParameters,
}

impl<'a> MotorController<'a> {
    pub fn new(
        path: &'a str,
        baud_rate: u32,
        timeout: Duration,
    ) -> Result<MotorController, SynScanError> {
        Self::new_with_port(serialport::new(path, baud_rate).timeout(timeout).open()?)
    }

    pub fn new_with_port(port: Box<dyn SerialPort + 'a>) -> SynScanResult<Self> {
        let mut port = MotorControllerPort { serial_port: port };

        let motor_parameters = MotorParameters {
            counts_per_revolution: BiChannelValue::new_from_result_fn(|c| {
                port.inquire_number(b'a', &c)
            })?,
            timer_interrupt_freq: port.inquire_number(b'b', &SingleChannel::Channel1)?,
            // motor_board_version: (0, 0),
            high_speed_ratio: BiChannelValue::new_from_result_fn(|c| {
                port.inquire_number(b'g', &c)
            })?,
        };

        Ok(Self {
            port,
            motor_parameters,
        })
    }
}
