pub trait Channel {
    fn get_byte(&self) -> u8;
}

/* Channel */
#[derive(Clone, Copy)]
pub enum MultiChannel {
    Both,
}

impl Channel for MultiChannel {
    fn get_byte(&self) -> u8 {
        b'3'
    }
}

/* SingleChannel */
#[derive(Clone, Copy)]
pub enum SingleChannel {
    Channel1,
    Channel2,
}

impl SingleChannel {
    pub const VALUES: [SingleChannel; 2] = [SingleChannel::Channel1, SingleChannel::Channel2];
}

impl Channel for SingleChannel {
    fn get_byte(&self) -> u8 {
        match self {
            SingleChannel::Channel1 => b'1',
            SingleChannel::Channel2 => b'2',
        }
    }
}
