use core::ffi::{c_int, c_void};
use std::mem::MaybeUninit;

use crate::sys::nvEncodeAPI::{
    NvEncodeAPICreateInstance,
    NvEncodeAPIGetMaxSupportedVersion,
    GUID,
    NVENCAPI_MAJOR_VERSION,
    NVENCAPI_MINOR_VERSION,
    NVENCSTATUS,
    NV_ENCODE_API_FUNCTION_LIST,
    NV_ENCODE_API_FUNCTION_LIST_VER,
    NV_ENC_BUFFER_FORMAT,
    NV_ENC_CAPS_PARAM,
    NV_ENC_CREATE_BITSTREAM_BUFFER,
    NV_ENC_CREATE_INPUT_BUFFER,
    NV_ENC_CREATE_MV_BUFFER,
    NV_ENC_CUSTREAM_PTR,
    NV_ENC_EVENT_PARAMS,
    NV_ENC_INITIALIZE_PARAMS,
    NV_ENC_INPUT_PTR,
    NV_ENC_LOCK_BITSTREAM,
    NV_ENC_LOCK_INPUT_BUFFER,
    NV_ENC_MAP_INPUT_RESOURCE,
    NV_ENC_MEONLY_PARAMS,
    NV_ENC_OPEN_ENCODE_SESSION_EX_PARAMS,
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
    /// A lazy static for the Encoder API.
    ///
    /// You should not interact with this directly.
    /// [`Encoder`] exposes much of the functionality and provides a nicer API.
    pub(crate) static ref ENCODE_API: EncodeAPI =
        EncodeAPI::new();
}

// Function type aliases to shorten later definitions.
type OpenEncodeSession = unsafe extern "C" fn(*mut c_void, u32, *mut *mut c_void) -> NVENCSTATUS;
type GetEncodeGUIDCount = unsafe extern "C" fn(*mut c_void, *mut u32) -> NVENCSTATUS;
type GetEncodeGUIDs = unsafe extern "C" fn(*mut c_void, *mut GUID, u32, *mut u32) -> NVENCSTATUS;
type GetInputFormatCount = unsafe extern "C" fn(*mut c_void, GUID, *mut u32) -> NVENCSTATUS;
type GetInputFormats = unsafe extern "C" fn(
    *mut c_void,
    GUID,
    *mut NV_ENC_BUFFER_FORMAT,
    u32,
    *mut u32,
) -> NVENCSTATUS;
type GetEncodeCaps =
    unsafe extern "C" fn(*mut c_void, GUID, *mut NV_ENC_CAPS_PARAM, *mut c_int) -> NVENCSTATUS;
type GetEncodePresetCount = unsafe extern "C" fn(*mut c_void, GUID, *mut u32) -> NVENCSTATUS;
type GetEncodePresetGUIDs =
    unsafe extern "C" fn(*mut c_void, GUID, *mut GUID, u32, *mut u32) -> NVENCSTATUS;
type GetEncodeProfileGUIDCount = GetEncodePresetCount;
type GetEncodeProfileGUIDs = GetEncodePresetGUIDs;
type GetEncodePresetConfig =
    unsafe extern "C" fn(*mut c_void, GUID, GUID, *mut NV_ENC_PRESET_CONFIG) -> NVENCSTATUS;
type GetEncodePresetConfigEx = unsafe extern "C" fn(
    *mut c_void,
    GUID,
    GUID,
    NV_ENC_TUNING_INFO,
    *mut NV_ENC_PRESET_CONFIG,
) -> NVENCSTATUS;
type InitializeEncoder =
    unsafe extern "C" fn(*mut c_void, *mut NV_ENC_INITIALIZE_PARAMS) -> NVENCSTATUS;
type CreateInputBuffer =
    unsafe extern "C" fn(*mut c_void, *mut NV_ENC_CREATE_INPUT_BUFFER) -> NVENCSTATUS;
type DestroyInputBuffer = unsafe extern "C" fn(*mut c_void, NV_ENC_INPUT_PTR) -> NVENCSTATUS;
type CreateBitstreamBuffer =
    unsafe extern "C" fn(*mut c_void, *mut NV_ENC_CREATE_BITSTREAM_BUFFER) -> NVENCSTATUS;
