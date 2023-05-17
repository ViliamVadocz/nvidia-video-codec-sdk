use std::{ffi::c_void, ptr};

use super::{api::ENCODE_API, encoder::Encoder, result::EncodeResult};
use crate::sys::nvEncodeAPI::{
    NV_ENC_BUFFER_FORMAT,
    NV_ENC_BUFFER_USAGE,
    NV_ENC_CREATE_BITSTREAM_BUFFER,
    NV_ENC_CREATE_BITSTREAM_BUFFER_VER,
    NV_ENC_CREATE_INPUT_BUFFER,
    NV_ENC_CREATE_INPUT_BUFFER_VER,
    NV_ENC_LOCK_BITSTREAM,
    NV_ENC_LOCK_BITSTREAM_VER,
    NV_ENC_LOCK_INPUT_BUFFER,
    NV_ENC_LOCK_INPUT_BUFFER_VER,
    NV_ENC_MAP_INPUT_RESOURCE,
    NV_ENC_MAP_INPUT_RESOURCE_VER,
    NV_ENC_REGISTER_RESOURCE,
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

impl Encoder {
    pub fn create_input_buffer(
        &self,
        width: u32,
        height: u32,
        buffer_format: NV_ENC_BUFFER_FORMAT,
    ) -> EncodeResult<Buffer> {
        let mut create_input_buffer_params = NV_ENC_CREATE_INPUT_BUFFER {
            version: NV_ENC_CREATE_INPUT_BUFFER_VER,
            width,
            height,
            bufferFmt: buffer_format,
            inputBuffer: ptr::null_mut(),
            ..Default::default()
        };
        unsafe { (ENCODE_API.create_input_buffer)(self.ptr, &mut create_input_buffer_params) }
            .result()?;
        Ok(Buffer {
            ptr: create_input_buffer_params.inputBuffer,
            encoder: self,
        })
    }
}

impl<'a> Buffer<'a> {
    pub fn lock_and_write(&mut self, do_not_wait: bool, data: &[u8]) -> EncodeResult<()> {
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

impl Encoder {
    pub fn create_output_bitstream(&self) -> EncodeResult<Bitstream> {
        let mut create_bitstream_buffer_params = NV_ENC_CREATE_BITSTREAM_BUFFER {
            version: NV_ENC_CREATE_BITSTREAM_BUFFER_VER,
            bitstreamBuffer: ptr::null_mut(),
            ..Default::default()
        };
        unsafe {
            (ENCODE_API.create_bitstream_buffer)(self.ptr, &mut create_bitstream_buffer_params)
        }
        .result()?;
        Ok(Bitstream {
            ptr: create_bitstream_buffer_params.bitstreamBuffer,
            encoder: self,
        })
    }
}

impl<'a> Bitstream<'a> {
    pub fn lock_and_read(&mut self) -> EncodeResult<&[u8]> {
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

impl Encoder {
    pub fn register_and_map_input_resource(
        &self,
        mut register_resource_params: NV_ENC_REGISTER_RESOURCE,
    ) -> EncodeResult<(MappedResource, NV_ENC_BUFFER_FORMAT)> {
        // Currently it looks like
        assert_eq!(
            register_resource_params.bufferUsage,
            NV_ENC_BUFFER_USAGE::NV_ENC_INPUT_IMAGE
        );

        // Register resource.
        unsafe { (ENCODE_API.register_resource)(self.ptr, &mut register_resource_params) }
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
        unsafe { (ENCODE_API.map_input_resource)(self.ptr, &mut map_input_resource_params) }
            .result()?;

        let mapped_resource = map_input_resource_params.mappedResource;
        let input_buffer_format = map_input_resource_params.mappedBufferFmt;
        Ok((
            MappedResource {
                reg_ptr: registered_resource,
                map_ptr: mapped_resource,
                encoder: self,
            },
            input_buffer_format,
        ))
    }
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
