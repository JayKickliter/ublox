use crate::messages::primitive::*;
use nom::{do_parse, le_i16, le_i32, le_u16, le_u32, le_u8, named_attr, take};

/// This message combines position, velocity and time solution,
/// including accuracy figures. Note that during a leap second there
/// may be more or less than 60 seconds in a minute.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Pvt {
    /// GPS time of week of the navigation epoch.
    /// See the description of iTOW for details.
    ///
    /// ### Unit
    /// ms
    TOW: U4,

    /// Year  { UTC)
    ///
    /// ### Unit
    /// y
    year: U2,

    /// Month, range 1..12 {UTC)
    ///
    /// ### Unit
    /// month
    month: U1,

    /// Day of month,range 1..31{UTC)
    ///
    /// ### Unit
    /// d
    day: U1,

    /// Hour of day,range 0..23 {UTC)
    ///
    /// ### Unit
    /// h
    hour: U1,

    /// Minute of hour,range 0..59 {UTC)
    ///
    /// ### Unit
    /// min
    min: U1,

    /// Seconds of minute,range 0..60 { UTC)
    ///
    /// ### Unit
    /// s
    sec: U1,

    /// Validity flags (see graphic below)
    ///
    /// ### Unit
    /// -
    valid: X1,

    /// Time accuracy estimate {UTC)
    ///
    /// ### Unit
    /// ns
    tAcc: U4,

    /// Fraction of second, range -1e9 ..1e9 {UTC)
    ///
    /// ### Unit
    /// ns
    nano: I4,

    /// GNSSfix Type: 0: no fix
    /// 1:dead reckoning only
    /// 2: 2 0-fix
    /// 3: 30-fix
    /// 4: GNSS + dead reckoning combined 5: time only fix"
    fxType: U1,

    /// Fix status flags
    ///
    /// ### Unit
    /// -
    flags: X1,

    /// Additional flags (see graphic below)
    ///
    /// ### Unit
    /// -
    fags2: X1,

    /// Number of satellites used in Nav Solution
    ///
    /// ### Unit
    /// -
    numSV: U1,

    /// Longitude
    ///
    /// ### Unit
    /// deg
    lon: I4,

    /// Latitude
    ///
    /// ### Unit
    /// deg
    lat: I4,

    /// Height above ellipsoid
    ///
    /// ### Unit
    /// mm
    height: I4,

    /// Height above mean sea level
    ///
    /// ### Unit
    /// mm
    hMSL: I4,

    /// Horizontal accuracy estimate
    ///
    /// ### Unit
    /// mm
    hAcc: U4,

    /// Vertical accuracy estimate
    ///
    /// ### Unit
    /// mm
    vAcc: U4,

    /// NEDnorth velocity
    ///
    /// ### Unit
    /// mm/s
    velN: I4,

    /// NEDeast velocity
    ///
    /// ### Unit
    /// mm/s
    velE: I4,

    /// NEDdown velocity
    ///
    /// ### Unit
    /// mm/s
    velD: I4,

    /// Ground Speed (2-D)
    ///
    /// ### Unit
    /// mm/s
    gSpeed: I4,

    /// Heading of motion (2-D)
    ///
    /// ### Unit
    /// deg
    headMot: I4,

    /// Speed accuracy estimate
    ///
    /// ### Unit
    /// mm/s
    sAcc: U4,

    /// Heading accuracy estimate {both motion and vehicle)
    ///
    /// ### Unit
    /// deg
    headAcc: U4,

    /// Position DOP
    ///
    /// ### Unit
    /// -
    pDOP: U2,

    /// Additional flags (see graphic below)
    ///
    /// ### Unit
    /// -
    flags3: X1,

    // Reserved
    // ### Unit
    // -
    // reserved1: [U1; 5],
    /// Heading of vehicle (2-D), this is only valid when headVehValid is set, otherwise the output is set to the heading of motion
    ///
    /// ### Unit
    /// deg
    headVeh: I4,

    /// Magnetic declination. Only supported in ADR 4.10 and later.
    ///
    /// ### Unit
    /// deg
    magDec: I2,

    /// Magnetic declination accuracy. Only supported in ADR 4.10 and later.
    ///
    /// ### Unit
    /// deg
    macAcc: U2,
}

impl Pvt {
    /// NAV-PVT ID.
    pub const ID: u8 = 0x07;

    named_attr!(
        #[doc = "Parses `Self` from provided buffer."],
        pub parse<&[u8], Pvt>,
        do_parse!(TOW: le_u32 >>
                  year: le_u16 >>
                  month: le_u8 >>
                  day: le_u8 >>
                  hour: le_u8 >>
                  min: le_u8 >>
                  sec: le_u8 >>
                  valid: le_u8 >>
                  tAcc: le_u32 >>
                  nano: le_i32 >>
                  fxType: le_u8 >>
                  flags: le_u8 >>
                  fags2: le_u8 >>
                  numSV: le_u8 >>
                  lon: le_i32 >>
                  lat: le_i32 >>
                  height: le_i32 >>
                  hMSL: le_i32 >>
                  hAcc: le_u32 >>
                  vAcc: le_u32 >>
                  velN: le_i32 >>
                  velE: le_i32 >>
                  velD: le_i32 >>
                  gSpeed: le_i32 >>
                  headMot: le_i32 >>
                  sAcc: le_u32 >>
                  headAcc: le_u32 >>
                  pDOP: le_u16 >>
                  flags3: le_u8 >>
                  _reserved1: take!(5) >>
                  headVeh: le_i32 >>
                  magDec: le_i16 >>
                  macAcc: le_u16 >>
                  (Self{TOW,
                        year,
                        month,
                        day,
                        hour,
                        min,
                        sec,
                        valid,
                        tAcc,
                        nano,
                        fxType,
                        flags,
                        fags2,
                        numSV,
                        lon,
                        lat,
                        height,
                        hMSL,
                        hAcc,
                        vAcc,
                        velN,
                        velE,
                        velD,
                        gSpeed,
                        headMot,
                        sAcc,
                        headAcc,
                        pDOP,
                        flags3,
                        headVeh,
                        magDec,
                        macAcc,
                  })
        )
    );
}