type DestroyBitstreamBuffer = unsafe extern "C" fn(*mut c_void, NV_ENC_OUTPUT_PTR) -> NVENCSTATUS;
type EncodePicture = unsafe extern "C" fn(*mut c_void, *mut NV_ENC_PIC_PARAMS) -> NVENCSTATUS;
type LockBitstream = unsafe extern "C" fn(*mut c_void, *mut NV_ENC_LOCK_BITSTREAM) -> NVENCSTATUS;
type UnlockBitstream = unsafe extern "C" fn(*mut c_void, NV_ENC_OUTPUT_PTR) -> NVENCSTATUS;
type LockInputBuffer =
    unsafe extern "C" fn(*mut c_void, *mut NV_ENC_LOCK_INPUT_BUFFER) -> NVENCSTATUS;
type UnlockInputBuffer = unsafe extern "C" fn(*mut c_void, NV_ENC_INPUT_PTR) -> NVENCSTATUS;
type GetEncodeStats = unsafe extern "C" fn(*mut c_void, *mut NV_ENC_STAT) -> NVENCSTATUS;
type GetSequenceParams =
    unsafe extern "C" fn(*mut c_void, *mut NV_ENC_SEQUENCE_PARAM_PAYLOAD) -> NVENCSTATUS;
type RegisterAsyncEvent =
    unsafe extern "C" fn(*mut c_void, *mut NV_ENC_EVENT_PARAMS) -> NVENCSTATUS;
type UnregisterAsyncEvent =
    unsafe extern "C" fn(*mut c_void, *mut NV_ENC_EVENT_PARAMS) -> NVENCSTATUS;
type MapInputResource =
    unsafe extern "C" fn(*mut c_void, *mut NV_ENC_MAP_INPUT_RESOURCE) -> NVENCSTATUS;
type UnmapInputResource = unsafe extern "C" fn(*mut c_void, NV_ENC_INPUT_PTR) -> NVENCSTATUS;
type DestroyEncoder = unsafe extern "C" fn(encoder: *mut c_void) -> NVENCSTATUS;
type InvalidateRefFrames = unsafe extern "C" fn(*mut c_void, u64) -> NVENCSTATUS;
type OpenEncodeSessionEx = unsafe extern "C" fn(
    *mut NV_ENC_OPEN_ENCODE_SESSION_EX_PARAMS,
    *mut *mut c_void,
) -> NVENCSTATUS;
type RegisterResource =
    unsafe extern "C" fn(*mut c_void, *mut NV_ENC_REGISTER_RESOURCE) -> NVENCSTATUS;
type UnregisterResource = unsafe extern "C" fn(*mut c_void, NV_ENC_REGISTERED_PTR) -> NVENCSTATUS;
type ReconfigureEncoder =
    unsafe extern "C" fn(*mut c_void, *mut NV_ENC_RECONFIGURE_PARAMS) -> NVENCSTATUS;
type CreateMVBuffer =
    unsafe extern "C" fn(*mut c_void, *mut NV_ENC_CREATE_MV_BUFFER) -> NVENCSTATUS;
type DestroyMVBuffer = unsafe extern "C" fn(*mut c_void, NV_ENC_OUTPUT_PTR) -> NVENCSTATUS;
type RunMotionEstimationOnly =
    unsafe extern "C" fn(*mut c_void, *mut NV_ENC_MEONLY_PARAMS) -> NVENCSTATUS;
type GetLastErrorString = unsafe extern "C" fn(encoder: *mut c_void) -> *const ::core::ffi::c_char;
type SetIOCudaStreams =
    unsafe extern "C" fn(*mut c_void, NV_ENC_CUSTREAM_PTR, NV_ENC_CUSTREAM_PTR) -> NVENCSTATUS;
type GetSequenceParamEx = unsafe extern "C" fn(
    *mut c_void,
    *mut NV_ENC_INITIALIZE_PARAMS,
    *mut NV_ENC_SEQUENCE_PARAM_PAYLOAD,
) -> NVENCSTATUS;

