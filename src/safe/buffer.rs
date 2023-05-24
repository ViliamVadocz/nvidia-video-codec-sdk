use std::{ffi::c_void, ptr};

use super::{
    api::ENCODE_API,
    encoder::{Encoder, Session},
    result::EncodeError,
};
use crate::sys::nvEncodeAPI::{
    NV_ENC_BUFFER_FORMAT,
    NV_ENC_BUFFER_USAGE,
    NV_ENC_CREATE_BITSTREAM_BUFFER,
    NV_ENC_CREATE_BITSTREAM_BUFFER_VER,
    NV_ENC_CREATE_INPUT_BUFFER,
    NV_ENC_CREATE_INPUT_BUFFER_VER,
    NV_ENC_INPUT_RESOURCE_TYPE,
    NV_ENC_LOCK_BITSTREAM,
    NV_ENC_LOCK_BITSTREAM_VER,
    NV_ENC_LOCK_INPUT_BUFFER,
    NV_ENC_LOCK_INPUT_BUFFER_VER,
    NV_ENC_MAP_INPUT_RESOURCE,
    NV_ENC_MAP_INPUT_RESOURCE_VER,
    NV_ENC_REGISTER_RESOURCE,
    NV_ENC_REGISTER_RESOURCE_VER,
};

/// If a type implements this trait it means it is a valid input buffer
/// for the encoding API.
pub trait EncoderInput {
    /// Get the handle of the input resource.
    fn handle(&mut self) -> *mut c_void;
}

/// If a type implements this trait it means it is a valid output buffer
/// for the encoding API.
pub trait EncoderOutput {
    /// Get the handle of the output resource.
    fn handle(&mut self) -> *mut c_void;
}

/// Functions for creating input and output buffers.
impl Session {
    /// Create a [`Buffer`].
    ///
    /// See [NVIDIA docs](https://docs.nvidia.com/video-technologies/video-codec-sdk/12.0/nvenc-video-encoder-api-prog-guide/index.html#creating-resources-required-to-hold-inputoutput-data).
    ///
    /// # Errors
    ///
    /// Could error if the `width`, `height`, or `buffer_format` is invalid,
    /// or if we run out of memory.
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
    /// //* Begin encoder session. *//
    /// # let session = encoder
    /// #     .start_session(
    /// #         NV_ENC_INITIALIZE_PARAMS::new(encode_guid, WIDTH, HEIGHT)
    /// #             .display_aspect_ratio(16, 9)
    /// #             .framerate(30, 1)
    /// #             .enable_picture_type_decision(),
    /// #     )
    /// #     .unwrap();
    ///
    /// // Create an input buffer.
    /// let _input_buffer = session
    ///     .create_input_buffer(WIDTH, HEIGHT, buffer_format)
    ///     .unwrap();
    /// ```
    pub fn create_input_buffer(
        &self,
        width: u32,
        height: u32,
        buffer_format: NV_ENC_BUFFER_FORMAT,
    ) -> Result<Buffer, EncodeError> {
        let mut create_input_buffer_params = NV_ENC_CREATE_INPUT_BUFFER {
            version: NV_ENC_CREATE_INPUT_BUFFER_VER,
            width,
            height,
            bufferFmt: buffer_format,
            inputBuffer: ptr::null_mut(),
            ..Default::default()
        };
        unsafe {
            (ENCODE_API.create_input_buffer)(self.encoder.ptr, &mut create_input_buffer_params)
        }
        .result(&self.encoder)?;
        Ok(Buffer {
            ptr: create_input_buffer_params.inputBuffer,
            encoder: &self.encoder,
        })
    }

