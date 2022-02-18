mod goto;
mod motion_rate;
mod pos;
mod status;

mod types {
    mod autoguide_speed;
    mod direction;
    mod drive_mode;
    mod motor_parameters;

    pub use autoguide_speed::*;
    pub use direction::*;
    pub use drive_mode::*;
    pub use motor_parameters::*;
}

#[cfg(test)]
mod tests;

use crate::port::commands::*;
use crate::port::SynScanPort;
use crate::util::*;
use crate::*;

pub use goto::*;
pub use motion_rate::*;
pub use pos::*;
pub use status::*;
pub use types::*;

/// A MotorController is a handle for controlling the SkyWatcher mount through a serial port
pub struct MotorController<T: SerialPort> {
    port: SynScanPort<T>,
    motor_parameters: MotorParameters,
}

impl<T> MotorController<T>
where
    T: SerialPort,
{
    /// Returns a new MotorController attached through the given port
    pub fn new(port: T) -> SynScanResult<Self> {
        let port = SynScanPort::new(port);
        port.test()?;

        let motor_parameters = port.get_motor_parameters()?;
        Ok(Self {
            port,
            motor_parameters,
        })
    }
}

impl<T: SerialPort> MotorController<T> {
    /// Returns the motor parameters for the controller. These are static and eagerly queried.
    pub fn get_motor_parameters(&self) -> &MotorParameters {
        &self.motor_parameters
    }

    /// Tests the mount is connected
    pub fn test(&self) -> SynScanResult<()> {
        self.port.test()
    }

    /// Sets the autoguide speed of the mount
    pub fn set_autoguide_speed(
        &self,
        channel: impl Channel,
        speed: AutoGuideSpeed,
    ) -> SynScanResult<()> {
        self.port
            .send_cmd_bytes(SET_AUTOGUIDE_SPEED, channel, &[speed.comm_byte()])
    }
}
