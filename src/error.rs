use thiserror::Error;
use std::string::FromUtf8Error;

#[non_exhaustive]
#[derive(Error, Debug)]
pub enum RMeshError {
    #[error(transparent)]
    NonUTF8(FromUtf8Error),
    #[error("Error while trying to write data: {0}")]
    BinRwError(#[from] binrw::Error),
}

impl From<FromUtf8Error> for RMeshError {
    fn from(error: FromUtf8Error) -> Self {
        RMeshError::NonUTF8(error)
    }
}