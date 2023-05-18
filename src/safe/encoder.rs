use std::{
    ffi::{c_void, CStr},
    sync::Arc,
};

use cudarc::driver::CudaDevice;

use super::{
    api::ENCODE_API,
    buffer::{EncoderInput, EncoderOutput},
    result::EncodeResult,
};
use crate::sys::nvEncodeAPI::{
    GUID,
    NV_ENC_BUFFER_FORMAT,
    NV_ENC_CODEC_PIC_PARAMS,
    NV_ENC_CONFIG,
    NV_ENC_CONFIG_VER,
    NV_ENC_INITIALIZE_PARAMS,
    NV_ENC_INITIALIZE_PARAMS_VER,
    NV_ENC_PIC_FLAGS,
    NV_ENC_PIC_PARAMS,
    NV_ENC_PIC_PARAMS_VER,
    NV_ENC_PIC_STRUCT,
    NV_ENC_PRESET_CONFIG,
    NV_ENC_PRESET_CONFIG_VER,
    NV_ENC_TUNING_INFO,
};

type Device = Arc<CudaDevice>;

pub struct Encoder {
    pub(crate) ptr: *mut c_void,
    // Used to make sure that CudaDevice stays alive while the Encoder does
    _device: Device,
}

impl Drop for Encoder {
    fn drop(&mut self) {
        // Destroy encoder when it goes out of scope.
        unsafe { (ENCODE_API.destroy_encoder)(self.ptr) }
            .result()
            .unwrap();
    }
}

impl Encoder {
    pub(crate) fn new(ptr: *mut c_void, device: Device) -> Self {
        Self {
            ptr,
            _device: device,
        }
    }

    #[must_use]
    pub fn get_last_error_string(&self) -> &CStr {
        unsafe { CStr::from_ptr((ENCODE_API.get_last_error_string)(self.ptr)) }
    }

    pub fn get_encode_guids(&self) -> EncodeResult<Vec<GUID>> {
        // Query number of supported encoder codec GUIDs.
        let mut supported_count = 0;
        unsafe { (ENCODE_API.get_encode_guid_count)(self.ptr, &mut supported_count) }.result()?;
        // Get the supported GUIDs.
        let mut encode_guids = vec![GUID::default(); supported_count as usize];
        let mut actual_count: u32 = 0;
        unsafe {
            (ENCODE_API.get_encode_guids)(
                self.ptr,
                encode_guids.as_mut_ptr(),
                supported_count,
                &mut actual_count,
            )
        }
        .result()?;
        encode_guids.truncate(actual_count as usize);
        Ok(encode_guids)
    }

    pub fn get_preset_guids(&self, encode_guid: GUID) -> EncodeResult<Vec<GUID>> {
        // Query the number of preset GUIDS.
        let mut preset_count = 0;
        unsafe { (ENCODE_API.get_encode_preset_count)(self.ptr, encode_guid, &mut preset_count) }
            .result()?;
        // Get the preset GUIDs.
        let mut actual_count: u32 = 0;
        let mut preset_guids = vec![GUID::default(); preset_count as usize];
        unsafe {
            (ENCODE_API.get_encode_preset_guids)(
                self.ptr,
                encode_guid,
                preset_guids.as_mut_ptr(),
                preset_count,
                &mut actual_count,
            )
        }
        .result()?;
        preset_guids.truncate(actual_count as usize);
        Ok(preset_guids)
    }

    pub fn get_profile_guids(&self, encode_guid: GUID) -> EncodeResult<Vec<GUID>> {
        // Query the number of profile GUIDs.
        let mut profile_count = 0;
        unsafe {
            (ENCODE_API.get_encode_profile_guid_count)(self.ptr, encode_guid, &mut profile_count)
        }
        .result()?;
        // Get the profile GUIDs.
        let mut profile_guids = vec![GUID::default(); profile_count as usize];
        let mut actual_count: u32 = 0;
        unsafe {
            (ENCODE_API.get_encode_profile_guids)(
                self.ptr,
                encode_guid,
                profile_guids.as_mut_ptr(),
                profile_count,
                &mut actual_count,
            )
        }
        .result()?;
        profile_guids.truncate(actual_count as usize);
        Ok(profile_guids)
    }

