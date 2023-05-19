use std::{error::Error, fmt};

use crate::sys::nvEncodeAPI::NVENCSTATUS;

/// Wrapper enum around [`NVENCSTATUS`].
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum EncodeError {
    /// No encode capable devices were detected.
    NoEncodeDevice = 1,
    /// The device passed by the client is not supported.
    UnsupportedDevice = 2,
    /// The encoder device supplied by the client is not valid.
    InvalidEncoderDevice = 3,
    /// The device passed to the API call is invalid.
    InvalidDevice = 4,
    /// The device passed to the API call is no longer available
    /// and needs to be reinitialized. The clients need to destroy the
    /// current encoder session by freeing the allocated input output
    /// buffers and destroying the device and create a new encoding session.
    DeviceNotExist = 5,
    /// One or more of the pointers passed to the API call is invalid.
    InvalidPtr = 6,
    /// The completion event passed in the [`EncodeAPI.encode_picture`]
    /// call is invalid.
    InvalidEvent = 7,
    /// One or more of the parameter passed to the API call is invalid.
    InvalidParam = 8,
    /// An API call was made in wrong sequence or order.
    InvalidCall = 9,
    /// the API call failed because it was unable to allocate enough memory
    /// to perform the requested operation.
    OutOfMemory = 10,
    /// The encoder has not been initialized with
    /// [`EncodeAPI.initialize_encoder`] or that initialization has failed.
    /// The client cannot allocate input or output buffers or do any encoding
    /// related operation before successfully initializing the encoder.
    EncoderNotInitialized = 11,
    /// An unsupported parameter was passed by the client.
    UnsupportedParam = 12,
    /// The [`EncodeAPI.lock_bitstream`] failed to lock the output
    /// buffer. This happens when the client makes a non blocking lock call
    /// to access the output bitstream by passing the `do_not_wait` flag.
    /// This is not a fatal error and client should retry the same operation
    /// after few milliseconds.
    LockBusy = 13,
    /// The size of the user buffer passed by the client is insufficient for
    /// the requested operation.
    NotEnoughBuffer = 14,
    /// An invalid struct version was used by the client.
    InvalidVersion = 15,
    /// [`EncodeAPI.map_input_resource`] failed to map the client provided
    /// input resource.
    MapFailed = 16,
    /// The encode driver requires more input buffers to produce an output
    /// bitstream. If this error is returned from [`EncodeAPI.encode_picture`],
    /// this is not a fatal error. If the client is encoding with B frames
    /// then, [`EncodeAPI.encode_picture`] might be buffering the input
    /// frame for re-ordering.
    ///
    /// A client operating in synchronous mode cannot call
    /// [`EncodeAPI.lock_bitstream`] on the output bitstream buffer if
    /// [`EncodeAPI.encode_picture`] returned this variant. The client must
    /// continue providing input frames until encode driver returns
    /// successfully. After a success the client
    /// can call [`EncodeAPI.lock_bitstream`] on the output buffers in the
    /// same order in which it has called [`EncodeAPI.encode_picture`].
    NeedMoreInput = 17,
    /// The hardware encoder is busy encoding and is unable to encode
    /// the input. The client should call [EncodeAPI.encode_picture] again after
    /// few milliseconds.
    EncoderBusy = 18,
    /// The completion event passed in [`EncodeAPI.encode_picture`]
    /// has not been registered with encoder driver using
    /// [`EncodeAPI.register_async_event`].
    EventNotRegistered = 19,
    /// An unknown internal error has occurred.
    Generic = 20,
    /// The client is attempting to use a feature
    /// that is not available for the license type for the current system.
    IncompatibleClientKey = 21,
    /// the client is attempting to use a feature
    /// that is not implemented for the current version.
    Unimplemented = 22,
    /// [`EncodeAPI.register_resource`] failed to register the resource.
    ResourceRegisterFailed = 23,
    /// The client is attempting to unregister a resource
    /// that has not been successfully registered.
    ResourceNotRegistered = 24,
    /// The client is attempting to unmap a resource
    /// that has not been successfully mapped.
    ResourceNotMapped = 25,
}

impl fmt::Display for EncodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl Error for EncodeError {}

