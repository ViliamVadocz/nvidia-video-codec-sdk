use std::{ffi::c_void, ptr, sync::Arc};

use cudarc::driver::{result::init, CudaDevice};

use super::{
    api::ENCODE_API,
    buffer::{EncoderInput, EncoderOutput},
    result::EncodeError,
    session::Session,
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
/// such as [`Encoder::get_encode_guids`] or
/// [`Encoder::get_supported_input_formats`].
///
/// Once the configuration is completed, a session should be initialized with
/// [`Encoder::start_session`] to get a [`Session`].
/// This type has further function to create input and output buffers
/// and encode pictures.
///
/// See the [NVIDIA Video Codec SDK Encoder Programming Guide](https://docs.nvidia.com/video-technologies/video-codec-sdk/12.0/nvenc-video-encoder-api-prog-guide/index.html).
#[derive(Debug)]
pub struct Encoder {
    pub(crate) ptr: *mut c_void,
    // Used to make sure that CudaDevice stays alive while the Encoder does
    _device: Device,
}

/// The client must flush the encoder before freeing any resources.
/// Do this by sending an EOS encode frame.
/// (This is also done automatically when [`Session`] is dropped.).
///
/// Sending an EOS frame might still generate data, so if you care
/// about this you should send an EOS frame yourself.
///
/// The client must free all the input and output resources before
/// destroying the encoder.
/// If using events, they must also be unregistered.
impl Drop for Encoder {
    fn drop(&mut self) {
        unsafe { (ENCODE_API.destroy_encoder)(self.ptr) }
            .result(self)
            .expect("The encoder pointer should be valid.");
    }
}

impl Encoder {
    /// Create an [`Encoder`] with CUDA as the encode device.
    ///
    /// See [NVIDIA docs](https://docs.nvidia.com/video-technologies/video-codec-sdk/12.0/nvenc-video-encoder-api-prog-guide/index.html#cuda).
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
    /// let encoder = Encoder::initialize_with_cuda(cuda_device).unwrap();
    /// ```
    pub fn initialize_with_cuda(cuda_device: Arc<CudaDevice>) -> Result<Self, EncodeError> {
        let mut encoder = ptr::null_mut();
        let mut session_params = NV_ENC_OPEN_ENCODE_SESSION_EX_PARAMS {
            version: NV_ENC_OPEN_ENCODE_SESSION_EX_PARAMS_VER,
            deviceType: NV_ENC_DEVICE_TYPE::NV_ENC_DEVICE_TYPE_CUDA,
            apiVersion: NVENCAPI_VERSION,
            // Pass the CUDA Context as the device.
            device: (*cuda_device.cu_primary_ctx()).cast::<c_void>(),
            ..Default::default()
        };

        if let err @ Err(_) =
            unsafe { (ENCODE_API.open_encode_session_ex)(&mut session_params, &mut encoder) }
                .result_without_string()
        {
            // We are required to destroy the encoder if there was an error.
            unsafe { (ENCODE_API.destroy_encoder)(encoder) }.result_without_string()?;
            err?;
        };

        Ok(Encoder {
            ptr: encoder,
            _device: cuda_device,
        })
    }

    // TODO:
    // - Make Encoder generic in Device.
    // - Add functions to create Encoder from other encode devices.

    /// Get the encode GUIDs which the encoder supports.
    ///
    /// You should use this function to check whether your
    /// machine supports the video compression standard
    /// that you with to use.
    ///
    /// See [NVIDIA docs](https://docs.nvidia.com/video-technologies/video-codec-sdk/12.0/nvenc-video-encoder-api-prog-guide/index.html#selecting-encoder-codec-guid).
    ///
    /// # Errors
    ///
    /// Could error if we run out of memory.
    ///
    /// # Examples
    ///
    /// ```
    /// # use cudarc::driver::CudaDevice;
    /// # use nvidia_video_codec_sdk::{sys::nvEncodeAPI::NV_ENC_CODEC_H264_GUID, Encoder};
    /// # let cuda_device = CudaDevice::new(0).unwrap();
    /// let encoder = Encoder::initialize_with_cuda(cuda_device).unwrap();
    /// let encode_guids = encoder.get_encode_guids().unwrap();
    /// // Confirm that this machine support encoding to H.264.
    /// assert!(encode_guids.contains(&NV_ENC_CODEC_H264_GUID));
    /// ```
    pub fn get_encode_guids(&self) -> Result<Vec<GUID>, EncodeError> {
        // Query number of supported encoder codec GUIDs.
        let mut supported_count = 0;
        unsafe { (ENCODE_API.get_encode_guid_count)(self.ptr, &mut supported_count) }
            .result(self)?;
        // Get the supported GUIDs.
        let mut encode_guids = vec![GUID::default(); supported_count as usize];
        let mut actual_count = 0;
        unsafe {
            (ENCODE_API.get_encode_guids)(
                self.ptr,
                encode_guids.as_mut_ptr(),
                supported_count,
                &mut actual_count,
            )
        }
        .result(self)?;
        encode_guids.truncate(actual_count as usize);
        Ok(encode_guids)
    }

    /// Get the encode preset GUIDs which the encoder supports
    /// for the given codec GUID.
    ///
    /// You should use this function to check whether your
    /// machine supports the encode preset that you wish to use.
    ///
    /// See [NVIDIA docs](https://docs.nvidia.com/video-technologies/video-codec-sdk/12.0/nvenc-video-encoder-api-prog-guide/index.html#selecting-encoder-preset-configuration).
    ///
    /// # Errors
    ///
    /// Could error if the encode GUID is invalid
    /// or we run out of memory.
    ///
    /// # Examples
    ///
    /// ```
    /// # use cudarc::driver::CudaDevice;
    /// # use nvidia_video_codec_sdk::{
    /// #     sys::nvEncodeAPI::{NV_ENC_CODEC_H264_GUID, NV_ENC_PRESET_P1_GUID},
    /// #     Encoder,
    /// # };
    /// # let cuda_device = CudaDevice::new(0).unwrap();
    /// let encoder = Encoder::initialize_with_cuda(cuda_device).unwrap();
    ///
    /// //* Check if H.264 encoding is supported. *//
    /// # let encode_guids = encoder.get_encode_guids().unwrap();
    /// # assert!(encode_guids.contains(&NV_ENC_CODEC_H264_GUID));
    ///
    /// let preset_guids = encoder.get_preset_guids(NV_ENC_CODEC_H264_GUID).unwrap();
    /// // Confirm that H.264 support the P1 preset (high performance, low quality) on this machine.
    /// assert!(preset_guids.contains(&NV_ENC_PRESET_P1_GUID));
    /// ```
    pub fn get_preset_guids(&self, encode_guid: GUID) -> Result<Vec<GUID>, EncodeError> {
        // Query the number of preset GUIDS.
        let mut preset_count = 0;
        unsafe { (ENCODE_API.get_encode_preset_count)(self.ptr, encode_guid, &mut preset_count) }
            .result(self)?;
        // Get the preset GUIDs.
        let mut actual_count = 0;
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
        .result(self)?;
        preset_guids.truncate(actual_count as usize);
        Ok(preset_guids)
    }

    /// Get the encode profile GUIDs which the encoder supports
    /// for the given codec GUID.
    ///
    /// You should use this function to check whether your
    /// machine supports the encode profile that you wish to use.
    ///
    /// See [NVIDIA docs](https://docs.nvidia.com/video-technologies/video-codec-sdk/12.0/nvenc-video-encoder-api-prog-guide/index.html#selecting-an-encoder-profile).
    ///
    /// # Errors
    ///
    /// Could error if the encode GUID is invalid
    /// or we run out of memory.
    ///
    /// # Examples
    ///
    /// ```
    /// # use cudarc::driver::CudaDevice;
    /// # use nvidia_video_codec_sdk::{
    /// #     sys::nvEncodeAPI::{NV_ENC_CODEC_H264_GUID, NV_ENC_H264_PROFILE_HIGH_GUID},
    /// #     Encoder,
    /// # };
    /// # let cuda_device = CudaDevice::new(0).unwrap();
    /// let encoder = Encoder::initialize_with_cuda(cuda_device).unwrap();
    ///
    /// //* Check if H.264 encoding is supported. *//
    /// # let encode_guids = encoder.get_encode_guids().unwrap();
    /// # assert!(encode_guids.contains(&NV_ENC_CODEC_H264_GUID));
    ///
    /// let profile_guids = encoder.get_profile_guids(NV_ENC_CODEC_H264_GUID).unwrap();
    /// // Confirm that H.264 supports the HIGH profile on this machine.
    /// assert!(profile_guids.contains(&NV_ENC_H264_PROFILE_HIGH_GUID));
    /// ```
    pub fn get_profile_guids(&self, encode_guid: GUID) -> Result<Vec<GUID>, EncodeError> {
        // Query the number of profile GUIDs.
        let mut profile_count = 0;
        unsafe {
            (ENCODE_API.get_encode_profile_guid_count)(self.ptr, encode_guid, &mut profile_count)
        }
        .result(self)?;
        // Get the profile GUIDs.
        let mut profile_guids = vec![GUID::default(); profile_count as usize];
        let mut actual_count = 0;
        unsafe {
            (ENCODE_API.get_encode_profile_guids)(
                self.ptr,
                encode_guid,
                profile_guids.as_mut_ptr(),
                profile_count,
                &mut actual_count,
            )
        }
        .result(self)?;
        profile_guids.truncate(actual_count as usize);
        Ok(profile_guids)
    }

    /// Get the buffer formats which the encoder supports
    /// for the given codec GUID.
    ///
    /// You should use this function to check whether your
    /// machine supports the buffer format that you wish to use.
    ///
    /// See [NVIDIA docs](https://docs.nvidia.com/video-technologies/video-codec-sdk/12.0/nvenc-video-encoder-api-prog-guide/index.html#getting-supported-list-of-input-formats).
    ///
    /// # Errors
    ///
    /// Could error if the encode GUID is invalid
    /// or we run out of memory.
    ///
    /// # Examples
    ///
    /// ```
    /// # use cudarc::driver::CudaDevice;
    /// # use nvidia_video_codec_sdk::{
    /// #     sys::nvEncodeAPI::{NV_ENC_BUFFER_FORMAT, NV_ENC_CODEC_H264_GUID},
    /// #     Encoder,
    /// # };
    /// # let cuda_device = CudaDevice::new(0).unwrap();
    /// let encoder = Encoder::initialize_with_cuda(cuda_device).unwrap();
    ///
    /// //* Check if H.264 encoding is supported. *//
    /// # let encode_guids = encoder.get_encode_guids().unwrap();
    /// # assert!(encode_guids.contains(&NV_ENC_CODEC_H264_GUID));
    ///
    /// let input_guids = encoder
    ///     .get_supported_input_formats(NV_ENC_CODEC_H264_GUID)
    ///     .unwrap();
    /// // Confirm that H.264 supports the `ARGB10` format on this machine.
    /// assert!(input_guids.contains(&NV_ENC_BUFFER_FORMAT::NV_ENC_BUFFER_FORMAT_ARGB10));
    /// ```
    pub fn get_supported_input_formats(
        &self,
        encode_guid: GUID,
    ) -> Result<Vec<NV_ENC_BUFFER_FORMAT>, EncodeError> {
        // Query the number of supported input formats.
        let mut format_count = 0;
        unsafe { (ENCODE_API.get_input_format_count)(self.ptr, encode_guid, &mut format_count) }
            .result(self)?;
        // Get the supported input formats.
        let mut supported_input_formats =
            vec![NV_ENC_BUFFER_FORMAT::NV_ENC_BUFFER_FORMAT_UNDEFINED; format_count as usize];
        let mut actual_count = 0;
        unsafe {
            (ENCODE_API.get_input_formats)(
                self.ptr,
                encode_guid,
                supported_input_formats.as_mut_ptr(),
                format_count,
                &mut actual_count,
            )
        }
        .result(self)?;
        supported_input_formats.truncate(actual_count as usize);
        Ok(supported_input_formats)
    }

    /// Get the preset config struct from the given codec GUID, preset GUID,
    /// and tuning info.
    ///
    /// You should use this function to generate a preset config for the
    /// encoder session if you want to modify the preset further.
    ///
    /// See [NVIDIA docs](https://docs.nvidia.com/video-technologies/video-codec-sdk/12.0/nvenc-video-encoder-api-prog-guide/index.html#selecting-encoder-preset-configuration)
    ///
    /// # Errors
    ///
    /// Could error if `encode_guid` or `preset_guid` is invalid,
    /// if `tuning_info` is set to
    /// [`NV_ENC_TUNING_INFO::NV_ENC_TUNING_INFO_UNDEFINED`] or
    /// [`NV_ENC_TUNING_INFO::NV_ENC_TUNING_INFO_COUNT`],
    /// or if we run out of memory.
    ///
    /// # Examples
    ///
    /// ```
    /// # #![allow(deprecated)]
    /// # use cudarc::driver::CudaDevice;
    /// # use nvidia_video_codec_sdk::{
    /// #     sys::nvEncodeAPI::{
    /// #         NV_ENC_CODEC_H264_GUID,
    /// #         NV_ENC_PRESET_LOW_LATENCY_HQ_GUID,
    /// #         NV_ENC_PRESET_P1_GUID,
    /// #         NV_ENC_TUNING_INFO,
    /// #     },
    /// #     Encoder,
    /// # };
    /// # let cuda_device = CudaDevice::new(0).unwrap();
    /// let encoder = Encoder::initialize_with_cuda(cuda_device).unwrap();
    ///
    /// //* Check if H.264 encoding and the low latency preset are supported. *//
    /// # let encode_guids = encoder.get_encode_guids().unwrap();
    /// # assert!(encode_guids.contains(&NV_ENC_CODEC_H264_GUID));
    /// # let preset_guids = encoder.get_preset_guids(NV_ENC_CODEC_H264_GUID).unwrap();
    /// # assert!(preset_guids.contains(&NV_ENC_PRESET_LOW_LATENCY_HQ_GUID));
    ///
    /// // Create the preset config.
    /// let _preset_config = encoder
    ///     .get_preset_config(
    ///         NV_ENC_CODEC_H264_GUID,
    ///         NV_ENC_PRESET_LOW_LATENCY_HQ_GUID,
    ///         NV_ENC_TUNING_INFO::NV_ENC_TUNING_INFO_ULTRA_LOW_LATENCY,
    ///     )
    ///     .unwrap();
    /// ```
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
        .result(self)?;
        Ok(preset_config)
    }

    /// Initialize an encoder session with the given configuration.
    ///
    /// You must do this before you can encode a picture.
    /// You should use the [`NV_ENC_INITIALIZE_PARAMS`] builder
    /// via [`NV_ENC_INITIALIZE_PARAMS::new`].
    ///
    /// See [NVIDIA docs](https://docs.nvidia.com/video-technologies/video-codec-sdk/12.0/nvenc-video-encoder-api-prog-guide/index.html#initializing-the-hardware-encoder-session).
    ///
    /// # Errors
    ///
    /// Could error if the `initialize_params` are invalid
    /// or if we run out of memory.
    ///
    /// # Examples
    ///
    /// ```
    /// # use cudarc::driver::CudaDevice;
    /// # use nvidia_video_codec_sdk::{
    /// #     sys::nvEncodeAPI::{
    /// #         NV_ENC_CODEC_H264_GUID,
    /// #         NV_ENC_INITIALIZE_PARAMS,
    /// #         NV_ENC_PRESET_LOW_LATENCY_HP_GUID,
    /// #     },
    /// #     Encoder,
    /// # };
    /// # let cuda_device = CudaDevice::new(0).unwrap();
    /// let encoder = Encoder::initialize_with_cuda(cuda_device).unwrap();
    ///
    /// //* Check if `NV_ENC_CODEC_H264_GUID` is supported. *//
    /// # let encode_guids = encoder.get_encode_guids().unwrap();
    /// # assert!(encode_guids.contains(&NV_ENC_CODEC_H264_GUID));
    ///
    /// // Initialize the encoder session.
    /// let _session = encoder
    ///     .start_session(NV_ENC_INITIALIZE_PARAMS::new(
    ///         NV_ENC_CODEC_H264_GUID,
    ///         1920,
    ///         1080,
    ///     ))
    ///     .unwrap();
    /// ```
    pub fn start_session(
        self,
        buffer_format: NV_ENC_BUFFER_FORMAT,
        mut initialize_params: NV_ENC_INITIALIZE_PARAMS,
    ) -> Result<Session, EncodeError> {
        let width = initialize_params.encodeWidth;
        let height = initialize_params.encodeHeight;
        unsafe { (ENCODE_API.initialize_encoder)(self.ptr, &mut initialize_params) }
            .result(&self)?;
        Ok(Session {
            encoder: self,
            width,
            height,
            buffer_format,
        })
    }
}

