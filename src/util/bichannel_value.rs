/* BiChannelValue */
use crate::port::channels::*;

#[derive(Copy, Clone, Debug)]
pub struct BiChannelValue<T> {
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

impl<T> std::ops::Index<SingleChannel> for BiChannelValue<T> {
    type Output = T;

    fn index(&self, channel: SingleChannel) -> &Self::Output {
        match channel {
            SingleChannel::Channel1 => &self.channel1,
            SingleChannel::Channel2 => &self.channel2,
        }
    }
}

impl<T> std::iter::FromIterator<T> for BiChannelValue<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut stream = iter.into_iter();
        Self {
            channel1: stream
                .next()
                .expect("Not enough values to collect into BiChannelValue"),
            channel2: stream
                .next()
                .expect("Not enough values to collect into BiChannelValue"),
        }
    }
}