    /// Create a [`Bitstream`].
    ///
    /// See [NVIDIA docs](https://docs.nvidia.com/video-technologies/video-codec-sdk/12.0/nvenc-video-encoder-api-prog-guide/index.html#creating-resources-required-to-hold-inputoutput-data).
    ///
    /// # Errors
    ///
    /// Could error is we run out of memory.
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
    /// //* Begin encoder session. *//
    /// # let session = encoder
    /// #     .start_session(
    /// #         NV_ENC_INITIALIZE_PARAMS::new(encode_guid, WIDTH, HEIGHT)
    /// #             .display_aspect_ratio(16, 9)
    /// #             .framerate(30, 1)
    /// #             .enable_picture_type_decision(),
    /// #     )
    /// #     .unwrap();
    ///
    /// // Create an output bitstream buffer.
    /// let _output_bitstream = session
    ///     .create_output_bitstream()
    ///     .unwrap();
    /// ```
    pub fn create_output_bitstream(&self) -> Result<Bitstream, EncodeError> {
        let mut create_bitstream_buffer_params = NV_ENC_CREATE_BITSTREAM_BUFFER {
            version: NV_ENC_CREATE_BITSTREAM_BUFFER_VER,
            bitstreamBuffer: ptr::null_mut(),
            ..Default::default()
        };
        unsafe {
            (ENCODE_API.create_bitstream_buffer)(
                self.encoder.ptr,
                &mut create_bitstream_buffer_params,
            )
        }
        .result(&self.encoder)?;
        Ok(Bitstream {
            ptr: create_bitstream_buffer_params.bitstreamBuffer,
            encoder: &self.encoder,
        })
    }

    /// Create a [`MappedResource`].
    ///
    /// See [NVIDIA docs](https://docs.nvidia.com/video-technologies/video-codec-sdk/12.0/nvenc-video-encoder-api-prog-guide/index.html#input-buffers-allocated-externally).
    ///
    /// # Panics
    ///
    /// Panics if the `register_resource_params.bufferUsage` is not
    /// [`NV_ENC_BUFFER_USAGE::NV_ENC_INPUT_IMAGE`].
    ///
    /// # Errors
    ///
    /// Could error if registration or mapping fails,
    /// if the resource is invalid, or if we run out of memory.
    ///
    /// ```
    /// # use std::ffi::c_void;
    /// # use cudarc::driver::{CudaDevice, DevicePtr, CudaSlice};
    /// # use nvidia_video_codec_sdk::{
    /// #     sys::nvEncodeAPI::{
    /// #         NV_ENC_BUFFER_FORMAT::NV_ENC_BUFFER_FORMAT_ARGB,
    /// #         NV_ENC_CODEC_H264_GUID,
    /// #         NV_ENC_INITIALIZE_PARAMS,
    /// #         NV_ENC_INPUT_RESOURCE_TYPE,
    /// #         NV_ENC_PIC_PARAMS,
    /// #         NV_ENC_PIC_STRUCT,
    /// #         NV_ENC_REGISTER_RESOURCE,
    /// #     },
    /// #     Encoder,
    /// # };
    /// # const WIDTH: u32 = 1920;
    /// # const HEIGHT: u32 = 1080;
    /// # const DATA_LEN: usize = (WIDTH * HEIGHT * 4) as usize;
    /// //* Create encoder. *//
    /// # let cuda_device = CudaDevice::new(0).unwrap();
    /// # let encoder = Encoder::initialize_with_cuda(cuda_device.clone()).unwrap();
    ///
    /// //* Set `encode_guid` and `buffer_format`, and check that H.264 encoding and the ARGB format are supported. *//
    /// # let encode_guid = NV_ENC_CODEC_H264_GUID;
    /// # let encode_guids = encoder.get_encode_guids().unwrap();
    /// # assert!(encode_guids.contains(&encode_guid));
    /// # let buffer_format = NV_ENC_BUFFER_FORMAT_ARGB;
    /// # let input_formats = encoder.get_supported_input_formats(encode_guid).unwrap();
    /// # assert!(input_formats.contains(&buffer_format));
    ///
    /// //* Begin encoder session. *//
    /// # let session = encoder
    /// #     .start_session(
    /// #         NV_ENC_INITIALIZE_PARAMS::new(encode_guid, WIDTH, HEIGHT)
    /// #             .display_aspect_ratio(16, 9)
    /// #             .framerate(30, 1)
    /// #             .enable_picture_type_decision(),
    /// #     )
    /// #     .unwrap();
    ///
    /// // Allocate memory with CUDA.
    /// let cuda_slice = cuda_device.alloc_zeros::<u8>(DATA_LEN).unwrap();
    ///
    /// // FIXME: Fails for unknown reason.
    /// // Register and map the resource.
    /// let (_mapped_resource, buf_fmt) = session.register_and_map_input_resource(
    ///     NV_ENC_REGISTER_RESOURCE::new(
    ///         NV_ENC_INPUT_RESOURCE_TYPE::NV_ENC_INPUT_RESOURCE_TYPE_CUDADEVICEPTR,
    ///         WIDTH,
    ///         HEIGHT,
    ///         *cuda_slice.device_ptr() as *mut c_void,
    ///         buffer_format,
    ///     )
    ///     .pitch(WIDTH * 4), // ARGB format has 4 bytes per pixel.
    /// ).unwrap();
    /// assert_eq!(buffer_format, buf_fmt);
    /// ```
    pub fn register_and_map_input_resource(
        &self,
        mut register_resource_params: NV_ENC_REGISTER_RESOURCE,
    ) -> Result<(MappedResource, NV_ENC_BUFFER_FORMAT), EncodeError> {
        // Currently it looks like only input is supported.
        assert_eq!(
            register_resource_params.bufferUsage,
            NV_ENC_BUFFER_USAGE::NV_ENC_INPUT_IMAGE
        );

        // Register resource.
        unsafe { (ENCODE_API.register_resource)(self.encoder.ptr, &mut register_resource_params) }
            .result(&self.encoder)?;
        let registered_resource = register_resource_params.registeredResource;

        // Map resource.
        let mut map_input_resource_params = NV_ENC_MAP_INPUT_RESOURCE {
            version: NV_ENC_MAP_INPUT_RESOURCE_VER,
            registeredResource: registered_resource,
            mappedResource: ptr::null_mut(),
            mappedBufferFmt: NV_ENC_BUFFER_FORMAT::NV_ENC_BUFFER_FORMAT_UNDEFINED,
            ..Default::default()
        };
        unsafe {
            (ENCODE_API.map_input_resource)(self.encoder.ptr, &mut map_input_resource_params)
        }
        .result(&self.encoder)?;

        let mapped_resource = map_input_resource_params.mappedResource;
        let input_buffer_format = map_input_resource_params.mappedBufferFmt;
        Ok((
            MappedResource {
                reg_ptr: registered_resource,
                map_ptr: mapped_resource,
                encoder: &self.encoder,
            },
            input_buffer_format,
        ))
    }
}

