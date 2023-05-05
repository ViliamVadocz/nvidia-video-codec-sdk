mod version;

#[cfg(target_os = "linux")]
mod linux_sys;
#[cfg(target_os = "linux")]
pub use linux_sys::*;
