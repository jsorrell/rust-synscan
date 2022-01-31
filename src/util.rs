use std::ops;

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

/* BiChannelValue */
#[derive(Copy, Clone, Hash, Debug)]
pub(crate) struct BiChannelValue<T> {
    channel1: T,
    channel2: T,
}

impl<T> BiChannelValue<T> {
    pub fn new(channel1: T, channel2: T) -> BiChannelValue<T> {
        BiChannelValue { channel1, channel2 }
    }

    #[allow(dead_code)]
    pub fn new_from_fn<F>(mut f: F) -> BiChannelValue<T>
    where
        F: FnMut(SingleChannel) -> T,
    {
        BiChannelValue::new(f(SingleChannel::Channel1), f(SingleChannel::Channel2))
    }

    pub fn new_from_result_fn<F, E>(mut f: F) -> Result<BiChannelValue<T>, E>
    where
        F: FnMut(SingleChannel) -> Result<T, E>,
    {
        Ok(BiChannelValue::new(
            f(SingleChannel::Channel1)?,
            f(SingleChannel::Channel2)?,
        ))
    }
}

impl<T> ops::Index<SingleChannel> for BiChannelValue<T> {
    type Output = T;

    fn index(&self, channel: SingleChannel) -> &Self::Output {
        match channel {
            SingleChannel::Channel1 => &self.channel1,
            SingleChannel::Channel2 => &self.channel2,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum AutoGuideSpeed {
    One,
    ThreeQuarters,
    Half,
    Quarter,
    Eighth,
}

impl AutoGuideSpeed {
    pub fn multiplier(&self) -> f64 {
        match self {
            AutoGuideSpeed::One => 1.0,
            AutoGuideSpeed::ThreeQuarters => 0.75,
            AutoGuideSpeed::Half => 0.5,
            AutoGuideSpeed::Quarter => 0.25,
            AutoGuideSpeed::Eighth => 0.125,
        }
    }

    pub(crate) fn comm_byte(&self) -> u8 {
        match self {
            AutoGuideSpeed::One => b'0',
            AutoGuideSpeed::ThreeQuarters => b'1',
            AutoGuideSpeed::Half => b'2',
            AutoGuideSpeed::Quarter => b'3',
            AutoGuideSpeed::Eighth => b'4',
        }
    }
}