impl NV_ENC_INITIALIZE_PARAMS {
    /// Builder for [`NV_ENC_INITIALIZE_PARAMS`].
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

    /// Specifies the preset for encoding. If the preset GUID is set then
    /// the preset configuration will be applied before any other parameter.
    #[must_use]
    pub fn preset_guid(mut self, preset_guid: GUID) -> Self {
        self.presetGUID = preset_guid;
        self
    }

    /// Specifies the advanced codec specific structure. If client has sent a
    /// valid codec config structure, it will override parameters set by the
    /// [`NV_ENC_INITIALIZE_PARAMS::preset_guid`].
    ///
    /// The client can query the interface for codec-specific parameters
    /// using [`Encoder::get_preset_config`]. It can then modify (if required)
    /// some of the codec config parameters and send down a custom config
    /// structure using this method. Even in this case the client is
    /// recommended to pass the same preset GUID it has used to get the config.
    #[must_use]
    pub fn encode_config(mut self, encode_config: &mut NV_ENC_CONFIG) -> Self {
        self.encodeConfig = encode_config;
        self
    }

    /// Specifies the display aspect ratio (H264/HEVC) or the render
    /// width/height (AV1).
    #[must_use]
    pub fn display_aspect_ratio(mut self, width: u32, height: u32) -> Self {
        self.darWidth = width;
        self.darHeight = height;
        self
    }

