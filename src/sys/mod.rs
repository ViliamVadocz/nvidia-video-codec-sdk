#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(clippy::useless_transmute)]
#![allow(clippy::too_many_arguments)]

mod guid;
mod version;

#[rustfmt::skip]
#[cfg(target_os = "linux")]
mod linux_sys;
#[cfg(target_os = "linux")]
pub use linux_sys::*;

#[rustfmt::skip]
#[cfg(target_os = "windows")]
mod windows_sys;
#[cfg(target_os = "windows")]
pub use windows_sys::*;