/// Abstraction around input buffer allocated using
/// the NVIDIA Video Encoder API.
///
/// The buffer is automatically destroyed when dropped.
#[derive(Debug)]
pub struct Buffer<'a> {
    pub(crate) ptr: *mut c_void,
    encoder: &'a Encoder,
}

impl<'a> Buffer<'a> {
    /// Lock the input buffer.
    ///
    /// On a successful lock you get a [`Lock`] which can be used to write
    /// data to the input buffer. On drop, [`Lock`] will unlock the buffer.
    ///
    /// This function will block until a lock is acquired. For the non-blocking
    /// version see [`Buffer::try_lock`].
    ///
    /// See [NVIDIA docs](https://docs.nvidia.com/video-technologies/video-codec-sdk/12.0/nvenc-video-encoder-api-prog-guide/index.html#input-buffers-allocated-through-nvidia-video-encoder-interface).
    ///
    /// # Errors
    ///
    /// Could error if we run out of memory.
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
    /// //* Set `encode_guid` and `buffer_format`, and check that H.264 encoding and the ARGB format are supported. *//
    /// # let encode_guid = NV_ENC_CODEC_H264_GUID;
    /// # let encode_guids = encoder.get_encode_guids().unwrap();
    /// # assert!(encode_guids.contains(&encode_guid));
    /// # let buffer_format = NV_ENC_BUFFER_FORMAT_ARGB;
    /// # let input_formats = encoder.get_supported_input_formats(encode_guid).unwrap();
    /// # assert!(input_formats.contains(&buffer_format));
    /// //* Begin encoder session. *//
    /// # let session = encoder
    /// #     .start_session(
    /// #         NV_ENC_INITIALIZE_PARAMS::new(encode_guid, WIDTH, HEIGHT)
    /// #             .display_aspect_ratio(16, 9)
    /// #             .framerate(30, 1)
    /// #             .enable_picture_type_decision(),
    /// #     )
    /// #     .unwrap();
    ///
    /// // Create an input buffer.
    /// let mut input_buffer = session
    ///     .create_input_buffer(WIDTH, HEIGHT, buffer_format)
    ///     .unwrap();
    /// unsafe { input_buffer.lock().unwrap().write(&[0; DATA_LEN]) };
    /// ```
    pub fn lock<'b>(&'b self) -> Result<Lock<'b, 'a>, EncodeError> {
        self.lock_inner(true)
    }

    /// Non-blocking version of [`Buffer::lock`]. See it for more info.
    ///
    /// This function will return [`EncodeError::EncoderBusy`] or
    /// [`EncodeError::LockBusy`] if the lock is being used. The NVIDIA
    /// documentation from the header file is unclear about this.
    ///
    /// # Errors
    ///
    /// Could error if we run out of memory.
    ///
    /// If this returns [`EncodeError::EncoderBusy`] or
    /// [`EncodeError::LockBusy`] then that means the lock is still busy and
    /// the client should retry in a few milliseconds.
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
    /// #     EncodeError,
    /// #     Encoder,
    /// # };
    /// # const WIDTH: u32 = 1920;
    /// # const HEIGHT: u32 = 1080;
    /// //* Create encoder. *//
    /// # let cuda_device = CudaDevice::new(0).unwrap();
    /// # let encoder = Encoder::initialize_with_cuda(cuda_device).unwrap();
    /// //* Set `encode_guid` and `buffer_format`, and check that H.264 encoding and the ARGB format are supported. *//
    /// # let encode_guid = NV_ENC_CODEC_H264_GUID;
    /// # let encode_guids = encoder.get_encode_guids().unwrap();
    /// # assert!(encode_guids.contains(&encode_guid));
    /// # let buffer_format = NV_ENC_BUFFER_FORMAT_ARGB;
    /// # let input_formats = encoder.get_supported_input_formats(encode_guid).unwrap();
    /// # assert!(input_formats.contains(&buffer_format));
    /// //* Begin encoder session. *//
    /// # let session = encoder
    /// #     .start_session(
    /// #         NV_ENC_INITIALIZE_PARAMS::new(encode_guid, WIDTH, HEIGHT)
    /// #             .display_aspect_ratio(16, 9)
    /// #             .framerate(30, 1)
    /// #             .enable_picture_type_decision(),
    /// #     )
    /// #     .unwrap();
    ///
    /// // Create an input buffer.
    /// let mut input_buffer = session
    ///     .create_input_buffer(WIDTH, HEIGHT, buffer_format)
    ///     .unwrap();
    ///
    /// let lock1 = input_buffer.lock().unwrap();
    /// let lock2 = input_buffer.try_lock();
    /// // FIXME: Apparently two locks are Ok?
    /// assert_eq!(lock2.unwrap_err(), EncodeError::LockBusy);
    /// drop(lock1)
    /// ```
    pub fn try_lock<'b>(&'b self) -> Result<Lock<'b, 'a>, EncodeError> {
        self.lock_inner(false)
    }

    #[inline]
    fn lock_inner<'b>(&'b self, wait: bool) -> Result<Lock<'b, 'a>, EncodeError> {
        let mut lock_input_buffer_params = NV_ENC_LOCK_INPUT_BUFFER {
            version: NV_ENC_LOCK_INPUT_BUFFER_VER,
            inputBuffer: self.ptr,
            ..Default::default()
        };
        if !wait {
            lock_input_buffer_params.set_doNotWait(1);
        }
        unsafe { (ENCODE_API.lock_input_buffer)(self.encoder.ptr, &mut lock_input_buffer_params) }
            .result(self.encoder)?;

        let data_ptr = lock_input_buffer_params.bufferDataPtr;
        let pitch = lock_input_buffer_params.pitch;

        Ok(Lock {
            buffer: self,
            data_ptr,
            pitch,
        })
    }
}

