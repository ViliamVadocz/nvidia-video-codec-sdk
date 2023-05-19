mod guid;
mod version;

#[allow(warnings)]
#[rustfmt::skip]
#[cfg(target_os = "linux")]
mod linux_sys;
#[cfg(target_os = "linux")]
pub use linux_sys::*;

#[allow(warnings)]
#[rustfmt::skip]
#[cfg(target_os = "windows")]
mod windows_sys;
#[cfg(target_os = "windows")]
pub use windows_sys::*;