    pub fn get_supported_input_formats(
        &self,
        encode_guid: GUID,
    ) -> EncodeResult<Vec<NV_ENC_BUFFER_FORMAT>> {
        // Query the number of supported input formats.
        let mut format_count = 0;
        unsafe { (ENCODE_API.get_input_format_count)(self.ptr, encode_guid, &mut format_count) }
            .result()?;
        // Get the supported input formats.
        let mut supported_input_formats =
            vec![NV_ENC_BUFFER_FORMAT::NV_ENC_BUFFER_FORMAT_UNDEFINED; format_count as usize];
        let mut actual_count: u32 = 0;
        unsafe {
            (ENCODE_API.get_input_formats)(
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
    }

    pub fn get_preset_config(
        &self,
        encode_guid: GUID,
        preset_guid: GUID,
        tuning_info: NV_ENC_TUNING_INFO,
    ) -> EncodeResult<NV_ENC_PRESET_CONFIG> {
        let mut preset_config = NV_ENC_PRESET_CONFIG {
            version: NV_ENC_PRESET_CONFIG_VER,
            presetCfg: NV_ENC_CONFIG {
                version: NV_ENC_CONFIG_VER,
                ..Default::default()
            },
            ..Default::default()
        };
        unsafe {
            (ENCODE_API.get_encode_preset_config_ex)(
                self.ptr,
                encode_guid,
                preset_guid,
                tuning_info,
                &mut preset_config,
            )
        }
        .result()?;
        Ok(preset_config)
    }

    pub fn initialize_encoder_session(
        &self,
        mut initialize_params: NV_ENC_INITIALIZE_PARAMS,
    ) -> EncodeResult<()> {
        unsafe { (ENCODE_API.initialize_encoder)(self.ptr, &mut initialize_params) }.result()
    }

    pub fn encode_picture(&self, mut encode_pic_params: NV_ENC_PIC_PARAMS) -> EncodeResult<()> {
        unsafe { (ENCODE_API.encode_picture)(self.ptr, &mut encode_pic_params) }.result()
    }
}

// Builder pattern
impl NV_ENC_INITIALIZE_PARAMS {
    #[must_use]
    pub fn new(encode_guid: GUID, width: u32, height: u32) -> Self {
        NV_ENC_INITIALIZE_PARAMS {
            version: NV_ENC_INITIALIZE_PARAMS_VER,
            encodeGUID: encode_guid,
            encodeWidth: width,
            encodeHeight: height,
            ..Default::default()
        }
    }

    #[must_use]
    pub fn preset_guid(mut self, preset_guid: GUID) -> Self {
        self.presetGUID = preset_guid;
        self
    }

    #[must_use]
    pub fn encode_config(mut self, encode_config: &mut NV_ENC_CONFIG) -> Self {
        self.encodeConfig = encode_config;
        self
    }

    #[must_use]
    pub fn display_aspect_ratio(mut self, width: u32, height: u32) -> Self {
        self.darWidth = width;
        self.darHeight = height;
        self
    }

    #[must_use]
    pub fn framerate(mut self, numerator: u32, denominator: u32) -> Self {
        self.frameRateNum = numerator;
        self.frameRateDen = denominator;
        self
    }

    #[must_use]
    pub fn enable_picture_type_decision(mut self) -> Self {
        self.enablePTD = 1;
        self
    }

    // TODO: Add other options
}

// Builder pattern
impl NV_ENC_PIC_PARAMS {
    #[must_use]
    pub fn new<INPUT: EncoderInput, OUTPUT: EncoderOutput>(
        width: u32,
        height: u32,
        input_buffer: &mut INPUT,
        output_bitstream: &mut OUTPUT,
        buffer_format: NV_ENC_BUFFER_FORMAT,
        picture_struct: NV_ENC_PIC_STRUCT,
    ) -> Self {
        NV_ENC_PIC_PARAMS {
            version: NV_ENC_PIC_PARAMS_VER,
            inputWidth: width,
            inputHeight: height,
            inputPitch: width,
            // TODO: Which flag should be used when?
            inputBuffer: input_buffer.handle(),
            outputBitstream: output_bitstream.handle(),
            bufferFmt: buffer_format,
            pictureStruct: picture_struct,
            ..Default::default()
        }
    }

    #[must_use]
    pub fn pitch(mut self, pitch: u32) -> Self {
        self.inputPitch = pitch;
        self
    }

    #[must_use]
    pub fn frame_id(mut self, frame_id: u32) -> Self {
        self.frameIdx = frame_id;
        self
    }

    #[must_use]
    pub fn codec_pic_params(mut self, codec_pic_params: NV_ENC_CODEC_PIC_PARAMS) -> Self {
        self.codecPicParams = codec_pic_params;
        self
    }

    #[must_use]
    pub fn end_of_stream(mut self) -> Self {
        self.encodePicFlags |= NV_ENC_PIC_FLAGS::NV_ENC_PIC_FLAG_EOS as u32;
        self
    }

    // TODO: Add other options
}