impl Drop for Buffer<'_> {
    fn drop(&mut self) {
        unsafe { (ENCODE_API.destroy_input_buffer)(self.encoder.ptr, self.ptr) }
            .result(self.encoder)
            .expect("The encoder and buffer pointers should be valid.");
    }
}

impl EncoderInput for Buffer<'_> {
    fn handle(&mut self) -> *mut c_void {
        self.ptr
    }
}

/// An RAII lock on the input buffer.
///
/// This type is created via [`Buffer::lock`] or [`Buffer::try_lock`].
/// The purpose of this type is similar to [`std::sync::MutexGuard`] -
/// it automatically unlocks the buffer then the lock goes out of scope.
#[derive(Debug)]
pub struct Lock<'a, 'b> {
    buffer: &'a Buffer<'b>,
    data_ptr: *mut c_void,
    #[allow(dead_code)]
    pitch: u32,
}

impl Lock<'_, '_> {
    /// Write data to the buffer.
    ///
    /// # Safety
    ///
    /// The size of the data should be less or equal to the size of the buffer.
    /// The size of the buffer depends on the width, height, and buffer format.
    ///
    /// The user should also account for pitch, the data is written
    /// contiguously.
    pub unsafe fn write(&mut self, data: &[u8]) {
        // TODO: Make this safe by doing checks.
        // - Check that length of data fits (depends on format).
        // - Write pitched?
        data.as_ptr()
            .copy_to(self.data_ptr.cast::<u8>(), data.len());
    }
}

