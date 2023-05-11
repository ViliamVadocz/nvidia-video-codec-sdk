use std::{ffi::c_void, sync::Arc};

use cudarc::driver::CudaDevice;

use super::{encoder::Encoder, result::EncodeResult};
use crate::sys::nvEncodeAPI::{
    NvEncodeAPICreateInstance,
    GUID,
    NVENCAPI_VERSION,
    NVENCSTATUS,
    NV_ENCODE_API_FUNCTION_LIST,
    NV_ENCODE_API_FUNCTION_LIST_VER,
    NV_ENC_BUFFER_FORMAT,
    NV_ENC_CAPS_PARAM,
    NV_ENC_CREATE_BITSTREAM_BUFFER,
    NV_ENC_CREATE_INPUT_BUFFER,
    NV_ENC_CREATE_MV_BUFFER,
    NV_ENC_CUSTREAM_PTR,
    NV_ENC_DEVICE_TYPE,
    NV_ENC_EVENT_PARAMS,
    NV_ENC_INITIALIZE_PARAMS,
    NV_ENC_INPUT_PTR,
    NV_ENC_LOCK_BITSTREAM,
    NV_ENC_LOCK_INPUT_BUFFER,
    NV_ENC_MAP_INPUT_RESOURCE,
    NV_ENC_MEONLY_PARAMS,
    NV_ENC_OPEN_ENCODE_SESSION_EX_PARAMS,
    NV_ENC_OPEN_ENCODE_SESSION_EX_PARAMS_VER,
    NV_ENC_OUTPUT_PTR,
    NV_ENC_PIC_PARAMS,
    NV_ENC_PRESET_CONFIG,
    NV_ENC_RECONFIGURE_PARAMS,
    NV_ENC_REGISTERED_PTR,
    NV_ENC_REGISTER_RESOURCE,
    NV_ENC_SEQUENCE_PARAM_PAYLOAD,
    NV_ENC_STAT,
    NV_ENC_TUNING_INFO,
};

lazy_static! {
    pub static ref ENCODE_API: EncodeAPI = EncodeAPI::new().unwrap();
}

type OpenEncodeSession = unsafe extern "C" fn(
    *mut ::core::ffi::c_void,
    u32,
    *mut *mut ::core::ffi::c_void,
) -> NVENCSTATUS;
type GetEncodeGUIDCount = unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut u32) -> NVENCSTATUS;
type GetEncodeGUIDs =
    unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut GUID, u32, *mut u32) -> NVENCSTATUS;
type GetInputFormatCount =
    unsafe extern "C" fn(*mut ::core::ffi::c_void, GUID, *mut u32) -> NVENCSTATUS;
type GetInputFormats = unsafe extern "C" fn(
    *mut ::core::ffi::c_void,
    GUID,
    *mut NV_ENC_BUFFER_FORMAT,
    u32,
    *mut u32,
) -> NVENCSTATUS;
type GetEncodeCaps = unsafe extern "C" fn(
    *mut ::core::ffi::c_void,
    GUID,
    *mut NV_ENC_CAPS_PARAM,
    *mut ::core::ffi::c_int,
) -> NVENCSTATUS;
type GetEncodePresetCount =
    unsafe extern "C" fn(*mut ::core::ffi::c_void, GUID, *mut u32) -> NVENCSTATUS;
type GetEncodePresetGUIDs =
    unsafe extern "C" fn(*mut ::core::ffi::c_void, GUID, *mut GUID, u32, *mut u32) -> NVENCSTATUS;
type GetEncodeProfileGUIDCount = GetEncodePresetCount;
type GetEncodeProfileGUIDs = GetEncodePresetGUIDs;
type GetEncodePresetConfig = unsafe extern "C" fn(
    *mut ::core::ffi::c_void,
    GUID,
    GUID,
    *mut NV_ENC_PRESET_CONFIG,
) -> NVENCSTATUS;
type GetEncodePresetConfigEx = unsafe extern "C" fn(
    *mut ::core::ffi::c_void,
    GUID,
    GUID,
    NV_ENC_TUNING_INFO,
    *mut NV_ENC_PRESET_CONFIG,
) -> NVENCSTATUS;
type InitializeEncoder =
    unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut NV_ENC_INITIALIZE_PARAMS) -> NVENCSTATUS;