impl From<NVENCSTATUS> for EncodeError {
    fn from(status: NVENCSTATUS) -> Self {
        match status {
            NVENCSTATUS::NV_ENC_SUCCESS => {
                unreachable!("Success should not be converted to an error.")
            }
            NVENCSTATUS::NV_ENC_ERR_NO_ENCODE_DEVICE => EncodeError::NoEncodeDevice,
            NVENCSTATUS::NV_ENC_ERR_UNSUPPORTED_DEVICE => EncodeError::UnsupportedDevice,
            NVENCSTATUS::NV_ENC_ERR_INVALID_ENCODERDEVICE => EncodeError::InvalidEncoderDevice,
            NVENCSTATUS::NV_ENC_ERR_INVALID_DEVICE => EncodeError::InvalidDevice,
            NVENCSTATUS::NV_ENC_ERR_DEVICE_NOT_EXIST => EncodeError::DeviceNotExist,
            NVENCSTATUS::NV_ENC_ERR_INVALID_PTR => EncodeError::InvalidPtr,
            NVENCSTATUS::NV_ENC_ERR_INVALID_EVENT => EncodeError::InvalidEvent,
            NVENCSTATUS::NV_ENC_ERR_INVALID_PARAM => EncodeError::InvalidParam,
            NVENCSTATUS::NV_ENC_ERR_INVALID_CALL => EncodeError::InvalidCall,
            NVENCSTATUS::NV_ENC_ERR_OUT_OF_MEMORY => EncodeError::OutOfMemory,
            NVENCSTATUS::NV_ENC_ERR_ENCODER_NOT_INITIALIZED => EncodeError::EncoderNotInitialized,
            NVENCSTATUS::NV_ENC_ERR_UNSUPPORTED_PARAM => EncodeError::UnsupportedParam,
            NVENCSTATUS::NV_ENC_ERR_LOCK_BUSY => EncodeError::LockBusy,
            NVENCSTATUS::NV_ENC_ERR_NOT_ENOUGH_BUFFER => EncodeError::NotEnoughBuffer,
            NVENCSTATUS::NV_ENC_ERR_INVALID_VERSION => EncodeError::InvalidVersion,
            NVENCSTATUS::NV_ENC_ERR_MAP_FAILED => EncodeError::MapFailed,
            NVENCSTATUS::NV_ENC_ERR_NEED_MORE_INPUT => EncodeError::NeedMoreInput,
            NVENCSTATUS::NV_ENC_ERR_ENCODER_BUSY => EncodeError::EncoderBusy,
            NVENCSTATUS::NV_ENC_ERR_EVENT_NOT_REGISTERD => EncodeError::EventNotRegistered,
            NVENCSTATUS::NV_ENC_ERR_GENERIC => EncodeError::Generic,
            NVENCSTATUS::NV_ENC_ERR_INCOMPATIBLE_CLIENT_KEY => EncodeError::IncompatibleClientKey,
            NVENCSTATUS::NV_ENC_ERR_UNIMPLEMENTED => EncodeError::Unimplemented,
            NVENCSTATUS::NV_ENC_ERR_RESOURCE_REGISTER_FAILED => EncodeError::ResourceRegisterFailed,
            NVENCSTATUS::NV_ENC_ERR_RESOURCE_NOT_REGISTERED => EncodeError::ResourceNotRegistered,
            NVENCSTATUS::NV_ENC_ERR_RESOURCE_NOT_MAPPED => EncodeError::ResourceNotMapped,
        }
    }
}

impl NVENCSTATUS {
    /// Convert an [`NVENCSTATUS`] to a [`Result`].
    ///
    /// [`NVENCSTATUS::NV_ENC_SUCCESS`] is converted to `Ok(())`,
    /// and all other variants are mapped to the corresponding variant
    /// in [`EncodeError`].
    ///
    /// # Errors
    ///
    /// Returns an error whenever the status is not
    /// [`NVENCSTATUS::NV_ENC_SUCCESS`].
    pub fn result(self) -> Result<(), EncodeError> {
        match self {
            NVENCSTATUS::NV_ENC_SUCCESS => Ok(()),
            err => Err(err.into()),
        }
    }
}
