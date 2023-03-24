use thiserror::Error;
use std::string::FromUtf8Error;

/// The `RMeshError` enum represents errors that can occur while reading for [`RMesh`].
#[non_exhaustive]
#[derive(Error, Debug)]
pub enum RMeshError {
    /// An error that occurs when the header of the room mesh file is invalid.
    #[error("Invalid Header: (Expected RoomMesh or RoomMesh.HasTriggerBox, instead got {0})")]
    InvalidHeader(String),
    /// An I/O error that occurs while reading data.
    #[error("io error while reading data: {0}")]
    IO(#[from] std::io::Error),
    /// An error that occurs when a non-UTF-8 byte sequence is encountered while reading data.
    #[error(transparent)]
    NonUTF8(FromUtf8Error),
}

impl From<FromUtf8Error> for RMeshError {
    fn from(error: FromUtf8Error) -> Self {
        RMeshError::NonUTF8(error)
    }
}