//! Bindings for the [NVIDIA Video Codec SDK](https://developer.nvidia.com/video-codec-sdk).
//!
//! The raw bindings can be found in [`sys`].
//! Parts of the API have been wrapped in [`safe`].
//!
//! Feel free to contribute!
#![warn(missing_docs)]
#![warn(clippy::pedantic)]
#![warn(clippy::unwrap_used)]

/// Safe wrapper around the raw bindings.
///
/// Largely unfinished, so you might still have to dip into
/// [`sys`](crate::sys) for the missing functionality.
pub mod safe;

/// Auto-generated bindings to NVIDIA Video Codec SDK.
///
/// The bindings were generated using [bindgen](https://github.com/rust-lang/rust-bindgen)
/// using the scripts `sys/linux_sys/bindgen.sh` and
/// `sys/windows_sys/bindgen.ps1` for the respective operating system.
pub mod sys;

#[macro_use]
extern crate lazy_static;