/// An instance of the `NvEncodeAPI` interface, containing function pointers
/// which should be used to interface with the rest of the Encoder API.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub(crate) struct EncodeAPI {
    /// NvEncOpenEncodeSession
    pub(crate) open_encode_session: OpenEncodeSession,
    /// NvEncOpenEncodeSessionEx
    pub(crate) open_encode_session_ex: OpenEncodeSessionEx,
    /// NvEncInitializeEncoder
    pub(crate) initialize_encoder: InitializeEncoder,
    /// NvEncReconfigureEncoder
    pub(crate) reconfigure_encoder: ReconfigureEncoder,
    /// NvEncDestroyEncoder
    pub(crate) destroy_encoder: DestroyEncoder,
    /// NvEncGetEncodeGuidCount
    pub(crate) get_encode_guid_count: GetEncodeGUIDCount,
    /// NvEncGetEncodeGUIDs
    pub(crate) get_encode_guids: GetEncodeGUIDs,
    /// NvEncGetEncodeProfileGuidCount
    pub(crate) get_encode_profile_guid_count: GetEncodeProfileGUIDCount,
    /// NvEncGetEncodeProfileGUIDs
    pub(crate) get_encode_profile_guids: GetEncodeProfileGUIDs,
    /// NvEncGetInputFormatCount
    pub(crate) get_input_format_count: GetInputFormatCount,
    /// NvEncGetInputFormats
    pub(crate) get_input_formats: GetInputFormats,
    /// NvEncGetEncodePresetCount
    pub(crate) get_encode_preset_count: GetEncodePresetCount,
    /// NvEncGetEncodePresetGUIDs
    pub(crate) get_encode_preset_guids: GetEncodePresetGUIDs,
    /// NvEncGetEncodePresetConfig
    pub(crate) get_encode_preset_config: GetEncodePresetConfig,
    /// NvEncGetEncodePresetConfigEx
    pub(crate) get_encode_preset_config_ex: GetEncodePresetConfigEx,
    /// NvEncGetEncodeCaps
    pub(crate) get_encode_caps: GetEncodeCaps,
    /// NvEncCreateInputBuffer
    pub(crate) create_input_buffer: CreateInputBuffer,
    /// NvEncDestroyInputBuffer
    pub(crate) destroy_input_buffer: DestroyInputBuffer,
    /// NvLockInputBuffer
    pub(crate) lock_input_buffer: LockInputBuffer,
    /// NvUnlockInputBuffer
    pub(crate) unlock_input_buffer: UnlockInputBuffer,
    /// NvEncCreateBitstreamBuffer
    pub(crate) create_bitstream_buffer: CreateBitstreamBuffer,
    /// NvEncDestroyBitstreamBuffer
    pub(crate) destroy_bitstream_buffer: DestroyBitstreamBuffer,
    /// NvEncLockBitstream
    pub(crate) lock_bitstream: LockBitstream,
    /// NvEncUnlockBitstream
    pub(crate) unlock_bitstream: UnlockBitstream,
    /// NvEncMapInputResource
    pub(crate) map_input_resource: MapInputResource,
    /// NvEncUnmapInputResource
    pub(crate) unmap_input_resource: UnmapInputResource,
    /// NvEncRegisterResource
    pub(crate) register_resource: RegisterResource,
    /// NvEncUnregisterResource
    pub(crate) unregister_resource: UnregisterResource,
    /// NvEncCreateMVBuffer
    pub(crate) create_mv_buffer: CreateMVBuffer,
    /// NvEncDestroyMVBuffer
    pub(crate) destroy_mv_buffer: DestroyMVBuffer,
    /// NvEncEncodePicture
    pub(crate) encode_picture: EncodePicture,
    /// NvEncGetEncodeStats
    pub(crate) get_encode_stats: GetEncodeStats,
    /// NvEncGetSequenceParams
    pub(crate) get_sequence_params: GetSequenceParams,
    /// NvEncGetSequenceParamEx
    pub(crate) get_sequence_param_ex: GetSequenceParamEx,
    /// NvEncRegisterAsyncEvent
    pub(crate) register_async_event: RegisterAsyncEvent,
    /// NvEncUnregisterAsyncEvent
    pub(crate) unregister_async_event: UnregisterAsyncEvent,
    /// NvEncInvalidateRefFrames
    pub(crate) invalidate_ref_frames: InvalidateRefFrames,
    /// NvEncRunMotionEstimationOnly
    pub(crate) run_motion_estimation_only: RunMotionEstimationOnly,
    /// NvEncGetLastErrorString
    pub(crate) get_last_error_string: GetLastErrorString,
    /// NvEncSetIOCudaStreams
    pub(crate) set_io_cuda_streams: SetIOCudaStreams,
}

fn assert_versions_match(max_supported_version: u32) {
    assert!(
        max_supported_version >= NVENCAPI_MAJOR_VERSION << 4 | (NVENCAPI_MINOR_VERSION & 0b1111),
        "The maximum supported version should be greater or equal than the header version."
    );
}

