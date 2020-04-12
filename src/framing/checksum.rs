/// A type used for incrementally calculating u-blox protocol message
/// checksums.
///
/// # Specification
///
/// From UBX-13003221-R18:
///
/// > The checksum algorithm used is the 8-Bit Fletcher Algorithm,
/// which is used in the TCP standard (RFC 1145)
///
/// # Example
///
/// ```
/// # use ublox::framing::Checksum;
/// let bytes = [1, 2, 3, 4];
/// let mut cksum = Checksum::new();
/// for b in &bytes {
///     cksum.push(*b);
/// }
/// let (ck_a, ck_b) = cksum.take();
/// ```
#[derive(Debug, Default, Clone, Copy)]
pub struct Checksum(Option<(u8, u8)>);

impl Checksum {
    /// Returns a new instance of `Self`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns self initialized with the first byte.
    ///
    /// As this initializes `self`, you must not call `push()` with
    /// this same byte again, else the calculated checksum will be
    /// incorrect.
    pub fn with(input: u8) -> Self {
        let mut s = Self::default();
        s.push(input);
        s
    }

    /// Update the running checksum with a received byte.
    #[inline]
    pub fn push(&mut self, input: u8) -> u8 {
        let (ck_a, ck_b) = self.0.get_or_insert((0, 0));
        *ck_a = ck_a.wrapping_add(input);
        *ck_b = ck_b.wrapping_add(*ck_a);
        input
    }

    /// Returns the running checksum, `(ck_a, ck_b)`, and resets `self` to default state.
    pub fn take(&mut self) -> (u8, u8) {
        self.0.take().unwrap_or((0, 0))
    }
}
