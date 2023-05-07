pub mod sys;

#[cfg(test)]
mod tests {
    use crate::sys::nvEncodeAPI::*;
    use cudarc::driver::sys::*;
    use std::{
        ffi::{c_char, c_int, c_uint, c_void},
        ptr,
    };

    #[allow(non_snake_case)]
    #[test]
    fn example() {
        const WIDTH: u32 = 1920;
        const HEIGHT: u32 = 1080;

        // TODO: Destroy encoding session on error.
        unsafe {
            // 3.1. Opening and Encode Session

            // Initialize Cuda Context. (TODO: Handle errors safely.)
            cuInit(0);
            let mut nGpu = 0;
            let iGpu = 0;
            cuDeviceGetCount(&mut nGpu as *mut c_int);
            let mut cuDevice = 0;
            cuDeviceGet(&mut cuDevice as *mut CUdevice, iGpu as c_int);
            let mut szDeviceName: [c_char; 80] = [0; 80];
            cuDeviceGetName(
                &mut szDeviceName as *mut c_char,
                szDeviceName.len() as c_int,
                cuDevice as CUdevice,
            );
            let mut cuContext = ptr::null_mut();
            cuCtxCreate_v2(
                &mut cuContext as *mut CUcontext,
                0 as c_uint,
                cuDevice as CUdevice,
            );

            // Create empty function buffer.
            let mut function_list = NV_ENCODE_API_FUNCTION_LIST {
                version: NV_ENCODE_API_FUNCTION_LIST_VER,
                ..Default::default()
            };

            // Create Encode API Instance (populate function buffer).
            NvEncodeAPICreateInstance(&mut function_list as *mut NV_ENCODE_API_FUNCTION_LIST);

            // Retrieve all functions.
            // let nvEncOpenEncodeSession = function_list.nvEncOpenEncodeSession.unwrap();
            let nvEncGetEncodeGUIDCount = function_list.nvEncGetEncodeGUIDCount.unwrap();
            let nvEncGetEncodeProfileGUIDCount =
                function_list.nvEncGetEncodeProfileGUIDCount.unwrap();
            let nvEncGetEncodeProfileGUIDs = function_list.nvEncGetEncodeProfileGUIDs.unwrap();
            let nvEncGetEncodeGUIDs = function_list.nvEncGetEncodeGUIDs.unwrap();
            let nvEncGetInputFormatCount = function_list.nvEncGetInputFormatCount.unwrap();
            let nvEncGetInputFormats = function_list.nvEncGetInputFormats.unwrap();
            // let nvEncGetEncodeCaps = function_list.nvEncGetEncodeCaps.unwrap();
            let nvEncGetEncodePresetCount = function_list.nvEncGetEncodePresetCount.unwrap();
            let nvEncGetEncodePresetGUIDs = function_list.nvEncGetEncodePresetGUIDs.unwrap();
            // let nvEncGetEncodePresetConfig = function_list.nvEncGetEncodePresetConfig.unwrap();
            let nvEncInitializeEncoder = function_list.nvEncInitializeEncoder.unwrap();
            let nvEncCreateInputBuffer = function_list.nvEncCreateInputBuffer.unwrap();
            let nvEncDestroyInputBuffer = function_list.nvEncDestroyInputBuffer.unwrap();
            let nvEncCreateBitstreamBuffer = function_list.nvEncCreateBitstreamBuffer.unwrap();
            let nvEncDestroyBitstreamBuffer = function_list.nvEncDestroyBitstreamBuffer.unwrap();
            let nvEncEncodePicture = function_list.nvEncEncodePicture.unwrap();
            let nvEncLockBitstream = function_list.nvEncLockBitstream.unwrap();
            let nvEncUnlockBitstream = function_list.nvEncUnlockBitstream.unwrap();
            let nvEncLockInputBuffer = function_list.nvEncLockInputBuffer.unwrap();
            let nvEncUnlockInputBuffer = function_list.nvEncUnlockInputBuffer.unwrap();
            // let nvEncGetEncodeStats = function_list.nvEncGetEncodeStats.unwrap();
            // let nvEncGetSequenceParams = function_list.nvEncGetSequenceParams.unwrap();
            // let nvEncRegisterAsyncEvent = function_list.nvEncRegisterAsyncEvent.unwrap();
            // let nvEncUnregisterAsyncEvent = function_list.nvEncUnregisterAsyncEvent.unwrap();
            // let nvEncMapInputResource = function_list.nvEncMapInputResource.unwrap();
            // let nvEncUnmapInputResource = function_list.nvEncUnmapInputResource.unwrap();
            let nvEncDestroyEncoder = function_list.nvEncDestroyEncoder.unwrap();
            // let nvEncInvalidateRefFrames = function_list.nvEncInvalidateRefFrames.unwrap();
            let nvEncOpenEncodeSessionEx = function_list.nvEncOpenEncodeSessionEx.unwrap();
            // let nvEncRegisterResource = function_list.nvEncRegisterResource.unwrap();
            // let nvEncUnregisterResource = function_list.nvEncUnregisterResource.unwrap();
            // let nvEncReconfigureEncoder = function_list.nvEncReconfigureEncoder.unwrap();
            // let nvEncCreateMVBuffer = function_list.nvEncCreateMVBuffer.unwrap();
            // let nvEncDestroyMVBuffer = function_list.nvEncDestroyMVBuffer.unwrap();
            // let nvEncRunMotionEstimationOnly = function_list.nvEncRunMotionEstimationOnly.unwrap();
            // let nvEncGetLastErrorString = function_list.nvEncGetLastErrorString.unwrap();
            // let nvEncSetIOCudaStreams = function_list.nvEncSetIOCudaStreams.unwrap();
            let nvEncGetEncodePresetConfigEx = function_list.nvEncGetEncodePresetConfigEx.unwrap();
            // let nvEncGetSequenceParamEx = function_list.nvEncGetSequenceParamEx.unwrap();

            // Begin encoding session.
            let mut encoder = ptr::null_mut();
            let mut session_params = NV_ENC_OPEN_ENCODE_SESSION_EX_PARAMS {
                version: NVENCAPI_STRUCT_VERSION(1),
                deviceType: NV_ENC_DEVICE_TYPE::NV_ENC_DEVICE_TYPE_CUDA,
                apiVersion: NVENCAPI_VERSION,
                device: cuContext as *mut c_void, // Pass the CUDA Context as the device.
                ..Default::default()
            };
            nvEncOpenEncodeSessionEx(
                &mut session_params as *mut _NV_ENC_OPEN_ENCODE_SESSIONEX_PARAMS,
                &mut encoder as *mut *mut c_void,
            );

            // 3.2. Selecting Encoder Codec GUID

            // Query number of supported encoder codec GUIDs.
            let mut supported_GUID_count = 0;
            nvEncGetEncodeGUIDCount(encoder, &mut supported_GUID_count as *mut u32);
            // Get the supported GUIDs.
            let mut encode_GUIDs = vec![GUID::default(); supported_GUID_count as usize];
            let mut actual_GUID_count: u32 = 0;
            nvEncGetEncodeGUIDs(
                encoder,
                encode_GUIDs.as_mut_ptr(),
                supported_GUID_count,
                &mut actual_GUID_count as *mut u32,
            );
            println!(
                "encode GUIDs: {:?}",
                &encode_GUIDs[..actual_GUID_count as usize]
            );
            // Just pick first one. (TODO: Figure out which to take.)
            let encode_GUID = encode_GUIDs
                .into_iter()
                .take(actual_GUID_count as usize)
                .next()
                .expect("There should be at least 1 encode GUID."); // TODO: Destroy encode session

            // 3.3. Encoder Tuning Info and Preset Configurations
            // 3.3.1. Enumerating Preset GUIDs.

            // Query number of preset GUIDs.
            let mut encode_preset_GUID_count = 0;
            nvEncGetEncodePresetCount(
                encoder,
                encode_GUID,
                &mut encode_preset_GUID_count as *mut u32,
            );
            // Get the preset GUIDs.
            let mut actual_preset_GUID_count: u32 = 0;
            let mut preset_GUIDs = vec![GUID::default(); encode_preset_GUID_count as usize];
            nvEncGetEncodePresetGUIDs(
                encoder,
                encode_GUID,
                preset_GUIDs.as_mut_ptr() as *mut _GUID,
                encode_preset_GUID_count,
                &mut actual_preset_GUID_count as *mut u32,
            );
            println!(
                "preset GUIDs: {:?}",
                &preset_GUIDs[..actual_preset_GUID_count as usize]
            );
            // Just get the first one. (TODO: Figure out which to take.)
            let preset_GUID = preset_GUIDs
                .into_iter()
                .take(actual_preset_GUID_count as usize)
                .next()
                .expect("There should be at least 1 preset GUID."); // TODO: Destroy encode session

            // 3.3.2. Selecting encoder preset configuration

            let mut preset_config = NV_ENC_PRESET_CONFIG::default();
            nvEncGetEncodePresetConfigEx(
                encoder,
                encode_GUID,
                preset_GUID,
                NV_ENC_TUNING_INFO::NV_ENC_TUNING_INFO_LOW_LATENCY,
                &mut preset_config as *mut _NV_ENC_PRESET_CONFIG,
            );

            assert_eq!(preset_config.version, NV_ENC_PRESET_CONFIG_VER);
            assert_eq!(preset_config.presetCfg.version, NV_ENC_CONFIG_VER);

            // TODO: modify preset as required.

            // 3.4. Selecting an Encoder Profile

            // Query the number of encoder profile GUIDs.
            let mut profile_GUID_count = 0;
            nvEncGetEncodeProfileGUIDCount(
                encoder,
                encode_GUID,
                &mut profile_GUID_count as *mut u32,
            );
            // Get the profile GUIDs.
            let mut profile_GUIDs = vec![GUID::default(); profile_GUID_count as usize];
            let mut actual_profile_GUID_count: u32 = 0;
            nvEncGetEncodeProfileGUIDs(
                encoder,
                encode_GUID,
                profile_GUIDs.as_mut_ptr(),
                profile_GUID_count,
                &mut actual_profile_GUID_count as *mut u32,
            );
            println!(
                "profile GUIDs: {:?}",
                &profile_GUIDs[..actual_profile_GUID_count as usize]
            );
            // Just pick first one. (TODO: Figure out which to take.)
            let _profile_GUID = profile_GUIDs
                .into_iter()
                .take(actual_profile_GUID_count as usize)
                .next()
                .expect("There should be at least 1 profile GUID."); // TODO: Destroy encode session

            // 3.5. Getting Supported List of Input Formats

            // Query the number of supported input formats.
            let mut supported_format_count = 0;
            nvEncGetInputFormatCount(
                encoder,
                encode_GUID,
                &mut supported_format_count as *mut u32,
            );
            // Get the supported formats.
            let mut supported_formats = vec![
                NV_ENC_BUFFER_FORMAT::NV_ENC_BUFFER_FORMAT_UNDEFINED;
                supported_format_count as usize
            ];
            let mut actual_format_count: u32 = 0;
            nvEncGetInputFormats(
                encoder,
                encode_GUID,
                supported_formats.as_mut_ptr(),
                supported_format_count,
                &mut actual_format_count as *mut u32,
            );
            println!(
                "supported formats: {:?}",
                &supported_formats[..actual_format_count as usize]
            );
            let buffer_format = supported_formats
                .into_iter()
                .take(actual_format_count as usize)
                .next()
                .expect("There should be at least 1 supported format.");

            // 3.6. Querying encoder Capabilities
            // TODO: idk

            // 3.7. Initializing the Hardware Encoder Session

            let mut initialize_params = NV_ENC_INITIALIZE_PARAMS {
                version: NV_ENC_INITIALIZE_PARAMS_VER,
                encodeGUID: encode_GUID,
                // presetGUID: preset_GUID,
                encodeWidth: WIDTH,
                encodeHeight: HEIGHT,
                darWidth: 16,
                darHeight: 9,
                frameRateNum: 30,
                frameRateDen: 1,
                enableEncodeAsync: 0, // We want synchronous mode.
                enablePTD: 1,         // 3.8.5.2 Picture-type decision.
                // encodeConfig: &mut preset_config.presetCfg as *mut NV_ENC_CONFIG,
                ..Default::default()
            };
            // TODO: Consider further options that are in bitfields
            // example: initialize_params.set_enableWeightedPrediction(1);
            nvEncInitializeEncoder(
                encoder,
                &mut initialize_params as *mut NV_ENC_INITIALIZE_PARAMS,
            );

            // 3.9. Creating Resources Required to Hold Input/output Data

            // Allocate input buffer.
            let mut create_input_buffer_params = NV_ENC_CREATE_INPUT_BUFFER {
                version: NV_ENC_CREATE_INPUT_BUFFER_VER,
                width: WIDTH,
                height: HEIGHT,
                bufferFmt: buffer_format,
                inputBuffer: ptr::null_mut(),
                pSysMemBuffer: ptr::null_mut(), // TODO: How to make a system memory buffer?
                ..Default::default()
            };
            nvEncCreateInputBuffer(
                encoder,
                &mut create_input_buffer_params as &mut NV_ENC_CREATE_INPUT_BUFFER,
            );
            let input_buffer = create_input_buffer_params.inputBuffer;

            // Allocate output buffer.
            let mut create_bitstream_buffer_params = NV_ENC_CREATE_BITSTREAM_BUFFER {
                version: NV_ENC_CREATE_BITSTREAM_BUFFER_VER,
                bitstreamBuffer: ptr::null_mut(),
                ..Default::default()
            };
            nvEncCreateBitstreamBuffer(
                encoder,
                &mut create_bitstream_buffer_params as *mut NV_ENC_CREATE_BITSTREAM_BUFFER,
            );
            let output_buffer = create_bitstream_buffer_params.bitstreamBuffer;

            // 4.1. Preparing Input Buffers for Encoding

            // Lock input buffer.
            let mut lock_input_buffer_params = NV_ENC_LOCK_INPUT_BUFFER {
                version: NV_ENC_LOCK_INPUT_BUFFER_VER,
                inputBuffer: input_buffer,
                ..Default::default()
            };
            nvEncLockInputBuffer(
                encoder,
                &mut lock_input_buffer_params as *mut NV_ENC_LOCK_INPUT_BUFFER,
            );
            let _input_buffer_data = lock_input_buffer_params.bufferDataPtr;

            // TODO: Fill input buffer with data.

            // Unlock input buffer.
            nvEncUnlockInputBuffer(encoder, input_buffer);

            // 4.3. Submitting Input Frame for Encoding

            // TODO: Way too many options. Figure it out!
            let mut encode_pic_params = NV_ENC_PIC_PARAMS {
                version: NV_ENC_PIC_PARAMS_VER,
                inputWidth: WIDTH,
                inputHeight: HEIGHT,
                inputPitch: WIDTH,
                encodePicFlags: NV_ENC_PIC_FLAGS::NV_ENC_PIC_FLAG_EOS as u32, // TODO: Which flag should be used when?
                inputBuffer: input_buffer,
                outputBitstream: output_buffer,
                bufferFmt: buffer_format,
                pictureStruct: NV_ENC_PIC_STRUCT::NV_ENC_PIC_STRUCT_FRAME,
                codecPicParams: NV_ENC_CODEC_PIC_PARAMS::default(),
                ..Default::default()
            };
            nvEncEncodePicture(encoder, &mut encode_pic_params as *mut NV_ENC_PIC_PARAMS);

            // 4.4. Retrieving Encoded Output

            // Lock output bitsream.
            let mut lock_bitstream_buffer_params = NV_ENC_LOCK_BITSTREAM {
                version: NV_ENC_LOCK_BITSTREAM_VER,
                outputBitstream: output_buffer,
                ..Default::default()
            };
            nvEncLockBitstream(
                encoder,
                &mut lock_bitstream_buffer_params as *mut NV_ENC_LOCK_BITSTREAM,
            );
            let _output_buffer_data = lock_bitstream_buffer_params.bitstreamBufferPtr;

            // TODO: Examine output buffer data.

            // Unlock output bitstream.
            nvEncUnlockBitstream(encoder, output_buffer);

            // 5.2. Releasing Resources

            nvEncDestroyInputBuffer(encoder, input_buffer);
            nvEncDestroyBitstreamBuffer(encoder, output_buffer);

            // 5.3. Closing Encode Session
            nvEncDestroyEncoder(encoder);
        }
    }
}