type CreateInputBuffer =
    unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut NV_ENC_CREATE_INPUT_BUFFER) -> NVENCSTATUS;
type DestroyInputBuffer =
    unsafe extern "C" fn(*mut ::core::ffi::c_void, NV_ENC_INPUT_PTR) -> NVENCSTATUS;
type CreateBitstreamBuffer = unsafe extern "C" fn(
    *mut ::core::ffi::c_void,
    *mut NV_ENC_CREATE_BITSTREAM_BUFFER,
) -> NVENCSTATUS;
type DestroyBitstreamBuffer =
    unsafe extern "C" fn(*mut ::core::ffi::c_void, NV_ENC_OUTPUT_PTR) -> NVENCSTATUS;
type EncodePicture =
    unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut NV_ENC_PIC_PARAMS) -> NVENCSTATUS;
type LockBitstream =
    unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut NV_ENC_LOCK_BITSTREAM) -> NVENCSTATUS;
type UnlockBitstream =
    unsafe extern "C" fn(*mut ::core::ffi::c_void, NV_ENC_OUTPUT_PTR) -> NVENCSTATUS;
type LockInputBuffer =
    unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut NV_ENC_LOCK_INPUT_BUFFER) -> NVENCSTATUS;
type UnlockInputBuffer =
    unsafe extern "C" fn(*mut ::core::ffi::c_void, NV_ENC_INPUT_PTR) -> NVENCSTATUS;
type GetEncodeStats =
    unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut NV_ENC_STAT) -> NVENCSTATUS;
type GetSequenceParams = unsafe extern "C" fn(
    *mut ::core::ffi::c_void,
    *mut NV_ENC_SEQUENCE_PARAM_PAYLOAD,
) -> NVENCSTATUS;
type RegisterAsyncEvent =
    unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut NV_ENC_EVENT_PARAMS) -> NVENCSTATUS;
type UnregisterAsyncEvent =
    unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut NV_ENC_EVENT_PARAMS) -> NVENCSTATUS;
type MapInputResource =
    unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut NV_ENC_MAP_INPUT_RESOURCE) -> NVENCSTATUS;
type UnmapInputResource =
    unsafe extern "C" fn(*mut ::core::ffi::c_void, NV_ENC_INPUT_PTR) -> NVENCSTATUS;
type DestroyEncoder = unsafe extern "C" fn(encoder: *mut ::core::ffi::c_void) -> NVENCSTATUS;
type InvalidateRefFrames = unsafe extern "C" fn(*mut ::core::ffi::c_void, u64) -> NVENCSTATUS;
type OpenEncodeSessionEx = unsafe extern "C" fn(
    *mut NV_ENC_OPEN_ENCODE_SESSION_EX_PARAMS,
    *mut *mut ::core::ffi::c_void,
) -> NVENCSTATUS;
type RegisterResource =
    unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut NV_ENC_REGISTER_RESOURCE) -> NVENCSTATUS;
type UnregisterResource =
    unsafe extern "C" fn(*mut ::core::ffi::c_void, NV_ENC_REGISTERED_PTR) -> NVENCSTATUS;
type ReconfigureEncoder =
    unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut NV_ENC_RECONFIGURE_PARAMS) -> NVENCSTATUS;
type CreateMVBuffer =
    unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut NV_ENC_CREATE_MV_BUFFER) -> NVENCSTATUS;
type DestroyMVBuffer =
    unsafe extern "C" fn(*mut ::core::ffi::c_void, NV_ENC_OUTPUT_PTR) -> NVENCSTATUS;
type RunMotionEstimationOnly =
    unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut NV_ENC_MEONLY_PARAMS) -> NVENCSTATUS;
type GetLastErrorString =
    unsafe extern "C" fn(encoder: *mut ::core::ffi::c_void) -> *const ::core::ffi::c_char;
