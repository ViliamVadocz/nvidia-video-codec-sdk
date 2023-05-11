use std::ffi::c_void;

use super::{api::ENCODE_API, encoder::Encoder, result::EncodeResult};
use crate::sys::nvEncodeAPI::{
    NV_ENC_LOCK_BITSTREAM,
    NV_ENC_LOCK_BITSTREAM_VER,
    NV_ENC_LOCK_INPUT_BUFFER,
    NV_ENC_LOCK_INPUT_BUFFER_VER,
};

// TODO:
// Maybe implement Read/Write for the buffers?

pub struct InputBuffer<'a> {
    pub(crate) ptr: *mut c_void,
    encoder: &'a Encoder,
}

impl<'a> InputBuffer<'a> {
    pub(crate) fn new(ptr: *mut c_void, encoder: &'a Encoder) -> Self {
        Self { ptr, encoder }
    }

    pub fn write(&mut self, do_not_wait: bool, data: &[u8]) -> EncodeResult<()> {
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

impl Drop for InputBuffer<'_> {
    fn drop(&mut self) {
        unsafe { (ENCODE_API.destroy_input_buffer)(self.encoder.ptr, self.ptr) }
            .result()
            .unwrap();
    }
}

pub struct OutputBitstream<'a> {
    pub(crate) ptr: *mut c_void,
    encoder: &'a Encoder,
}

impl<'a> OutputBitstream<'a> {
    pub(crate) fn new(ptr: *mut c_void, encoder: &'a Encoder) -> Self {
        Self { ptr, encoder }
    }

    pub fn read(&mut self) -> EncodeResult<&[u8]> {
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

impl Drop for OutputBitstream<'_> {
    fn drop(&mut self) {
        unsafe { (ENCODE_API.destroy_bitstream_buffer)(self.encoder.ptr, self.ptr) }
            .result()
            .unwrap();
    }
}
