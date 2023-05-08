use std::ffi::c_void;

use super::api::EncodeAPI;

pub struct Encoder {
    encoder: *mut c_void,
    encode_api: EncodeAPI,
}

impl Encoder {
    pub(crate) fn new(encoder: *mut c_void, encode_api: EncodeAPI) -> Self {
        Self {
            encoder,
            encode_api,
        }
    }
}

impl Drop for Encoder {
    fn drop(&mut self) {
        unsafe { (self.encode_api.destroy_encoder)(self.encoder) }
            .result()
            .unwrap();
    }
}
