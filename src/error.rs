use thiserror::Error;
use std::string::FromUtf8Error;

#[non_exhaustive]
#[derive(Error, Debug)]
pub enum RMeshError {
    #[error("Invalid Header: (Expected RoomMesh or RoomMesh.HasTriggerBox, instead got {0})")]
    InvalidHeader(String),
    #[error("io error while reading data: {0}")]
    IO(#[from] std::io::Error),
    #[error(transparent)]
    NonUTF8(FromUtf8Error),
}

impl From<FromUtf8Error> for RMeshError {
    fn from(error: FromUtf8Error) -> Self {
        RMeshError::NonUTF8(error)
    }
}