//! Safe wrapper around the raw bindings.
//!
//! Largely unfinished, so you might still have to dip into
//! [`sys`](crate::sys) for the missing functionality.

pub mod api;
pub mod buffer;
pub mod encoder;
pub mod result;
pub mod session;

pub use api::{EncodeAPI, ENCODE_API};
pub use buffer::{Bitstream, Buffer, EncoderInput, EncoderOutput, RegisteredResource};
pub use encoder::Encoder;
pub use result::{EncodeError, ErrorKind};
pub use session::Session;

pub mod builders;
