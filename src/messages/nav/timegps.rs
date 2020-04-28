use crate::messages::{primitive::*, Message};
use bytes::{Buf, BufMut};

/// This message reports the precise GPS time of the most recent
/// navigation solution including validity flags and an accuracy
/// estimate.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TimeGps {
    /// GPS time of week of the navigation epoch.
    ///
    /// ### Unit
    /// millisecond
    pub iTOW: U4,

    /// Fractional part of iTOW (range: +/- 500000).
    ///
    /// The precise GPS time of week in seconds is:
    /// (iTOW * 1e-3) + (fTOW * 1e-9)
    ///
    /// ### Unit
    /// nanosecond
    pub fTOW: I4,

    /// GPS week number of the navigation epoch.
    ///
    /// ### Unit
    /// week
    pub week: I2,

    /// GPS leap seconds (GPS-UTC).
    ///
    /// ### Unit
    /// second
    pub leapS: I1,

    /// Validity Flags.
    pub valid: X1,

    /// Time Accuracy Estimate.
    ///
    /// ### Unit
    /// nanosecond
    pub tAcc: U4,
}

impl Message for TimeGps {
    const CLASS: u8 = 0x01;
    const ID: u8 = 0x20;
    const LEN: usize = 16;

    fn serialize<B: BufMut>(&self, dst: &mut B) -> Result<(), ()> {
        if dst.remaining_mut() < Self::LEN {
            return Err(());
        }

        let &TimeGps {
            iTOW,
            fTOW,
            week,
            leapS,
            valid,
            tAcc,
        } = self;

        dst.put_u32_le(iTOW);
        dst.put_i32_le(fTOW);
        dst.put_i16_le(week);
        dst.put_i8(leapS);
        dst.put_u8(valid);
        dst.put_u32_le(tAcc);

        Ok(())
    }

    fn deserialize<B: Buf>(src: &mut B) -> Result<Self, ()> {
        if src.remaining() < Self::LEN {
            return Err(());
        }

        let iTOW = src.get_u32_le();
        let fTOW = src.get_i32_le();
        let week = src.get_i16_le();
        let leapS = src.get_i8();
        let valid = src.get_u8();
        let tAcc = src.get_u32_le();

        Ok(TimeGps {
            iTOW,
            fTOW,
            week,
            leapS,
            valid,
            tAcc,
        })
    }
}
