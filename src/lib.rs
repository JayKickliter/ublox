#![allow(non_snake_case)]
#![recursion_limit = "128"]
#![deny(missing_docs)]
#![no_std]

//! A collection of types and parsers for u-blox v8 messages.

extern crate alloc;

pub mod framing;
pub mod messages;
