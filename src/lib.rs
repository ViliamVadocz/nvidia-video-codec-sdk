#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

// include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub mod cuviddec;
pub mod nvcuvid;
pub mod nvencodeapi;

#[cfg(test)]
mod tests {
    use crate::nvencodeapi::*;
    use std::os::raw::c_void;

    #[test]
    fn example() {
        let ver = 2;
        unsafe {
            let mut function_list = NV_ENCODE_API_FUNCTION_LIST {
                version: (NVENCAPI_VERSION | ((ver) << 16) | (0x7 << 28)),
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

            NvEncodeAPICreateInstance(&mut function_list as *mut NV_ENCODE_API_FUNCTION_LIST);

            let device = CudaDevice::new(0);

            let nvEncOpenEncodeSessionEx = function_list.nvEncOpenEncodeSessionEx.unwrap();
            nvEncOpenEncodeSessionEx()
        }
    }
}
