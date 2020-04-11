/// A type used for incrementally calculating u-blox protocol message
/// checksums.
#[derive(Debug, Default, Clone, Copy)]
pub struct Checksum(Option<(u8, u8)>);

impl Checksum {
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
