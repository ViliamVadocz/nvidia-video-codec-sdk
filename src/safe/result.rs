use std::{error::Error, fmt};

use crate::sys::nvEncodeAPI::NVENCSTATUS;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum EncodeError {
    NoEncodeDevice,
    UnsupportedDevice,
    InvalidEncoderdevice,
    InvalidDevice,
    DeviceNotExist,
    InvalidPtr,
    InvalidEvent,
    InvalidParam,
    InvalidCall,
    OutOfMemory,
    EncoderNotInitialized,
    UnsupportedParam,
    LockBusy,
    NotEnoughBuffer,
    InvalidVersion,
    MapFailed,
    NeedMoreInput,
    EncoderBusy,
    EventNotRegisterd,
    Generic,
    IncompatibleClientKey,
    Unimplemented,
    ResourceRegisterFailed,
    ResourceNotRegistered,
    ResourceNotMapped,
}

impl fmt::Display for EncodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
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
            NVENCSTATUS::NV_ENC_ERR_INVALID_ENCODERDEVICE => EncodeError::InvalidEncoderdevice,
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
            NVENCSTATUS::NV_ENC_ERR_EVENT_NOT_REGISTERD => EncodeError::EventNotRegisterd,
            NVENCSTATUS::NV_ENC_ERR_GENERIC => EncodeError::Generic,
            NVENCSTATUS::NV_ENC_ERR_INCOMPATIBLE_CLIENT_KEY => EncodeError::IncompatibleClientKey,
            NVENCSTATUS::NV_ENC_ERR_UNIMPLEMENTED => EncodeError::Unimplemented,
            NVENCSTATUS::NV_ENC_ERR_RESOURCE_REGISTER_FAILED => EncodeError::ResourceRegisterFailed,
            NVENCSTATUS::NV_ENC_ERR_RESOURCE_NOT_REGISTERED => EncodeError::ResourceNotRegistered,
            NVENCSTATUS::NV_ENC_ERR_RESOURCE_NOT_MAPPED => EncodeError::ResourceNotMapped,
        }
    }
}

pub type EncodeResult<T> = Result<T, EncodeError>;

impl NVENCSTATUS {
    pub fn result(self) -> Result<(), EncodeError> {
        match self {
            NVENCSTATUS::NV_ENC_SUCCESS => Ok(()),
            err => Err(err.into()),
        }
    }
}

// TODO: Improve error types (each function only uses a subset of the possible error variants)
