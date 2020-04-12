use crate::coding::{DecError, DecResult, Decode, Decoder};
use crate::primitive::*;
use nom::{do_parse, le_i16, le_i32, le_i8, le_u32, le_u8, named_attr};

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

impl TimeGps {
    /// NAV-TIMEGPS ID.
    pub const ID: u8 = 0x20;
}

named_attr!(
    #[allow(missing_docs)],
    pub time_gps<&[u8], TimeGps>,
    do_parse!(iTOW: le_u32 >>
              fTOW: le_i32 >>
              week: le_i16 >>
              leapS: le_i8 >>
              valid: le_u8 >>
              tAcc: le_u32 >>
              (TimeGps{iTOW, fTOW, week, leapS, valid, tAcc})
    )
);

impl Decode for TimeGps {
    const MIN_SIZE: usize = 16;
    const MAX_SIZE: usize = 16;
    fn decode(ctx: &mut Decoder) -> DecResult<Self> {
        if (ctx.decode_u2()? != 0x01)
            || (ctx.decode_u2()? != 0x20)
            || ((ctx.decode_u2()? as usize) < Self::MIN_SIZE)
        {
            return Err(DecError);
        }

        Ok(Self {
            iTOW: ctx.decode_u4()?,
            fTOW: ctx.decode_i4()?,
            week: ctx.decode_i2()?,
            leapS: ctx.decode_i2()? as i8,
            valid: ctx.decode_i2()? as u8,
            tAcc: ctx.decode_u4()?,
        })
    }
}