type SetIOCudaStreams = unsafe extern "C" fn(
    *mut ::core::ffi::c_void,
    NV_ENC_CUSTREAM_PTR,
    NV_ENC_CUSTREAM_PTR,
) -> NVENCSTATUS;
type GetSequenceParamEx = unsafe extern "C" fn(
    *mut ::core::ffi::c_void,
    *mut NV_ENC_INITIALIZE_PARAMS,
    *mut NV_ENC_SEQUENCE_PARAM_PAYLOAD,
) -> NVENCSTATUS;

#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
pub struct EncodeAPI {
    pub(crate) open_encode_session: OpenEncodeSession,
    pub(crate) open_encode_session_ex: OpenEncodeSessionEx,
    pub(crate) initialize_encoder: InitializeEncoder,
    pub(crate) reconfigure_encoder: ReconfigureEncoder,
    pub(crate) destroy_encoder: DestroyEncoder,
    pub(crate) get_encode_guid_count: GetEncodeGUIDCount,
    pub(crate) get_encode_guids: GetEncodeGUIDs,
    pub(crate) get_encode_profile_guid_count: GetEncodeProfileGUIDCount,
    pub(crate) get_encode_profile_guids: GetEncodeProfileGUIDs,
    pub(crate) get_input_format_count: GetInputFormatCount,
    pub(crate) get_input_formats: GetInputFormats,
    pub(crate) get_encode_preset_count: GetEncodePresetCount,
    pub(crate) get_encode_preset_guids: GetEncodePresetGUIDs,
    pub(crate) get_encode_preset_config: GetEncodePresetConfig,
    pub(crate) get_encode_preset_config_ex: GetEncodePresetConfigEx,
    pub(crate) get_encode_caps: GetEncodeCaps,
    pub(crate) create_input_buffer: CreateInputBuffer,
    pub(crate) destroy_input_buffer: DestroyInputBuffer,
    pub(crate) lock_input_buffer: LockInputBuffer,
    pub(crate) unlock_input_buffer: UnlockInputBuffer,
    pub(crate) create_bitstream_buffer: CreateBitstreamBuffer,
    pub(crate) destroy_bitstream_buffer: DestroyBitstreamBuffer,
    pub(crate) lock_bitstream: LockBitstream,
    pub(crate) unlock_bitstream: UnlockBitstream,
    pub(crate) map_input_resource: MapInputResource,
    pub(crate) unmap_input_resource: UnmapInputResource,
    pub(crate) register_resource: RegisterResource,
    pub(crate) unregister_resource: UnregisterResource,
    pub(crate) create_mv_buffer: CreateMVBuffer,
    pub(crate) destroy_mv_buffer: DestroyMVBuffer,
    pub(crate) encode_picture: EncodePicture,
    pub(crate) get_encode_stats: GetEncodeStats,
    pub(crate) get_sequence_params: GetSequenceParams,
    pub(crate) get_sequence_param_ex: GetSequenceParamEx,
    pub(crate) register_async_event: RegisterAsyncEvent,
    pub(crate) unregister_async_event: UnregisterAsyncEvent,
    pub(crate) invalidate_ref_frames: InvalidateRefFrames,
    pub(crate) run_motion_estimation_only: RunMotionEstimationOnly,
    pub(crate) get_last_error_string: GetLastErrorString,
    pub(crate) set_io_cuda_streams: SetIOCudaStreams,
}

