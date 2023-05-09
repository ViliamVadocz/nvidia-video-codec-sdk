use std::ffi::c_void;

use super::{encoder::Encoder, result::EncodeResult};

pub struct InputBuffer<'a> {
    pub(crate) ptr: *mut c_void,
    encoder: &'a Encoder,
}

impl<'a> InputBuffer<'a> {
    pub(crate) fn new(ptr: *mut c_void, encoder: &'a Encoder) -> Self {
        Self { ptr, encoder }
    }

}

impl Drop for InputBuffer<'_> {
    fn drop(&mut self) {
        unsafe { (self.encoder.encode_api.destroy_input_buffer)(self.encoder.ptr, self.ptr) }
            .result()
            .unwrap();
    }
}
