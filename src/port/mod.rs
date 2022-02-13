pub mod commands;
mod serial_port;
mod synscan_port;

pub mod channels;
#[cfg(test)]
pub mod tests;

mod impls {
    #[cfg(test)]
    pub mod mock;
    #[cfg(feature = "serialport")]
    pub mod serialport;
}

pub use channels::*;
pub use serial_port::*;
pub use impls::*;