impl Drop for Lock<'_, '_> {
    fn drop(&mut self) {
        unsafe { (ENCODE_API.unlock_input_buffer)(self.buffer.encoder.ptr, self.buffer.ptr) }
            .result(self.buffer.encoder)
            .expect("The encoder and buffer pointers should be valid.");
    }
}

/// Abstraction around the output bitstream buffer that
/// is used as the output of the encoding.
///
/// The buffer is automatically destroyed when dropped.
#[derive(Debug)]
pub struct Bitstream<'a> {
    pub(crate) ptr: *mut c_void,
    encoder: &'a Encoder,
}

// TODO: There is a lot of extra data and statistics that we get when we lock
// the bitstream. Maybe expose that somehow? Consider an API similar to `Buffer`
// and `Lock`.
//
// No example because I think I will still change this API.
impl<'a> Bitstream<'a> {
    /// Lock the output bitstream and read from it.
    ///
    /// The function can block until the bitstream is available when `wait` is
    /// set to `true`.
    ///
    /// # Errors
    ///
    /// [`EncodeError::LockBusy`] could be returned if `wait` is set to `false`
    /// and the lock is currently busy. This is a recoverable error and the
    /// client should retry in a few milliseconds.
    pub fn lock_and_read(&mut self, wait: bool) -> Result<&[u8], EncodeError> {
        // Lock bitstream.
        let mut lock_bitstream_buffer_params = NV_ENC_LOCK_BITSTREAM {
            version: NV_ENC_LOCK_BITSTREAM_VER,
            outputBitstream: self.ptr,
            ..Default::default()
        };
        if !wait {
            lock_bitstream_buffer_params.set_doNotWait(1);
        }
        unsafe { (ENCODE_API.lock_bitstream)(self.encoder.ptr, &mut lock_bitstream_buffer_params) }
            .result(self.encoder)?;

        // Get data.
        let data_ptr = lock_bitstream_buffer_params.bitstreamBufferPtr;
        let data_size = lock_bitstream_buffer_params.bitstreamSizeInBytes as usize;
        let data = unsafe { std::slice::from_raw_parts_mut(data_ptr.cast::<u8>(), data_size) };

        // Unlock bitstream.
        unsafe { (ENCODE_API.unlock_bitstream)(self.encoder.ptr, self.ptr) }
            .result(self.encoder)?;

        Ok(data)
    }
}

impl Drop for Bitstream<'_> {
    fn drop(&mut self) {
        unsafe { (ENCODE_API.destroy_bitstream_buffer)(self.encoder.ptr, self.ptr) }
            .result(self.encoder)
            .expect("The encoder and bitstream pointers should be valid.");
    }
}

impl EncoderOutput for Bitstream<'_> {
    fn handle(&mut self) -> *mut c_void {
        self.ptr
    }
}

/// Abstraction for a registered and mapped external resource.
///
/// The Encoder API exposes a way to use input buffers allocated externally,
/// for example through CUDA or OpenGL.
///
/// The buffer is automatically unmapped and unregistered when dropped.
/// The external buffer memory should still be properly destroyed by the client.
#[derive(Debug)]
pub struct MappedResource<'a> {
    pub(crate) reg_ptr: *mut c_void,
    pub(crate) map_ptr: *mut c_void,
    encoder: &'a Encoder,
}

