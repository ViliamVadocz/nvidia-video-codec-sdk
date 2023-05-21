use std::{
    ffi::{c_void, CStr},
    mem::MaybeUninit,
    sync::Arc,
};

use cudarc::driver::CudaDevice;

use super::{
    api::ENCODE_API,
    buffer::{EncoderInput, EncoderOutput},
    result::EncodeError,
};
use crate::sys::nvEncodeAPI::{
    GUID,
    NVENCAPI_VERSION,
    NV_ENC_BUFFER_FORMAT,
    NV_ENC_CODEC_PIC_PARAMS,
    NV_ENC_CONFIG,
    NV_ENC_CONFIG_VER,
    NV_ENC_DEVICE_TYPE,
    NV_ENC_INITIALIZE_PARAMS,
    NV_ENC_INITIALIZE_PARAMS_VER,
    NV_ENC_OPEN_ENCODE_SESSION_EX_PARAMS,
    NV_ENC_OPEN_ENCODE_SESSION_EX_PARAMS_VER,
    NV_ENC_PIC_FLAGS,
    NV_ENC_PIC_PARAMS,
    NV_ENC_PIC_PARAMS_VER,
    NV_ENC_PIC_STRUCT,
    NV_ENC_PRESET_CONFIG,
    NV_ENC_PRESET_CONFIG_VER,
    NV_ENC_TUNING_INFO,
};

type Device = Arc<CudaDevice>;

/// Entrypoint for the Encoder API.
///
/// The general usage follows these steps:
/// 1. Initialize the encoder.
/// 2. Set up the desired encoding parameters.
/// 3. Allocate or register input and output buffers.
/// 4. Copy frames to input buffers, encode, and read out of output bitstream.
/// 5. Close the encoding session and clean up.
///
/// With this wrapper cleanup is performed automatically.
/// To do the other steps this struct provides associated functions
/// such as [`Encoder::get_encode_guids`], [`Encoder::encode_picture`],
/// and [`Encoder::create_input_buffer`].
///
/// See the [NVIDIA Video Codec SDK Encoder Programming Guide](https://docs.nvidia.com/video-technologies/video-codec-sdk/12.0/nvenc-video-encoder-api-prog-guide/index.html).
pub struct Encoder {
    pub(crate) ptr: *mut c_void,
    // Used to make sure that CudaDevice stays alive while the Encoder does
    _device: Device,
}

/// The client must flush the encoder before freeing any resources.
/// Do this by sending an EOS encode picture packet.
/// The client must free all the input and output resources before
/// destroying the encoder.
/// If using events, they must also be unregistered.
impl Drop for Encoder {
    fn drop(&mut self) {
        unsafe { (ENCODE_API.destroy_encoder)(self.ptr) }
            .result()
            .expect("The encoder pointer should be valid.");
    }
}

impl Encoder {
    /// Create an [`Encoder`] with CUDA as the encode device.
    ///
    /// # Errors
    ///
    /// Could error if there was no encode capable device detected
    /// or if the encode device was invalid.
    ///
    /// # Examples
    ///
    /// ```
    /// # use cudarc::driver::CudaDevice;
    /// # use nvidia_video_codec_sdk::Encoder;
    /// let cuda_device = CudaDevice::new(0).unwrap();
    /// let encoder = Encoder::cuda(cuda_device).unwrap();
    /// ```
    pub fn cuda(cuda_device: Arc<CudaDevice>) -> Result<Self, EncodeError> {
        let mut encoder = MaybeUninit::uninit();
        let mut session_params = NV_ENC_OPEN_ENCODE_SESSION_EX_PARAMS {
            version: NV_ENC_OPEN_ENCODE_SESSION_EX_PARAMS_VER,
            deviceType: NV_ENC_DEVICE_TYPE::NV_ENC_DEVICE_TYPE_CUDA,
            apiVersion: NVENCAPI_VERSION,
            // Pass the CUDA Context as the device.
            device: (*cuda_device.cu_primary_ctx()).cast::<c_void>(),
            ..Default::default()
        };

        if let err @ Err(_) = unsafe {
            (ENCODE_API.open_encode_session_ex)(&mut session_params, encoder.as_mut_ptr())
        }
        .result()
        {
            // We are required to destroy the encoder if there was an error.
            unsafe { (ENCODE_API.destroy_encoder)(encoder.assume_init()) }.result()?;
            err?;
        };

        Ok(Encoder {
            ptr: unsafe { encoder.assume_init() },
            _device: cuda_device,
        })
    }

    // TODO: other encode devices

    /// Get the description of the last error reported by the API.
    ///
    /// # Examples
    ///
    /// ```
    /// # use cudarc::driver::CudaDevice;
    /// # use nvidia_video_codec_sdk::{Encoder, EncodeError, sys::nvEncodeAPI::GUID};
    /// # let cuda_device = CudaDevice::new(0).unwrap();
    /// let encoder = Encoder::cuda(cuda_device).unwrap();
    /// // Cause an error by passing in an invalid GUID.
    /// assert_eq!(
    ///     encoder.get_supported_input_formats(GUID::default()),
    ///     Err(EncodeError::InvalidParam)
    /// );
    /// // Get the error message.
    /// // Unfortunately, it's not always helpful.
    /// assert_eq!(
    ///     encoder.get_last_error_string().to_string_lossy(),
    ///     "EncodeAPI Internal Error."
    /// );
    /// ```
    #[must_use]
    pub fn get_last_error_string(&self) -> &CStr {
        unsafe { CStr::from_ptr((ENCODE_API.get_last_error_string)(self.ptr)) }
    }

    pub fn get_encode_guids(&self) -> Result<Vec<GUID>, EncodeError> {
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

    pub fn get_preset_guids(&self, encode_guid: GUID) -> Result<Vec<GUID>, EncodeError> {
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

    pub fn get_profile_guids(&self, encode_guid: GUID) -> Result<Vec<GUID>, EncodeError> {
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
    ) -> Result<Vec<NV_ENC_BUFFER_FORMAT>, EncodeError> {
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
    ) -> Result<NV_ENC_PRESET_CONFIG, EncodeError> {
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
    ) -> Result<(), EncodeError> {
        unsafe { (ENCODE_API.initialize_encoder)(self.ptr, &mut initialize_params) }.result()
    }

    pub fn encode_picture(
        &self,
        mut encode_pic_params: NV_ENC_PIC_PARAMS,
    ) -> Result<(), EncodeError> {
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
    pub fn end_of_stream() -> Self {
        NV_ENC_PIC_PARAMS {
            version: NV_ENC_PIC_PARAMS_VER,
            encodePicFlags: NV_ENC_PIC_FLAGS::NV_ENC_PIC_FLAG_EOS as u32,
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

    // TODO: Add other options
}
