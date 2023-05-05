pub mod sys;

#[cfg(test)]
mod tests {
    use crate::{sys::nvEncodeAPI::*, NVENCAPI_STRUCT_VERSION};
    use cudarc::driver::sys::*;
    use std::{
        ffi::c_int,
        ffi::{c_char, c_uint, c_void},
    };

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
            cuDeviceGetName(
                &mut szDeviceName as *mut c_char,
                szDeviceName.len() as c_int,
                cuDevice as CUdevice,
            );
            let mut cuContext = std::ptr::null::<c_void>();
            cuCtxCreate_v2(
                &mut cuContext as *mut CUcontext,
                0 as c_uint,
                cuDevice as CUdevice,
            );

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
            let status = nvEncOpenEncodeSessionEx(
                &mut session_params as *mut _NV_ENC_OPEN_ENCODE_SESSIONEX_PARAMS,
                &mut cuContext as *mut *mut c_void,
            );

            if status != _NVENCSTATUS::NV_ENC_SUCCESS {
                //If the creation of encoder session fails, the client must call ::NvEncDestroyEncoder API before exiting.
            }

            let nvEncGetEncodeGUIDCount = function_list.nvEncGetEncodeGUIDCount.unwrap();
            let mut supportedGuidsCount: u32 = 0;
            let _ = nvEncGetEncodeGUIDCount(
                &mut function_list as *mut c_void,
                &mut supportedGuidsCount as *mut u32,
            );

            let nvEncGetEncodeGUIDs = function_list.nvEncGetEncodeGUIDs.unwrap();
            let mut GUIDCount: u32 = 0;
            let mut encodeGUIDs = vec![GUID; supportedGuidsCount];
            nvEncGetEncodeGUIDs(
                &mut function_list as *mut c_void,
                encodeGUIDs.as_mut_ptr() as *mut _GUID,
                supportedGuidsCount,
                &mut GUIDCount as *mut u32,
            );
            //TODO: check using GUIDCount
            let encodeGUID = encodeGUIDs
                .first()
                .expect("There should be at least 1 encode GUID");

            let nvEncGetEncodePresetCount = function_list.nvEncGetEncodePresetCount.unwrap();
            let mut encodePresetGUIDCount: u32 = 0;
            nvEncGetEncodePresetCount(
                &mut function_list as *mut c_void,
                encodeGUID,
                &mut encodePresetGUIDCount as *mut u32,
            );

            let nvEncGetEncodePresetGUIDs = function_list.nvEncGetEncodePresetGUIDs.unwrap();
            let mut guidArraySize: u32 = 0;
            let mut presetGUIDs = vec![GUID; encodePresetGUIDCount];
            nvEncGetEncodePresetGUIDs(
                &mut function_list as *mut c_void,
                encodeGUID,
                presetGUIDs.as_mut_ptr() as *mut _GUID,
                encodePresetGUIDCount,
                &mut guidArraySize as *mut u32,
            );

            //TODO: check using guidArraySize
            let presetGUID = presetGUIDs
                .first()
                .expect("There should be at least 1 preset GUID");

            let nvEncGetEncodePresetConfigEx = function_list.nvEncGetEncodePresetConfigEx.unwrap();
            let mut presetConfig = NV_ENC_PRESET_CONFIG;
            nvEncGetEncodePresetConfigEx(
                &mut function_list as *mut c_void,
                encodeGUID,
                presetGUID,
                NV_ENC_TUNING_INFO::NV_ENC_TUNING_INFO_LOW_LATENCY,
                &mut presetConfig as *mut _NV_ENC_PRESET_CONFIG,
            );

            let nvEncInitializeEncoder = function_list.nvEncInitializeEncoder.unwrap();
            nvEncInitializeEncoder(
                &mut function_list as *mut c_void,
                &mut presetConfig as *mut _NV_ENC_PRESET_CONFIG,
            );

