//! Constants from `nvEncodeAPI` that bindgen fails to generate.

#[allow(clippy::must_use_candidate)]
#[allow(non_snake_case)]
/// Macro to generate per-structure version for use with API.
pub const fn NVENCAPI_STRUCT_VERSION(ver: u32) -> u32 {
    super::nvEncodeAPI::NVENCAPI_VERSION | (ver << 16) | (0x7 << 28)
}

#[allow(missing_docs)]
pub const NV_ENC_CAPS_PARAM_VER: u32 = NVENCAPI_STRUCT_VERSION(1);
#[allow(missing_docs)]
pub const NV_ENC_ENCODE_OUT_PARAMS_VER: u32 = NVENCAPI_STRUCT_VERSION(1);
#[allow(missing_docs)]
pub const NV_ENC_CREATE_INPUT_BUFFER_VER: u32 = NVENCAPI_STRUCT_VERSION(1);
#[allow(missing_docs)]
pub const NV_ENC_CREATE_BITSTREAM_BUFFER_VER: u32 = NVENCAPI_STRUCT_VERSION(1);
#[allow(missing_docs)]
pub const NV_ENC_CREATE_MV_BUFFER_VER: u32 = NVENCAPI_STRUCT_VERSION(1);
#[allow(missing_docs)]
pub const NV_ENC_RC_PARAMS_VER: u32 = NVENCAPI_STRUCT_VERSION(1);
#[allow(missing_docs)]
pub const NV_ENC_CONFIG_VER: u32 = NVENCAPI_STRUCT_VERSION(8) | (1 << 31);
#[allow(missing_docs)]
pub const NV_ENC_INITIALIZE_PARAMS_VER: u32 = NVENCAPI_STRUCT_VERSION(5) | (1 << 31);
#[allow(missing_docs)]
pub const NV_ENC_RECONFIGURE_PARAMS_VER: u32 = NVENCAPI_STRUCT_VERSION(1) | (1 << 31);
#[allow(missing_docs)]
pub const NV_ENC_PRESET_CONFIG_VER: u32 = NVENCAPI_STRUCT_VERSION(4) | (1 << 31);
#[allow(missing_docs)]
pub const NV_ENC_PIC_PARAMS_MVC_VER: u32 = NVENCAPI_STRUCT_VERSION(1);
#[allow(missing_docs)]
pub const NV_ENC_PIC_PARAMS_VER: u32 = NVENCAPI_STRUCT_VERSION(6) | (1 << 31);
#[allow(missing_docs)]
pub const NV_ENC_MEONLY_PARAMS_VER: u32 = NVENCAPI_STRUCT_VERSION(3);
#[allow(missing_docs)]
pub const NV_ENC_LOCK_BITSTREAM_VER: u32 = NVENCAPI_STRUCT_VERSION(2);
#[allow(missing_docs)]
pub const NV_ENC_LOCK_INPUT_BUFFER_VER: u32 = NVENCAPI_STRUCT_VERSION(1);
#[allow(missing_docs)]
pub const NV_ENC_MAP_INPUT_RESOURCE_VER: u32 = NVENCAPI_STRUCT_VERSION(4);
#[allow(missing_docs)]
pub const NV_ENC_FENCE_POINT_D3D12_VER: u32 = NVENCAPI_STRUCT_VERSION(1);
#[allow(missing_docs)]
pub const NV_ENC_INPUT_RESOURCE_D3D12_VER: u32 = NVENCAPI_STRUCT_VERSION(1);
#[allow(missing_docs)]
pub const NV_ENC_OUTPUT_RESOURCE_D3D12_VER: u32 = NVENCAPI_STRUCT_VERSION(1);
#[allow(missing_docs)]
pub const NV_ENC_REGISTER_RESOURCE_VER: u32 = NVENCAPI_STRUCT_VERSION(4);
#[allow(missing_docs)]
pub const NV_ENC_STAT_VER: u32 = NVENCAPI_STRUCT_VERSION(1);
#[allow(missing_docs)]
pub const NV_ENC_SEQUENCE_PARAM_PAYLOAD_VER: u32 = NVENCAPI_STRUCT_VERSION(1);
#[allow(missing_docs)]
pub const NV_ENC_EVENT_PARAMS_VER: u32 = NVENCAPI_STRUCT_VERSION(1);
#[allow(missing_docs)]
pub const NV_ENC_OPEN_ENCODE_SESSION_EX_PARAMS_VER: u32 = NVENCAPI_STRUCT_VERSION(1);
#[allow(missing_docs)]
pub const NV_ENCODE_API_FUNCTION_LIST_VER: u32 = NVENCAPI_STRUCT_VERSION(2);