    /// Specifies the framerate in frames per second as a fraction
    /// `numerator / denominator`.
    #[must_use]
    pub fn framerate(mut self, numerator: u32, denominator: u32) -> Self {
        self.frameRateNum = numerator;
        self.frameRateDen = denominator;
        self
    }

    /// Enable the Picture Type Decision to be taken by the
    /// `NvEncodeAPI` interface.
    #[must_use]
    pub fn enable_picture_type_decision(mut self) -> Self {
        self.enablePTD = 1;
        self
    }

    // TODO: Add other options
}

impl NV_ENC_PIC_PARAMS {
    /// Builder for [`NV_ENC_PIC_PARAMS`].
    ///
    /// # Arguments
    ///
    /// * `width` - Input frame width.
    /// * `height` - Input frame height.
    /// * `input_buffer` - Input buffer which implements [`EncoderInput`].
    /// * `output_bitstream` - Output bitstream buffer which implements
    ///   [`EncoderOutput`].
    /// * `buffer_format` - Input buffer format.
    /// * `picture_struct` - The structure of the input picture.
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

    /// Create an EOS empty frame that is used at the
    /// end of encoding to flush the encoder.
    #[must_use]
    pub fn end_of_stream() -> Self {
        NV_ENC_PIC_PARAMS {
            version: NV_ENC_PIC_PARAMS_VER,
            encodePicFlags: NV_ENC_PIC_FLAGS::NV_ENC_PIC_FLAG_EOS as u32,
            ..Default::default()
        }
    }

    /// Specifies the input buffer pitch.
    #[must_use]
    pub fn pitch(mut self, pitch: u32) -> Self {
        self.inputPitch = pitch;
        self
    }

    /// Specifies the frame index associated with the input frame.
    #[must_use]
    pub fn frame_id(mut self, frame_id: u32) -> Self {
        self.frameIdx = frame_id;
        self
    }

    /// Specifies the codec specific per-picture encoding parameters.
    #[must_use]
    pub fn codec_pic_params(mut self, codec_pic_params: NV_ENC_CODEC_PIC_PARAMS) -> Self {
        self.codecPicParams = codec_pic_params;
        self
    }

    // TODO: Add other options
}