impl EncodeAPI {
    fn new() -> Self {
        const MSG: &str = "The API instance should populate the whole function list";

        // Check that the driver max supported version matches the version
        // from the header files. If they do not match, the bindings should be updated.
        let mut version = MaybeUninit::uninit();
        unsafe { NvEncodeAPIGetMaxSupportedVersion(version.as_mut_ptr()) }
            .result()
            .expect("The pointer to the version should be valid.");
        assert_versions_match(unsafe { version.assume_init() });

        // Create empty function buffer.
        let mut function_list = NV_ENCODE_API_FUNCTION_LIST {
            version: NV_ENCODE_API_FUNCTION_LIST_VER,
            ..Default::default()
        };
        // Create Encode API Instance (populate function buffer).
        unsafe { NvEncodeAPICreateInstance(&mut function_list) }
            .result()
            .expect("The pointer to the function list should be valid.");

        Self {
            open_encode_session: function_list.nvEncOpenEncodeSession.expect(MSG),
            open_encode_session_ex: function_list.nvEncOpenEncodeSessionEx.expect(MSG),
            initialize_encoder: function_list.nvEncInitializeEncoder.expect(MSG),
            reconfigure_encoder: function_list.nvEncReconfigureEncoder.expect(MSG),
            destroy_encoder: function_list.nvEncDestroyEncoder.expect(MSG),
            get_encode_guid_count: function_list.nvEncGetEncodeGUIDCount.expect(MSG),
            get_encode_guids: function_list.nvEncGetEncodeGUIDs.expect(MSG),
            get_encode_profile_guid_count: function_list.nvEncGetEncodeProfileGUIDCount.expect(MSG),
            get_encode_profile_guids: function_list.nvEncGetEncodeProfileGUIDs.expect(MSG),
            get_input_format_count: function_list.nvEncGetInputFormatCount.expect(MSG),
            get_input_formats: function_list.nvEncGetInputFormats.expect(MSG),
            get_encode_preset_count: function_list.nvEncGetEncodePresetCount.expect(MSG),
            get_encode_preset_guids: function_list.nvEncGetEncodePresetGUIDs.expect(MSG),
            get_encode_preset_config: function_list.nvEncGetEncodePresetConfig.expect(MSG),
            get_encode_preset_config_ex: function_list.nvEncGetEncodePresetConfigEx.expect(MSG),
            get_encode_caps: function_list.nvEncGetEncodeCaps.expect(MSG),
            create_input_buffer: function_list.nvEncCreateInputBuffer.expect(MSG),
            destroy_input_buffer: function_list.nvEncDestroyInputBuffer.expect(MSG),
            lock_input_buffer: function_list.nvEncLockInputBuffer.expect(MSG),
            unlock_input_buffer: function_list.nvEncUnlockInputBuffer.expect(MSG),
            create_bitstream_buffer: function_list.nvEncCreateBitstreamBuffer.expect(MSG),
            destroy_bitstream_buffer: function_list.nvEncDestroyBitstreamBuffer.expect(MSG),
            lock_bitstream: function_list.nvEncLockBitstream.expect(MSG),
            unlock_bitstream: function_list.nvEncUnlockBitstream.expect(MSG),
            map_input_resource: function_list.nvEncMapInputResource.expect(MSG),
            unmap_input_resource: function_list.nvEncUnmapInputResource.expect(MSG),
            register_resource: function_list.nvEncRegisterResource.expect(MSG),
            unregister_resource: function_list.nvEncUnregisterResource.expect(MSG),
            create_mv_buffer: function_list.nvEncCreateMVBuffer.expect(MSG),
            destroy_mv_buffer: function_list.nvEncDestroyMVBuffer.expect(MSG),
            encode_picture: function_list.nvEncEncodePicture.expect(MSG),
            get_encode_stats: function_list.nvEncGetEncodeStats.expect(MSG),
            get_sequence_params: function_list.nvEncGetSequenceParams.expect(MSG),
            get_sequence_param_ex: function_list.nvEncGetSequenceParamEx.expect(MSG),
            register_async_event: function_list.nvEncRegisterAsyncEvent.expect(MSG),
            unregister_async_event: function_list.nvEncUnregisterAsyncEvent.expect(MSG),
            invalidate_ref_frames: function_list.nvEncInvalidateRefFrames.expect(MSG),
            run_motion_estimation_only: function_list.nvEncRunMotionEstimationOnly.expect(MSG),
            get_last_error_string: function_list.nvEncGetLastErrorString.expect(MSG),
            set_io_cuda_streams: function_list.nvEncSetIOCudaStreams.expect(MSG),
        }
    }
}
