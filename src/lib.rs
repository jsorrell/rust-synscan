mod motor_controller;
mod port;

pub mod util {
    mod bichannel_value;
    mod result;

    pub use bichannel_value::*;
    pub use result::*;
}

pub use motor_controller::*;
pub use port::channels::*;
pub use port::SerialPort;

#[cfg(feature = "serialport")]
pub use port::serialport;
