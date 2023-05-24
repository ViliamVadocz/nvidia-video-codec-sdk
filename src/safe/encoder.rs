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
/// such as [`Encoder::get_encode_guids`] or
/// [`Encoder::get_supported_input_formats`].
///
/// Once the configuration is completed, a session should be initialized with
/// [`Encoder::initialize_encoder_session`] to get a [`Session`].
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
/// Do this by sending an EOS encode picture packet
/// (This is done automatically when [`Session`] is dropped).
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

    // TODO:
    // - Make Encoder generic in Device.
    // - Add functions to create Encoder from other encode devices.

    /// Get the description of the last error reported by the API.
    ///
    /// # Examples
    ///
    /// ```
    /// # use cudarc::driver::CudaDevice;
    /// # use nvidia_video_codec_sdk::{sys::nvEncodeAPI::GUID, EncodeError, Encoder};
    /// # let cuda_device = CudaDevice::new(0).unwrap();
    /// let encoder = Encoder::initialize_with_cuda(cuda_device).unwrap();
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
        let mut supported_count = MaybeUninit::uninit();
        unsafe { (ENCODE_API.get_encode_guid_count)(self.ptr, supported_count.as_mut_ptr()) }
            .result()?;
        let supported_count = unsafe { supported_count.assume_init() };
        // Get the supported GUIDs.
        let mut encode_guids = vec![GUID::default(); supported_count as usize];
        let mut actual_count = MaybeUninit::uninit();
        unsafe {
            (ENCODE_API.get_encode_guids)(
                self.ptr,
                encode_guids.as_mut_ptr(),
                supported_count,
                actual_count.as_mut_ptr(),
            )
        }
        .result()?;
        encode_guids.truncate(unsafe { actual_count.assume_init() } as usize);
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
        let mut preset_count = MaybeUninit::uninit();
        unsafe {
            (ENCODE_API.get_encode_preset_count)(self.ptr, encode_guid, preset_count.as_mut_ptr())
        }
        .result()?;
        let preset_count = unsafe { preset_count.assume_init() };
        // Get the preset GUIDs.
        let mut actual_count = MaybeUninit::uninit();
        let mut preset_guids = vec![GUID::default(); preset_count as usize];
        unsafe {
            (ENCODE_API.get_encode_preset_guids)(
                self.ptr,
                encode_guid,
                preset_guids.as_mut_ptr(),
                preset_count,
                actual_count.as_mut_ptr(),
            )
        }
        .result()?;
        preset_guids.truncate(unsafe { actual_count.assume_init() } as usize);
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
        let mut profile_count = MaybeUninit::uninit();
        unsafe {
            (ENCODE_API.get_encode_profile_guid_count)(
                self.ptr,
                encode_guid,
                profile_count.as_mut_ptr(),
            )
        }
        .result()?;
        let profile_count = unsafe { profile_count.assume_init() };
        // Get the profile GUIDs.
        let mut profile_guids = vec![GUID::default(); profile_count as usize];
        let mut actual_count = MaybeUninit::uninit();
        unsafe {
            (ENCODE_API.get_encode_profile_guids)(
                self.ptr,
                encode_guid,
                profile_guids.as_mut_ptr(),
                profile_count,
                actual_count.as_mut_ptr(),
            )
        }
        .result()?;
        profile_guids.truncate(unsafe { actual_count.assume_init() } as usize);
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
        let mut format_count = MaybeUninit::uninit();
        unsafe {
            (ENCODE_API.get_input_format_count)(self.ptr, encode_guid, format_count.as_mut_ptr())
        }
        .result()?;
        let format_count = unsafe { format_count.assume_init() };
        // Get the supported input formats.
        let mut supported_input_formats =
            vec![NV_ENC_BUFFER_FORMAT::NV_ENC_BUFFER_FORMAT_UNDEFINED; format_count as usize];
        let mut actual_count = MaybeUninit::uninit();
        unsafe {
            (ENCODE_API.get_input_formats)(
                self.ptr,
                encode_guid,
                supported_input_formats.as_mut_ptr(),
                format_count,
                actual_count.as_mut_ptr(),
            )
        }
        .result()?;
        supported_input_formats.truncate(unsafe { actual_count.assume_init() } as usize);
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
        .result()?;
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
    ///     .initialize_encoder_session(NV_ENC_INITIALIZE_PARAMS::new(
    ///         NV_ENC_CODEC_H264_GUID,
    ///         1920,
    ///         1080,
    ///     ))
    ///     .unwrap();
    /// ```
    pub fn initialize_encoder_session(
        self,
        mut initialize_params: NV_ENC_INITIALIZE_PARAMS,
    ) -> Result<Session, EncodeError> {
        unsafe { (ENCODE_API.initialize_encoder)(self.ptr, &mut initialize_params) }.result()?;
        Ok(Session { encoder: self })
    }
}

