use std::ffi::c_void;

use super::api::EncodeAPI;

pub struct Encoder {
    encoder: *mut c_void,
    encode_api: EncodeAPI,
}

impl Encoder {
    pub fn get_supported_input_formats(
        &mut self,
        encode_guid: GUID,
    ) -> EncodeResult<Vec<NV_ENC_BUFFER_FORMAT>> {
        // Query the number of supported input formats.
        let mut format_count = 0;
        unsafe {
            (self.encode_api.get_input_format_count)(self.ptr, encode_guid, &mut format_count)
        }
        .result()?;
        // Get the supported input formats.
        let mut supported_input_formats =
            vec![NV_ENC_BUFFER_FORMAT::NV_ENC_BUFFER_FORMAT_UNDEFINED; format_count as usize];
        let mut actual_count: u32 = 0;
        unsafe {
            (self.encode_api.get_input_formats)(
                self.ptr,
                encode_guid,
                supported_input_formats.as_mut_ptr(),
                format_count,
                &mut actual_count,
            )
        }
        .result()?;
        supported_input_formats.truncate(actual_count as usize);
        Ok(supported_input_formats)
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