impl NV_ENC_REGISTER_RESOURCE {
    /// Builder for [`NV_ENC_REGISTER_RESOURCE`].
    ///
    /// # Arguments
    ///
    /// * `resource_type` - Specifies the type of resource to be registered.
    ///   Supported values are:
    ///   - [`NV_ENC_INPUT_RESOURCE_TYPE::NV_ENC_INPUT_RESOURCE_TYPE_DIRECTX`],
    ///   - [`NV_ENC_INPUT_RESOURCE_TYPE::NV_ENC_INPUT_RESOURCE_TYPE_CUDADEVICEPTR`],
    ///   - [`NV_ENC_INPUT_RESOURCE_TYPE::NV_ENC_INPUT_RESOURCE_TYPE_OPENGL_TEX`]
    /// * `width` - Input frame width.
    /// * `height` - Input frame height.
    /// * `resource_to_register` - Handle to the resource that is being
    ///   registered. In the case of
    ///   [`NV_ENC_INPUT_RESOURCE_TYPE::NV_ENC_INPUT_RESOURCE_TYPE_CUDADEVICEPTR`],
    ///   this should be a `CUdeviceptr` which you can get from
    ///   `cuExternalMemoryGetMappedBuffer`.
    /// * `buffer_format` - Buffer format of resource to be registered.
    #[must_use]
    pub fn new(
        resource_type: NV_ENC_INPUT_RESOURCE_TYPE,
        width: u32,
        height: u32,
        resource_to_register: *mut c_void,
        buffer_format: NV_ENC_BUFFER_FORMAT,
    ) -> Self {
        NV_ENC_REGISTER_RESOURCE {
            version: NV_ENC_REGISTER_RESOURCE_VER,
            resourceType: resource_type,
            width,
            height,
            pitch: width,
            resourceToRegister: resource_to_register,
            registeredResource: std::ptr::null_mut(),
            bufferFormat: buffer_format,
            ..Default::default()
        }
    }

    /// Set the input buffer pitch.
    ///
    /// - For [`NV_ENC_INPUT_RESOURCE_TYPE::NV_ENC_INPUT_RESOURCE_TYPE_DIRECTX`]
    /// resources, set this to 0.
    /// - For [`NV_ENC_INPUT_RESOURCE_TYPE::NV_ENC_INPUT_RESOURCE_TYPE_CUDADEVICEPTR`]
    /// resources, set this to the pitch as obtained from `cuMemAllocPitch()`,
    /// or to the width in **bytes** (if this resource was created by using
    /// `cuMemAlloc()`). This value must be a multiple of 4.
    /// - For [`NV_ENC_INPUT_RESOURCE_TYPE::NV_ENC_INPUT_RESOURCE_TYPE_OPENGL_TEX`]
    /// resources, set this to the texture width multiplied by the number of
    /// components in the texture format.
    #[must_use]
    pub fn pitch(mut self, pitch: u32) -> Self {
        self.pitch = pitch;
        self
    }

    /// Set the usage of resource to be registered.
    #[must_use]
    pub fn buffer_usage(mut self, buffer_usage: NV_ENC_BUFFER_USAGE) -> Self {
        self.bufferUsage = buffer_usage;
        self
    }

    // TODO: Add other options
}

/// Automatically unmap and unregister the external resource
/// when it goes out of scope.
impl Drop for MappedResource<'_> {
    fn drop(&mut self) {
        // Unmapping resource.
        unsafe { (ENCODE_API.unmap_input_resource)(self.encoder.ptr, self.map_ptr) }
            .result(self.encoder)
            .expect("The encoder pointer and map handle should be valid.");
        // Unregister resource.
        unsafe { (ENCODE_API.unregister_resource)(self.encoder.ptr, self.reg_ptr) }
            .result(self.encoder)
            .expect("The encoder pointer and resource handle should be valid.");
    }
}

impl EncoderInput for MappedResource<'_> {
    fn handle(&mut self) -> *mut c_void {
        self.map_ptr
    }
}
