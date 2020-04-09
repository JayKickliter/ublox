#![allow(non_snake_case)]
#![deny(missing_docs)]

//! A collection of types and parsers for u-blox v8 messages.

extern crate byteorder;
#[macro_use]
extern crate nom;

#[cfg(not(feature = "std"))]
extern crate heapless;

pub mod coding;
pub mod framing;
pub mod nav;
pub mod primitive;