/// An encoding session to create input/output buffers and encode frames.
///
/// You need to call [`Encoder::initialize_encoder_session`] before you can
/// encode frames using the session. On drop, the session will automatically
/// send an empty EOS frame to flush the encoder.
#[derive(Debug)]
pub struct Session {
    pub(crate) encoder: Encoder,
}

impl Session {
    /// Get the encoder used for this session.
    ///
    /// This might be useful if you want to use some of
    /// the functions on [`Encoder`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use cudarc::driver::CudaDevice;
    /// # use nvidia_video_codec_sdk::{
    /// #     sys::nvEncodeAPI::{NV_ENC_CODEC_H264_GUID, NV_ENC_INITIALIZE_PARAMS},
    /// #     Encoder,
    /// # };
    /// //* Create encoder. *//
    /// # let cuda_device = CudaDevice::new(0).unwrap();
    /// # let encoder = Encoder::initialize_with_cuda(cuda_device).unwrap();
    ///
    /// //* Set `encode_guid` and check that H.264 encoding is supported. *//
    /// # let encode_guid = NV_ENC_CODEC_H264_GUID;
    /// # let encode_guids = encoder.get_encode_guids().unwrap();
    /// # assert!(encode_guids.contains(&encode_guid));
    ///
    /// let session = encoder
    ///     .initialize_encoder_session(NV_ENC_INITIALIZE_PARAMS::new(encode_guid, 1920, 1080))
    ///     .unwrap();
    /// // We can still use the encoder like this:
    /// let _input_formats = session
    ///     .get_encoder()
    ///     .get_supported_input_formats(encode_guid);
    /// ```
    #[must_use]
    pub fn get_encoder(&self) -> &Encoder {
        &self.encoder
    }

    /// Encode a frame.
    ///
    /// See [NVIDIA docs](https://docs.nvidia.com/video-technologies/video-codec-sdk/12.0/nvenc-video-encoder-api-prog-guide/index.html#submitting-input-frame-for-encoding).
    ///
    /// # Errors
    ///
    /// Could error if the encode picture parameters were invalid or otherwise
    /// incorrect, or if we run out memory.
    ///
    /// There are two recoverable errors:
    /// - If this returns [`EncodeError::EncoderBusy`] then you should retry
    ///   after a few milliseconds.
    /// - If this returns [`EncodeError::NeedMoreInput`], the client should not
    ///   lock the output bitstream yet. They should continue encoding until
    ///   this function returns `Ok`, and then lock the bitstreams in the order
    ///   in which they were originally used.
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
    ///
    /// //* Set `encode_guid` and `buffer_format`, and check that H.264 encoding and the ARGB format are supported. *//
    /// # let encode_guid = NV_ENC_CODEC_H264_GUID;
    /// # let encode_guids = encoder.get_encode_guids().unwrap();
    /// # assert!(encode_guids.contains(&encode_guid));
    /// # let buffer_format = NV_ENC_BUFFER_FORMAT_ARGB;
    /// # let input_formats = encoder.get_supported_input_formats(encode_guid).unwrap();
    /// # assert!(input_formats.contains(&buffer_format));
    ///
    /// // Begin encoder session.
    /// let session = encoder
    ///     .initialize_encoder_session(
    ///         NV_ENC_INITIALIZE_PARAMS::new(encode_guid, WIDTH, HEIGHT)
    ///             .display_aspect_ratio(16, 9)
    ///             .framerate(30, 1)
    ///             .enable_picture_type_decision(),
    ///     )
    ///     .unwrap();
    ///
    /// //* Create input and output buffers. *//
    /// # let mut input_buffer = session
    /// #     .create_input_buffer(WIDTH, HEIGHT, buffer_format)
    /// #     .unwrap();
    /// # let mut output_bitstream = session.create_output_bitstream().unwrap();
    ///
    /// // Encode frame.
    /// unsafe { input_buffer.lock().unwrap().write(&[0; DATA_LEN]) };
    /// session
    ///     .encode_picture(NV_ENC_PIC_PARAMS::new(
    ///         WIDTH,
    ///         HEIGHT,
    ///         &mut input_buffer,
    ///         &mut output_bitstream,
    ///         buffer_format,
    ///         NV_ENC_PIC_STRUCT::NV_ENC_PIC_STRUCT_FRAME,
    ///     ))
    ///     .unwrap();
    /// # // TODO: check that output is correct.
    /// let _data = output_bitstream.lock_and_read(true).unwrap();
    /// ```
    pub fn encode_picture(
        &self,
        mut encode_pic_params: NV_ENC_PIC_PARAMS,
    ) -> Result<(), EncodeError> {
        unsafe { (ENCODE_API.encode_picture)(self.encoder.ptr, &mut encode_pic_params) }.result()
    }
}

/// Send an EOS notifications on drop to flush the encoder.
impl Drop for Session {
    fn drop(&mut self) {
        while let Err(EncodeError::EncoderBusy) =
            self.encode_picture(NV_ENC_PIC_PARAMS::end_of_stream())
        {}
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
