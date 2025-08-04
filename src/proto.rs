#![allow(clippy::all)]
#![allow(nonstandard_style, unused, irrefutable_let_patterns)]
include!(concat!(env!("OUT_DIR"), "/proto.rs"));

use hal::ieee802154;
use core::convert::TryInto;
use paste::paste;
use defmt::Format;

#[derive(Debug,Format)]
pub enum ChannelError {
    InvalidChannel,
}

macro_rules! channel_match {
    ( $expression:expr, { $($num:literal),+ }) => {
        match $expression {
            $( paste! { Channel::[<C $num>] } => paste! { Ok(ieee802154::Channel::[<_ $num>] )},)*
            _ => Err(ChannelError::InvalidChannel),
        }
    };
}


impl TryInto<ieee802154::Channel> for Channel {
    type Error = ChannelError;
    fn try_into(self) -> Result<ieee802154::Channel,Self::Error> {
        channel_match!(self, { 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26 })
    }
}

#[derive(Debug,Format)]
pub enum TxPowerError {
    InvalidTxPower,
}

macro_rules! txpower_match {
    ( $expression:expr, { $( $pattern:tt),* }) => {
        match $expression {
            $(txpower_match!(@proto $pattern) => txpower_match!(@rust $pattern),)*
            _ => Err(TxPowerError::InvalidTxPower),
        }
    };

    (@proto 0) => { TxPower::_0DBm };
    (@proto (+, $num:literal)) => { paste! { TxPower::[<Pos $num DBm>] } };
    (@proto (-, $num:literal)) => { paste! { TxPower::[<Neg $num DBm>] } };

    (@rust 0) => { Ok(ieee802154::TxPower::_0dBm) };
    (@rust (+, $num:literal)) => { paste! { Ok(ieee802154::TxPower::[<Pos $num dBm>]) } };
    (@rust (-, $num:literal)) => { paste! { Ok(ieee802154::TxPower::[<Neg $num dBm>]) } };

}

impl TryInto<ieee802154::TxPower> for TxPower {
    type Error = TxPowerError;
    fn try_into(self) -> Result<ieee802154::TxPower,Self::Error> {
        txpower_match!( self, { 
            (+,8), (+,7), (+,6), (+,5), (+,4), (+,2), 0, 
            (-,4), (-,8), (-,12), (-,16), (-,20), (-,40) })
    }
}
