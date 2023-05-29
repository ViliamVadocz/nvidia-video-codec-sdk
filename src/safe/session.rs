//! Defines [`Session`] which represents an ongoing encoder session.
//!
//! You need to start a session using [`Encoder::start_session`] before
//! you can initialize input or output buffers, and before you can encode
//! frames. The [`Session`] also stores some information such as the encode
//! width and height so that you do not have to keep repeating it each time.

use super::{api::ENCODE_API, encoder::Encoder, result::EncodeError};
use crate::{
    sys::nvEncodeAPI::{
        NV_ENC_BUFFER_FORMAT,
        NV_ENC_PIC_PARAMS,
        NV_ENC_PIC_PARAMS_VER,
        NV_ENC_PIC_STRUCT,
    },
    EncoderInput,
    EncoderOutput,
};

/// An encoding session to create input/output buffers and encode frames.
///
/// You need to call [`Encoder::start_session`] before you can
/// encode frames using the session. On drop, the session will automatically
/// send an empty EOS frame to flush the encoder.
#[derive(Debug)]
pub struct Session {
    pub(crate) encoder: Encoder,
    pub(crate) width: u32,
    pub(crate) height: u32,
    pub(crate) buffer_format: NV_ENC_BUFFER_FORMAT,
}

impl Session {
    /// Get the encoder used for this session.
    ///
    /// This might be useful if you want to use some of
    /// the functions on [`Encoder`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use cudarc::driver::CudaDevice;
    /// # use nvidia_video_codec_sdk::{
    /// #     sys::nvEncodeAPI::{
    /// #         NV_ENC_BUFFER_FORMAT::NV_ENC_BUFFER_FORMAT_ARGB,
    /// #         NV_ENC_CODEC_H264_GUID,
    /// #         NV_ENC_INITIALIZE_PARAMS,
    /// #     },
    /// #     Encoder,
    /// # };
    /// //* Create encoder. *//
    /// # let cuda_device = CudaDevice::new(0).unwrap();
    /// # let encoder = Encoder::initialize_with_cuda(cuda_device).unwrap();
    ///
    /// //* Set `encode_guid` and check that H.264 encoding is supported. *//
    /// # let encode_guid = NV_ENC_CODEC_H264_GUID;
    /// # let encode_guids = encoder.get_encode_guids().unwrap();
    /// # assert!(encode_guids.contains(&encode_guid));
    ///
    /// let session = encoder
    ///     .start_session(
    ///         NV_ENC_BUFFER_FORMAT_ARGB,
    ///         NV_ENC_INITIALIZE_PARAMS::new(encode_guid, 1920, 1080),
    ///     )
    ///     .unwrap();
    /// // We can still use the encoder like this:
    /// let _input_formats = session
    ///     .get_encoder()
    ///     .get_supported_input_formats(encode_guid);
    /// ```
    #[must_use]
    pub fn get_encoder(&self) -> &Encoder {
        &self.encoder
    }

