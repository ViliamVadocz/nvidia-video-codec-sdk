/// Defines [`ENCODE_API`], which is a lazy static of [`EncodeAPI`].
pub mod api;
/// Defines traits and types for dealing with input and output buffers.
pub mod buffer;
/// The [`Encoder`] is the main entrypoint for the Encoder API. It provides a
/// slightly higher-level abstraction over the encoder API. This module also
/// defines builders for some of the parameter structs used by the interface.
pub mod encoder;
/// Defines a wrapper around [`NVENCSTATUS`] to provide
/// ergonomic error handling.
pub mod result;

pub use buffer::{Bitstream, Buffer, EncoderInput, EncoderOutput, MappedResource};
pub use encoder::Encoder;
pub use result::EncodeError;
