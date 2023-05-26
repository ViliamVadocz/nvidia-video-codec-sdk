/// Defines `ENCODE_API`, which is a lazy static of [`EncodeAPI`].
pub mod api;
/// Defines traits and types for dealing with input and output buffers.
pub mod buffer;
/// The [`Encoder`] is the main entrypoint for the Encoder API.
///
/// The [`Encoder`] provides a slightly higher-level abstraction over the
/// encoder API. This module also defines builders for some of the parameter
/// structs used by the interface.
pub mod encoder;
/// Defines a wrapper around
/// [`NVENCSTATUS`](crate::sys::nvEncodeAPI::NVENCSTATUS) to provide ergonomic
/// error handling.
pub mod result;

pub mod session;

// pub use api::{EncodeAPI, ENCODE_API};
// pub use buffer::{Bitstream, Buffer, EncoderInput, EncoderOutput,
// RegisteredResource}; pub use encoder::Encoder;
// pub use result::EncodeError;
// pub use session::Session;