    /// Encode a frame.
    ///
    /// See [NVIDIA docs](https://docs.nvidia.com/video-technologies/video-codec-sdk/12.0/nvenc-video-encoder-api-prog-guide/index.html#submitting-input-frame-for-encoding).
    ///
    /// # Errors
    ///
    /// Could error if the encode picture parameters were invalid or otherwise
    /// incorrect, or if we run out memory.
    ///
    /// There are two recoverable errors:
    /// - If this returns [`EncodeError::EncoderBusy`] then you should retry
    ///   after a few milliseconds.
    /// - If this returns [`EncodeError::NeedMoreInput`], the client should not
    ///   lock the output bitstream yet. They should continue encoding until
    ///   this function returns `Ok`, and then lock the bitstreams in the order
    ///   in which they were originally used.
    ///
    /// # Examples
    ///
    /// ```
    /// # use cudarc::driver::CudaDevice;
    /// # use nvidia_video_codec_sdk::{
    /// #     sys::nvEncodeAPI::{
    /// #         NV_ENC_BUFFER_FORMAT::NV_ENC_BUFFER_FORMAT_ARGB,
    /// #         NV_ENC_CODEC_H264_GUID,
    /// #         NV_ENC_INITIALIZE_PARAMS,
    /// #         NV_ENC_PIC_PARAMS,
    /// #         NV_ENC_PIC_STRUCT,
    /// #     },
    /// #     Encoder,
    /// # };
    /// # const WIDTH: u32 = 1920;
    /// # const HEIGHT: u32 = 1080;
    /// # const DATA_LEN: usize = (WIDTH * HEIGHT * 4) as usize;
    /// //* Create encoder. *//
    /// # let cuda_device = CudaDevice::new(0).unwrap();
    /// # let encoder = Encoder::initialize_with_cuda(cuda_device).unwrap();
    ///
    /// //* Set `encode_guid` and `buffer_format`, and check that H.264 encoding and the ARGB format are supported. *//
    /// # let encode_guid = NV_ENC_CODEC_H264_GUID;
    /// # let encode_guids = encoder.get_encode_guids().unwrap();
    /// # assert!(encode_guids.contains(&encode_guid));
    /// # let buffer_format = NV_ENC_BUFFER_FORMAT_ARGB;
    /// # let input_formats = encoder.get_supported_input_formats(encode_guid).unwrap();
    /// # assert!(input_formats.contains(&buffer_format));
    ///
    /// // Begin encoder session.
    /// let session = encoder
    ///     .start_session(
    ///         buffer_format,
    ///         NV_ENC_INITIALIZE_PARAMS::new(encode_guid, WIDTH, HEIGHT)
    ///             .display_aspect_ratio(16, 9)
    ///             .framerate(30, 1)
    ///             .enable_picture_type_decision(),
    ///     )
    ///     .unwrap();
    ///
    /// //* Create input and output buffers. *//
    /// # let mut input_buffer = session
    /// #     .create_input_buffer()
    /// #     .unwrap();
    /// # let mut output_bitstream = session.create_output_bitstream().unwrap();
    ///
    /// // Encode frame.
    /// unsafe { input_buffer.lock().unwrap().write(&[0; DATA_LEN]) };
    /// session
    ///     .encode_picture(
    ///         &mut input_buffer,
    ///         &mut output_bitstream,
    ///     )
    ///     .unwrap();
    /// # // TODO: check that output is correct.
    /// let _data = output_bitstream.lock().unwrap().data();
    /// ```
    pub fn encode_picture<I: EncoderInput, O: EncoderOutput>(
        &self,
        input_buffer: &mut I,
        output_bitstream: &mut O,
    ) -> Result<(), EncodeError> {
        let mut encode_pic_params = NV_ENC_PIC_PARAMS {
            version: NV_ENC_PIC_PARAMS_VER,
            inputWidth: self.width,
            inputHeight: self.height,
            inputPitch: self.width, // FIXME: Does pitch need to be configurable here?
            inputBuffer: input_buffer.handle(),
            outputBitstream: output_bitstream.handle(),
            bufferFmt: self.buffer_format,
            pictureStruct: NV_ENC_PIC_STRUCT::NV_ENC_PIC_STRUCT_FRAME,
            ..Default::default()
        };
        unsafe { (ENCODE_API.encode_picture)(self.encoder.ptr, &mut encode_pic_params) }
            .result(&self.encoder)
    }

    /// Send an EOS notifications to flush the encoder.
    ///
    /// This function is called automatically on drop, but if you wish to
    /// get the data after flushing, you should call this function yourself.
    ///
    /// # Errors
    ///
    /// Could error if we run out of memory.
    ///
    /// If this returns [`EncodeError::EncoderBusy`] then you should retry after
    /// a few milliseconds.
    pub fn end_of_stream(&self) -> Result<(), EncodeError> {
        let mut encode_pic_params = NV_ENC_PIC_PARAMS::end_of_stream();
        unsafe { (ENCODE_API.encode_picture)(self.encoder.ptr, &mut encode_pic_params) }
            .result(&self.encoder)
    }
}

/// Send an EOS notifications on drop to flush the encoder.
impl Drop for Session {
    fn drop(&mut self) {
        self.end_of_stream()
            .expect("The encoder should not be busy.");
    }
}
