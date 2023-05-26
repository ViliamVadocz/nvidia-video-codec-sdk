//! Bindings for the [NVIDIA Video Codec SDK](https://developer.nvidia.com/video-codec-sdk).
//!
//! The raw bindings can be found in [`sys`].
//! Parts of the API have been wrapped in [`safe`].
//!
//! Feel free to contribute!

#![warn(missing_docs)]
#![warn(clippy::pedantic)]
#![warn(clippy::unwrap_used)]
#![warn(missing_debug_implementations)]
#![warn(missing_copy_implementations)]

pub mod safe;
pub mod sys;

#[macro_use]
extern crate lazy_static;

pub use safe::*;
