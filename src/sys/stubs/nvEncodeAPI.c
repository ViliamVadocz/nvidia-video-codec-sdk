#include "nvEncodeAPI.h"

NVENCSTATUS NVENCAPI NvEncOpenEncodeSession                     (void* device, uint32_t deviceType, void** encoder) { return 0; }
NVENCSTATUS NVENCAPI NvEncGetEncodeGUIDCount                    (void* encoder, uint32_t* encodeGUIDCount) { return 0; }
NVENCSTATUS NVENCAPI NvEncGetEncodeGUIDs                        (void* encoder, GUID* GUIDs, uint32_t guidArraySize, uint32_t* GUIDCount) { return 0; }
NVENCSTATUS NVENCAPI NvEncGetEncodeProfileGUIDCount                    (void* encoder, GUID encodeGUID, uint32_t* encodeProfileGUIDCount) { return 0; }
NVENCSTATUS NVENCAPI NvEncGetEncodeProfileGUIDs                               (void* encoder, GUID encodeGUID, GUID* profileGUIDs, uint32_t guidArraySize, uint32_t* GUIDCount) { return 0; }
NVENCSTATUS NVENCAPI NvEncGetInputFormatCount                   (void* encoder, GUID encodeGUID, uint32_t* inputFmtCount) { return 0; }
NVENCSTATUS NVENCAPI NvEncGetInputFormats                       (void* encoder, GUID encodeGUID, NV_ENC_BUFFER_FORMAT* inputFmts, uint32_t inputFmtArraySize, uint32_t* inputFmtCount) { return 0; }
NVENCSTATUS NVENCAPI NvEncGetEncodeCaps                     (void* encoder, GUID encodeGUID, NV_ENC_CAPS_PARAM* capsParam, int* capsVal) { return 0; }
NVENCSTATUS NVENCAPI NvEncGetEncodePresetCount              (void* encoder, GUID encodeGUID, uint32_t* encodePresetGUIDCount) { return 0; }
NVENCSTATUS NVENCAPI NvEncGetEncodePresetGUIDs                  (void* encoder, GUID encodeGUID, GUID* presetGUIDs, uint32_t guidArraySize, uint32_t* encodePresetGUIDCount) { return 0; }
NVENCSTATUS NVENCAPI NvEncGetEncodePresetConfig               (void* encoder, GUID encodeGUID, GUID  presetGUID, NV_ENC_PRESET_CONFIG* presetConfig) { return 0; }
NVENCSTATUS NVENCAPI NvEncGetEncodePresetConfigEx               (void* encoder, GUID encodeGUID, GUID  presetGUID, NV_ENC_TUNING_INFO tuningInfo, NV_ENC_PRESET_CONFIG* presetConfig) { return 0; }
NVENCSTATUS NVENCAPI NvEncInitializeEncoder                     (void* encoder, NV_ENC_INITIALIZE_PARAMS* createEncodeParams) { return 0; }
NVENCSTATUS NVENCAPI NvEncCreateInputBuffer                     (void* encoder, NV_ENC_CREATE_INPUT_BUFFER* createInputBufferParams) { return 0; }
NVENCSTATUS NVENCAPI NvEncDestroyInputBuffer                    (void* encoder, NV_ENC_INPUT_PTR inputBuffer) { return 0; }
NVENCSTATUS NVENCAPI NvEncSetIOCudaStreams                     (void* encoder, NV_ENC_CUSTREAM_PTR inputStream, NV_ENC_CUSTREAM_PTR outputStream) { return 0; }
NVENCSTATUS NVENCAPI NvEncCreateBitstreamBuffer                 (void* encoder, NV_ENC_CREATE_BITSTREAM_BUFFER* createBitstreamBufferParams) { return 0; }
NVENCSTATUS NVENCAPI NvEncDestroyBitstreamBuffer                (void* encoder, NV_ENC_OUTPUT_PTR bitstreamBuffer) { return 0; }
NVENCSTATUS NVENCAPI NvEncEncodePicture                         (void* encoder, NV_ENC_PIC_PARAMS* encodePicParams) { return 0; }
NVENCSTATUS NVENCAPI NvEncLockBitstream                         (void* encoder, NV_ENC_LOCK_BITSTREAM* lockBitstreamBufferParams) { return 0; }
NVENCSTATUS NVENCAPI NvEncUnlockBitstream                       (void* encoder, NV_ENC_OUTPUT_PTR bitstreamBuffer) { return 0; }
NVENCSTATUS NVENCAPI NvEncRestoreEncoderState                    (void* encoder, NV_ENC_RESTORE_ENCODER_STATE_PARAMS* restoreState) { return 0; }
NVENCSTATUS NVENCAPI NvEncLockInputBuffer                      (void* encoder, NV_ENC_LOCK_INPUT_BUFFER* lockInputBufferParams) { return 0; }
NVENCSTATUS NVENCAPI NvEncUnlockInputBuffer                     (void* encoder, NV_ENC_INPUT_PTR inputBuffer) { return 0; }
NVENCSTATUS NVENCAPI NvEncGetEncodeStats                        (void* encoder, NV_ENC_STAT* encodeStats) { return 0; }
NVENCSTATUS NVENCAPI NvEncGetSequenceParams                     (void* encoder, NV_ENC_SEQUENCE_PARAM_PAYLOAD* sequenceParamPayload) { return 0; }
NVENCSTATUS NVENCAPI NvEncGetSequenceParamEx                     (void* encoder, NV_ENC_INITIALIZE_PARAMS* encInitParams, NV_ENC_SEQUENCE_PARAM_PAYLOAD* sequenceParamPayload) { return 0; }
NVENCSTATUS NVENCAPI NvEncRegisterAsyncEvent                    (void* encoder, NV_ENC_EVENT_PARAMS* eventParams) { return 0; }
NVENCSTATUS NVENCAPI NvEncUnregisterAsyncEvent                  (void* encoder, NV_ENC_EVENT_PARAMS* eventParams) { return 0; }
NVENCSTATUS NVENCAPI NvEncMapInputResource                         (void* encoder, NV_ENC_MAP_INPUT_RESOURCE* mapInputResParams) { return 0; }
NVENCSTATUS NVENCAPI NvEncUnmapInputResource                         (void* encoder, NV_ENC_INPUT_PTR mappedInputBuffer) { return 0; }
NVENCSTATUS NVENCAPI NvEncDestroyEncoder                        (void* encoder) { return 0; }
NVENCSTATUS NVENCAPI NvEncInvalidateRefFrames(void* encoder, uint64_t invalidRefFrameTimeStamp) { return 0; }
NVENCSTATUS NVENCAPI NvEncOpenEncodeSessionEx                   (NV_ENC_OPEN_ENCODE_SESSION_EX_PARAMS *openSessionExParams, void** encoder) { return 0; }
NVENCSTATUS NVENCAPI NvEncRegisterResource                      (void* encoder, NV_ENC_REGISTER_RESOURCE* registerResParams) { return 0; }
NVENCSTATUS NVENCAPI NvEncUnregisterResource                    (void* encoder, NV_ENC_REGISTERED_PTR registeredResource) { return 0; }
NVENCSTATUS NVENCAPI NvEncReconfigureEncoder                   (void *encoder, NV_ENC_RECONFIGURE_PARAMS* reInitEncodeParams) { return 0; }
NVENCSTATUS NVENCAPI NvEncCreateMVBuffer                        (void* encoder, NV_ENC_CREATE_MV_BUFFER* createMVBufferParams) { return 0; }
NVENCSTATUS NVENCAPI NvEncDestroyMVBuffer                       (void* encoder, NV_ENC_OUTPUT_PTR mvBuffer) { return 0; }
NVENCSTATUS NVENCAPI NvEncRunMotionEstimationOnly               (void* encoder, NV_ENC_MEONLY_PARAMS* meOnlyParams) { return 0; }
NVENCSTATUS NVENCAPI NvEncodeAPIGetMaxSupportedVersion          (uint32_t* version) { return 0; }
NVENCSTATUS NVENCAPI NvEncLookaheadPicture          (void* encoder, NV_ENC_LOOKAHEAD_PIC_PARAMS *lookaheadParamas) { return 0; }
NVENCSTATUS NVENCAPI NvEncodeAPICreateInstance(NV_ENCODE_API_FUNCTION_LIST *functionList) { return 0; }