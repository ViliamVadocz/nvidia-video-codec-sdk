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

pub trait EncoderInput {
    fn handle(&mut self) -> *mut c_void;
}

pub trait EncoderOutput {
    fn handle(&mut self) -> *mut c_void;
}

pub struct Buffer<'a> {
    pub(crate) ptr: *mut c_void,
    encoder: &'a Encoder,
}

impl Session {
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
        .result()?;
        Ok(Buffer {
            ptr: create_input_buffer_params.inputBuffer,
            encoder: &self.encoder,
        })
    }
}

impl<'a> Buffer<'a> {
    pub fn lock_and_write(&mut self, do_not_wait: bool, data: &[u8]) -> Result<(), EncodeError> {
        let mut lock_input_buffer_params = NV_ENC_LOCK_INPUT_BUFFER {
            version: NV_ENC_LOCK_INPUT_BUFFER_VER,
            inputBuffer: self.ptr,
            ..Default::default()
        };
        if do_not_wait {
            lock_input_buffer_params.set_doNotWait(1);
        }
        unsafe { (ENCODE_API.lock_input_buffer)(self.encoder.ptr, &mut lock_input_buffer_params) }
            .result()?;

        let data_ptr = lock_input_buffer_params.bufferDataPtr;
        // TODO: Find out if pitch is needed and how to use it.
        // let pitch = lock_input_buffer_params.pitch;

        unsafe { data.as_ptr().copy_to(data_ptr.cast::<u8>(), data.len()) };

        unsafe { (ENCODE_API.unlock_input_buffer)(self.encoder.ptr, self.ptr) }.result()
    }
}

impl Drop for Buffer<'_> {
    fn drop(&mut self) {
        unsafe { (ENCODE_API.destroy_input_buffer)(self.encoder.ptr, self.ptr) }
            .result()
            .unwrap();
    }
}

impl EncoderInput for Buffer<'_> {
    fn handle(&mut self) -> *mut c_void {
        self.ptr
    }
}

pub struct Bitstream<'a> {
    pub(crate) ptr: *mut c_void,
    encoder: &'a Encoder,
}

impl Session {
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
        .result()?;
        Ok(Bitstream {
            ptr: create_bitstream_buffer_params.bitstreamBuffer,
            encoder: &self.encoder,
        })
    }
}

impl<'a> Bitstream<'a> {
    pub fn lock_and_read(&mut self) -> Result<&[u8], EncodeError> {
        // Lock bitstream.
        let mut lock_bitstream_buffer_params = NV_ENC_LOCK_BITSTREAM {
            version: NV_ENC_LOCK_BITSTREAM_VER,
            outputBitstream: self.ptr,
            ..Default::default()
        };
        unsafe { (ENCODE_API.lock_bitstream)(self.encoder.ptr, &mut lock_bitstream_buffer_params) }
            .result()?;

        // Get data.
        let data_ptr = lock_bitstream_buffer_params.bitstreamBufferPtr;
        let data_size = lock_bitstream_buffer_params.bitstreamSizeInBytes as usize;
        let data = unsafe { std::slice::from_raw_parts_mut(data_ptr.cast::<u8>(), data_size) };

        // Unlock bitstream.
        unsafe { (ENCODE_API.unlock_bitstream)(self.encoder.ptr, self.ptr) }.result()?;

        Ok(data)
    }
}

impl Drop for Bitstream<'_> {
    fn drop(&mut self) {
        unsafe { (ENCODE_API.destroy_bitstream_buffer)(self.encoder.ptr, self.ptr) }
            .result()
            .unwrap();
    }
}

impl EncoderOutput for Bitstream<'_> {
    fn handle(&mut self) -> *mut c_void {
        self.ptr
    }
}

pub struct MappedResource<'a> {
    pub(crate) reg_ptr: *mut c_void,
    pub(crate) map_ptr: *mut c_void,
    encoder: &'a Encoder,
}

impl Session {
    pub fn register_and_map_input_resource(
        &self,
        mut register_resource_params: NV_ENC_REGISTER_RESOURCE,
    ) -> Result<(MappedResource, NV_ENC_BUFFER_FORMAT), EncodeError> {
        // Currently it looks like
        assert_eq!(
            register_resource_params.bufferUsage,
            NV_ENC_BUFFER_USAGE::NV_ENC_INPUT_IMAGE
        );

        // Register resource.
        unsafe { (ENCODE_API.register_resource)(self.encoder.ptr, &mut register_resource_params) }
            .result()?;
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
        .result()?;

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

impl NV_ENC_REGISTER_RESOURCE {
    /// Create a `NV_ENC_REGISTER_RESOURCE`.
    ///
    /// # Arguments
    ///
    /// - `resource_type` - Specifies the type of resource to be registered.
    ///   Supported values are:
    ///   - [`NV_ENC_INPUT_RESOURCE_TYPE::NV_ENC_INPUT_RESOURCE_TYPE_DIRECTX`],
    ///   - [`NV_ENC_INPUT_RESOURCE_TYPE::NV_ENC_INPUT_RESOURCE_TYPE_CUDADEVICEPTR`],
    ///   - [`NV_ENC_INPUT_RESOURCE_TYPE::NV_ENC_INPUT_RESOURCE_TYPE_OPENGL_TEX`]
    ///
    /// - `width` - Input frame width.
    /// - `height` - Input frame height.
    /// - `resource_to_register` - Handle to the resource that is being
    ///   registered. In the case of
    ///   [`NV_ENC_INPUT_RESOURCE_TYPE::NV_ENC_INPUT_RESOURCE_TYPE_CUDADEVICEPTR`],
    ///   this should be a `CUdeviceptr` which you can get from
    ///   `cuExternalMemoryGetMappedBuffer`.
    ///
    /// - `buffer_format` - Buffer format of resource to be registered.
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

impl Drop for MappedResource<'_> {
    fn drop(&mut self) {
        // Unmapping resource.
        unsafe { (ENCODE_API.unmap_input_resource)(self.encoder.ptr, self.map_ptr) }
            .result()
            .unwrap();
        // Unregister resource.
        unsafe { (ENCODE_API.unregister_resource)(self.encoder.ptr, self.reg_ptr) }
            .result()
            .unwrap();
    }
}

impl EncoderInput for MappedResource<'_> {
    fn handle(&mut self) -> *mut c_void {
        self.map_ptr
    }
}
