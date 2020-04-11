/// The error type returned by [`Deframer::push()`].
///
/// [`Deframer::push()`]: enum.Deframer.html#method.push
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FrameError {
    /// The payload length parsed out of message is larger than we can
    /// store.
    #[cfg(not(feature = "std"))]
    Size {
        /// Declared message length parsed from byte stream.
        declared: usize,
        /// Payload buffer's capacity.
        capacity: usize,
    },

    /// Checksum mismatch.
    ///
    /// Note that declared or calaculated checksums are *not* included with
    /// the error. This is because the defamer may return this error
    /// after receiving only the first declared checksum byte.
    Checksum,
}
