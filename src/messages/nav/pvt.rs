use crate::messages::{primitive::*, Message};
use bitfield::bitfield;

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
    valid: Valid,

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
    flags: Flags,

    /// Additional flags (see graphic below)
    ///
    /// ### Unit
    /// -
    flags2: Flags2,

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

bitfield! {
    /// Bitfield `valid`.
    #[derive(Clone, Copy, Eq, PartialEq)]
    pub struct Valid(X1);
    impl Debug;
    /// valid magnetic declination
    pub validMag, _: 3;
    /// UTC time of day has been fully resolved (no seconds
    /// uncertainty). Cannot be used to check if time is completely
    /// solved.
    pub fullyResolved, _: 2;
    /// valid UTC time of day (see Time Validity section for details)
    pub validTime, _: 1;
    /// valid UTC Date (see Time Validity section for details)
    pub validDate, _: 0;

}

bitfield! {
    /// Bitfield `flags`.
    #[derive(Clone, Copy, Eq, PartialEq)]
    pub struct Flags(X1);
    impl Debug;
    /// Carrier phase range solution status
    ///
    /// 0: no carrier phase range solution
    /// 1: carrier phase range solution with floating ambiguities
    /// 2: carrier phase range solution with fixed ambiguities (not supported in protocol versions less than 20)
    pub carrSoln, _: 7, 6;
    /// heading of vehicle is valid, only set if the receiver is in
    /// sensor fusion mode
    pub headVehValid, _: 5;
    /// Undocumented
    pub psmState, _: 4, 2;
    /// differential corrections were applied
    pub diffSoln, _: 1;
    /// valid fix (i.e within DOP & accuracy masks)
    pub gnssFixOK, _: 0;
}

bitfield! {
    /// Bitfield `flags2`.
    #[derive(Clone, Copy, Eq, PartialEq)]
    pub struct Flags2(X1);
    impl Debug;
    /// information about UTC Date and Time of Day validity
    /// confirmation is available (see Time Validity section for
    /// details).
    ///
    /// This flag is only supported in Protocol Versions 19.00, 19.10,
    /// 20.10, 20.20, 20.30, 22.00, 23.00, 23.01, 27 and 28.
    pub confirmedAvai, _: 7;
    /// UTC Date validity could be confirmed (see Time Validity
    /// section for details)
    pub confirmedDate, _: 6;
    /// UTC Time of Day could be confirmed (see Time Validity section
    /// for details)
    pub confirmedTime, _: 5;
}

impl Message for Pvt {
    const CLASS: u8 = 0x01;
    const ID: u8 = 0x07;
    const LEN: usize = 92;

    fn serialize<B: bytes::BufMut>(&self, dst: &mut B) -> Result<(), ()> {
        if dst.remaining_mut() < Self::LEN {
            return Err(());
        }

        let &Self {
            TOW,
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
            flags2,
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
        } = self;

        dst.put_u32_le(TOW);
        dst.put_u16_le(year);
        dst.put_u8(month);
        dst.put_u8(day);
        dst.put_u8(hour);
        dst.put_u8(min);
        dst.put_u8(sec);
        dst.put_u8(valid.0);
        dst.put_u32_le(tAcc);
        dst.put_i32_le(nano);
        dst.put_u8(fxType);
        dst.put_u8(flags.0);
        dst.put_u8(flags2.0);
        dst.put_u8(numSV);
        dst.put_i32_le(lon);
        dst.put_i32_le(lat);
        dst.put_i32_le(height);
        dst.put_i32_le(hMSL);
        dst.put_u32_le(hAcc);
        dst.put_u32_le(vAcc);
        dst.put_i32_le(velN);
        dst.put_i32_le(velE);
        dst.put_i32_le(velD);
        dst.put_i32_le(gSpeed);
        dst.put_i32_le(headMot);
        dst.put_u32_le(sAcc);
        dst.put_u32_le(headAcc);
        dst.put_u16_le(pDOP);
        dst.put_u8(flags3);
        // reserved1
        dst.put_slice([0_u8; 5].as_ref());
        dst.put_i32_le(headVeh);
        dst.put_i16_le(magDec);
        dst.put_u16_le(macAcc);

        Ok(())
    }

    fn deserialize<B: bytes::Buf>(src: &mut B) -> Result<Self, ()> {
        if src.remaining() < Self::LEN {
            return Err(());
        }

        let TOW = src.get_u32_le();
        let year = src.get_u16_le();
        let month = src.get_u8();
        let day = src.get_u8();
        let hour = src.get_u8();
        let min = src.get_u8();
        let sec = src.get_u8();
        let valid = Valid(src.get_u8());
        let tAcc = src.get_u32_le();
        let nano = src.get_i32_le();
        let fxType = src.get_u8();
        let flags = Flags(src.get_u8());
        let flags2 = Flags2(src.get_u8());
        let numSV = src.get_u8();
        let lon = src.get_i32_le();
        let lat = src.get_i32_le();
        let height = src.get_i32_le();
        let hMSL = src.get_i32_le();
        let hAcc = src.get_u32_le();
        let vAcc = src.get_u32_le();
        let velN = src.get_i32_le();
        let velE = src.get_i32_le();
        let velD = src.get_i32_le();
        let gSpeed = src.get_i32_le();
        let headMot = src.get_i32_le();
        let sAcc = src.get_u32_le();
        let headAcc = src.get_u32_le();
        let pDOP = src.get_u16_le();
        let flags3 = src.get_u8();
        // reserved1
        src.advance(5);
        let headVeh = src.get_i32_le();
        let magDec = src.get_i16_le();
        let macAcc = src.get_u16_le();

        Ok(Self {
            TOW,
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
            flags2,
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
    }
}
