pub mod sys;

pub fn NVENCAPI_STRUCT_VERSION(ver: i32) -> i32 {
    use crate::nvEncodeAPI::NVENCAPI_VERSION;
    (NVENCAPI_VERSION | ((ver) << 16) | (0x7 << 28))
}

#[cfg(test)]
mod tests {
    use crate::{sys::nvEncodeAPI::*, NVENCAPI_STRUCT_VERSION};
    use cudarc::driver::sys::*;
    use std::{ffi::c_int, ffi::{c_void, c_char, c_uint}};

    #[test]
    fn example() {
        unsafe {
            let mut function_list = NV_ENCODE_API_FUNCTION_LIST {
                version: NVENCAPI_STRUCT_VERSION(2),
                reserved: 0,
                nvEncOpenEncodeSession: PNVENCOPENENCODESESSION::None,
                nvEncGetEncodeGUIDCount: PNVENCGETENCODEGUIDCOUNT::None,
                nvEncGetEncodeProfileGUIDCount: PNVENCGETENCODEPRESETCOUNT::None,
                nvEncGetEncodeProfileGUIDs: PNVENCGETENCODEPRESETGUIDS::None,
                nvEncGetEncodeGUIDs: PNVENCGETENCODEGUIDS::None,
                nvEncGetInputFormatCount: PNVENCGETINPUTFORMATCOUNT::None,
                nvEncGetInputFormats: PNVENCGETINPUTFORMATS::None,
                nvEncGetEncodeCaps: PNVENCGETENCODECAPS::None,
                nvEncGetEncodePresetCount: PNVENCGETENCODEPRESETCOUNT::None,
                nvEncGetEncodePresetGUIDs: PNVENCGETENCODEPRESETGUIDS::None,
                nvEncGetEncodePresetConfig: PNVENCGETENCODEPRESETCONFIG::None,
                nvEncInitializeEncoder: PNVENCINITIALIZEENCODER::None,
                nvEncCreateInputBuffer: PNVENCCREATEINPUTBUFFER::None,
                nvEncDestroyInputBuffer: PNVENCDESTROYINPUTBUFFER::None,
                nvEncCreateBitstreamBuffer: PNVENCCREATEBITSTREAMBUFFER::None,
                nvEncDestroyBitstreamBuffer: PNVENCDESTROYBITSTREAMBUFFER::None,
                nvEncEncodePicture: PNVENCENCODEPICTURE::None,
                nvEncLockBitstream: PNVENCLOCKBITSTREAM::None,
                nvEncUnlockBitstream: PNVENCUNLOCKBITSTREAM::None,
                nvEncLockInputBuffer: PNVENCLOCKINPUTBUFFER::None,
                nvEncUnlockInputBuffer: PNVENCUNLOCKINPUTBUFFER::None,
                nvEncGetEncodeStats: PNVENCGETENCODESTATS::None,
                nvEncGetSequenceParams: PNVENCGETSEQUENCEPARAMS::None,
                nvEncRegisterAsyncEvent: PNVENCREGISTERASYNCEVENT::None,
                nvEncUnregisterAsyncEvent: PNVENCUNREGISTERASYNCEVENT::None,
                nvEncMapInputResource: PNVENCMAPINPUTRESOURCE::None,
                nvEncUnmapInputResource: PNVENCUNMAPINPUTRESOURCE::None,
                nvEncDestroyEncoder: PNVENCDESTROYENCODER::None,
                nvEncInvalidateRefFrames: PNVENCINVALIDATEREFFRAMES::None,
                nvEncOpenEncodeSessionEx: PNVENCOPENENCODESESSIONEX::None,
                nvEncRegisterResource: PNVENCREGISTERRESOURCE::None,
                nvEncUnregisterResource: PNVENCUNREGISTERRESOURCE::None,
                nvEncReconfigureEncoder: PNVENCRECONFIGUREENCODER::None,
                reserved1: std::ptr::null::<c_void>() as *mut c_void,
                nvEncCreateMVBuffer: PNVENCCREATEMVBUFFER::None,
                nvEncDestroyMVBuffer: PNVENCDESTROYMVBUFFER::None,
                nvEncRunMotionEstimationOnly: PNVENCRUNMOTIONESTIMATIONONLY::None,
                nvEncGetLastErrorString: PNVENCGETLASTERROR::None,
                nvEncSetIOCudaStreams: PNVENCSETIOCUDASTREAMS::None,
                nvEncGetEncodePresetConfigEx: PNVENCGETENCODEPRESETCONFIGEX::None,
                nvEncGetSequenceParamEx: PNVENCGETSEQUENCEPARAMEX::None,
                reserved2: [std::ptr::null::<c_void>() as *mut c_void; 277],
            };

            cuInit(0);
            let mut nGpu = 0;
            let iGpu = 0;
            cuDeviceGetCount(&mut nGpu as *mut c_int);
            let mut cuDevice = 0;
            cuDeviceGet(&mut cuDevice as CUdevice, iGpu as c_int);
            let mut szDeviceName: [c_char; 80];
            cuDeviceGetName(&mut szDeviceName as *mut c_char, szDeviceName.len() as c_int, cuDevice as CUdevice);
            let mut cuContext = std::ptr::null::<c_void>();
            cuCtxCreate_v2(&mut cuContext as *mut CUcontext, 0 as c_uint, cuDevice as CUdevice);
            
            

            NvEncodeAPICreateInstance(&mut function_list as *mut NV_ENCODE_API_FUNCTION_LIST);

            use crate::nvEncodeAPI::NVENCAPI_VERSION; //TODO check with will if it can go inside

            let session_params = NV_ENC_OPEN_ENCODE_SESSION_EX_PARAMS {
                version: NVENCAPI_STRUCT_VERSION(1),
                deviceType: NV_ENC_DEVICE_TYPE::NV_ENC_DEVICE_TYPE_CUDA,
                apiVersion: nvEncodeAPI::NVENCAPI_VERSION,
                device,
                reserved: 0,
                reserved1: 0,
                reserved2: [std::ptr::null::<c_void>() as *mut c_void; 64],
            };

            let nvEncOpenEncodeSessionEx = function_list.nvEncOpenEncodeSessionEx.unwrap();
            let status = nvEncOpenEncodeSessionEx(&mut session_params as *mut _NV_ENC_OPEN_ENCODE_SESSIONEX_PARAMS, &mut cuContext as *mut *mut c_void);

            if status != _NVENCSTATUS::NV_ENC_SUCCESS {
                //If the creation of encoder session fails, the client must call ::NvEncDestroyEncoder API before exiting.
            }

            let nvEncGetEncodeGUIDCount = function_list.nvEncGetEncodeGUIDCount.unwrap();
            let mut supportedGuidsCount:u32 = 0;
            let _res = nvEncGetEncodeGUIDCount(&mut function_list as *mut c_void, &mut supportedGuidsCount as *mut u32);
        }
    }
}
