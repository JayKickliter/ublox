//! u-blox protocol \[de\]framing.

mod checksum;
mod deframer;
mod error;
mod frame;

pub use checksum::Checksum;
pub use deframer::{deframe, Deframer};
pub use error::FrameError;
pub use frame::{frame, Frame};

/// TODO: add `std` feature and use `heapless::Vec<u8,
/// heapless::consts::U128>` when not `std` feature is not enabled.
pub type FrameVec = Vec<u8>;