impl EncodeAPI {
    fn new() -> EncodeResult<Self> {
        // Create empty function buffer.
        let mut function_list = NV_ENCODE_API_FUNCTION_LIST {
            version: NV_ENCODE_API_FUNCTION_LIST_VER,
            ..Default::default()
        };

        // Create Encode API Instance (populate function buffer).
        unsafe {
            NvEncodeAPICreateInstance(&mut function_list as *mut NV_ENCODE_API_FUNCTION_LIST)
        }
        .result()?;

        Ok(Self {
            open_encode_session: function_list.nvEncOpenEncodeSession.unwrap(),
            open_encode_session_ex: function_list.nvEncOpenEncodeSessionEx.unwrap(),
            initialize_encoder: function_list.nvEncInitializeEncoder.unwrap(),
            reconfigure_encoder: function_list.nvEncReconfigureEncoder.unwrap(),
            destroy_encoder: function_list.nvEncDestroyEncoder.unwrap(),
            get_encode_guid_count: function_list.nvEncGetEncodeGUIDCount.unwrap(),
            get_encode_guids: function_list.nvEncGetEncodeGUIDs.unwrap(),
            get_encode_profile_guid_count: function_list.nvEncGetEncodeProfileGUIDCount.unwrap(),
            get_encode_profile_guids: function_list.nvEncGetEncodeProfileGUIDs.unwrap(),
            get_input_format_count: function_list.nvEncGetInputFormatCount.unwrap(),
            get_input_formats: function_list.nvEncGetInputFormats.unwrap(),
            get_encode_preset_count: function_list.nvEncGetEncodePresetCount.unwrap(),
            get_encode_preset_guids: function_list.nvEncGetEncodePresetGUIDs.unwrap(),
            get_encode_preset_config: function_list.nvEncGetEncodePresetConfig.unwrap(),
            get_encode_preset_config_ex: function_list.nvEncGetEncodePresetConfigEx.unwrap(),
            get_encode_caps: function_list.nvEncGetEncodeCaps.unwrap(),
            create_input_buffer: function_list.nvEncCreateInputBuffer.unwrap(),
            destroy_input_buffer: function_list.nvEncDestroyInputBuffer.unwrap(),
            lock_input_buffer: function_list.nvEncLockInputBuffer.unwrap(),
            unlock_input_buffer: function_list.nvEncUnlockInputBuffer.unwrap(),
            create_bitstream_buffer: function_list.nvEncCreateBitstreamBuffer.unwrap(),
            destroy_bitstream_buffer: function_list.nvEncDestroyBitstreamBuffer.unwrap(),
            lock_bitstream: function_list.nvEncLockBitstream.unwrap(),
            unlock_bitstream: function_list.nvEncUnlockBitstream.unwrap(),
            map_input_resource: function_list.nvEncMapInputResource.unwrap(),
            unmap_input_resource: function_list.nvEncUnmapInputResource.unwrap(),
            register_resource: function_list.nvEncRegisterResource.unwrap(),
            unregister_resource: function_list.nvEncUnregisterResource.unwrap(),
            create_mv_buffer: function_list.nvEncCreateMVBuffer.unwrap(),
            destroy_mv_buffer: function_list.nvEncDestroyMVBuffer.unwrap(),
            encode_picture: function_list.nvEncEncodePicture.unwrap(),
            get_encode_stats: function_list.nvEncGetEncodeStats.unwrap(),
            get_sequence_params: function_list.nvEncGetSequenceParams.unwrap(),
            get_sequence_param_ex: function_list.nvEncGetSequenceParamEx.unwrap(),
            register_async_event: function_list.nvEncRegisterAsyncEvent.unwrap(),
            unregister_async_event: function_list.nvEncUnregisterAsyncEvent.unwrap(),
            invalidate_ref_frames: function_list.nvEncInvalidateRefFrames.unwrap(),
            run_motion_estimation_only: function_list.nvEncRunMotionEstimationOnly.unwrap(),
            get_last_error_string: function_list.nvEncGetLastErrorString.unwrap(),
            set_io_cuda_streams: function_list.nvEncSetIOCudaStreams.unwrap(),
        })
    }

    pub fn open_encode_session_with_cuda(
        &self,
        cuda_device: Arc<CudaDevice>,
    ) -> EncodeResult<Encoder> {
        let mut encoder = std::ptr::null_mut();
        let mut session_params = NV_ENC_OPEN_ENCODE_SESSION_EX_PARAMS {
            version: NV_ENC_OPEN_ENCODE_SESSION_EX_PARAMS_VER,
            deviceType: NV_ENC_DEVICE_TYPE::NV_ENC_DEVICE_TYPE_CUDA,
            apiVersion: NVENCAPI_VERSION,
            // Pass the CUDA Context as the device.
            device: (*cuda_device.cu_primary_ctx()).cast::<c_void>(),
            ..Default::default()
        };

        if let err @ Err(_) =
            unsafe { (self.open_encode_session_ex)(&mut session_params, &mut encoder) }.result()
        {
            // We are required to destroy the encoder if there was an error.
            unsafe { (self.destroy_encoder)(encoder) }.result()?;
            err?;
        };

        Ok(Encoder::new(encoder, cuda_device))
    }

    // TODO: other encode devices
}