            //Encoding the stream
            //Allocate buffer
            let nvEncRegisterResource = function_list.nvEncRegisterResource.unwrap();
            //TODO set resource params
            let mut registerResParams = NV_ENC_REGISTER_RESOURCE {};
            nvEncRegisterResource(
                &mut function_list as *mut c_void,
                &mut registerResParams as *mut _NV_ENC_REGISTER_RESOURCE,
            );

            let nvEncMapInputResource = function_list.nvEncMapInputResource.unwrap();
            //TODO check if param is correct
            let mapInputResParams = NV_ENC_MAP_INPUT_RESOURCE {
                registeredResource: registerResParams.registeredResource,
            };
            nvEncMapInputResource(
                &mut function_list as *mut c_void,
                &mut mapInputResParams as *mut _NV_ENC_MAP_INPUT_RESOURCE,
            );

            //Submitting
            //TODO set pic params
            let mut picParams = NV_ENC_PIC_PARAMS {
                inputBuffer: mapInputResParams.mappedResource,
            };

            let nvEncEncodePicture = function_list.nvEncEncodePicture.unwrap();
            nvEncEncodePicture(
                &mut function_list as *mut c_void,
                &mut picParams as *mut _NV_ENC_PIC_PARAMS,
            );

            let nvEncLockBitstream = function_list.nvEncLockBitstream.unwrap();
            //TODO set bitstream buffer params
            let mut lockBitstreamBufferParams = NV_ENC_LOCK_BITSTREAM;
            nvEncLockBitstream(
                &mut function_list as *mut c_void,
                &mut lockBitstreamBufferParams as *mut _NV_ENC_LOCK_BITSTREAM,
            );

            // Start destroy
            let nvEncUnmapInputResource = function_list.nvEncUnmapInputResource.unwrap();
            nvEncUnmapInputResource(&mut function_list as *mut c_void);

            let nvEncUnregisterResource = function_list.nvEncUnregisterResource.unwrap();
            nvEncUnregisterResource(
                &mut function_list as *mut c_void,
                &mut registerResParams.registeredResource,
            );

            let mut picParamsEnd = NV_ENC_PIC_PARAMS {
                version: 0,
                inputWidth: 0,
                inputHeight: 0,
                inputPitch: 0,
                encodePicFlags: NV_ENC_PIC_FLAGS::NV_ENC_PIC_FLAG_EOS,
                frameIdx: 0,
                inputTimeStamp: 0,
                inputDuration: 0,
                inputBuffer: 0,
                outputBitstream: 0,
                completionEvent: 0,
                bufferFmt: 0,
                pictureStruct: 0,
                pictureType: 0,
                codecPicParams: 0,
                meHintCountsPerBlock: 0,
                meExternalHints: 0,
                reserved1: 0,
                reserved2: 0,
                qpDeltaMap: 0,
                qpDeltaMapSize: 0,
                reservedBitFields: 0,
                meHintRefPicDist: 0,
                alphaBuffer: 0,
                meExternalSbHints: 0,
                meSbHintsCount: 0,
                reserved3: 0,
                reserved4: 0,
            };

            nvEncEncodePicture(
                &mut function_list as *mut c_void,
                &mut picParamsEnd as *mut _NV_ENC_PIC_PARAMS,
            );

            let nvEncUnlockBitstream = function_list.nvEncUnlockBitstream.unwrap();
            nvEncUnlockBitstream(
                &mut function_list as *mut c_void,
                &mut lockBitstreamBufferParams as *mut c_void,
            );

            let nvEncDestroyBitstreamBuffer = function_list.nvEncDestroyBitstreamBuffer.unwrap();
            // Not clear if there parameters should be lockBitstreamBufferParams or lockBitstreamBufferParams.bitstreamBufferPtr
            nvEncDestroyBitstreamBuffer(
                &mut function_list as *mut c_void,
                &mut lockBitstreamBufferParams as *mut c_void,
            );

            let nvEncDestroyEncoder = function_list.nvEncDestroyEncoder.unwrap();
            nvEncDestroyEncoder(&mut function_list as *mut c_void);
        }
    }
}
