use std::string::FromUtf8Error;
use thiserror::Error;

#[non_exhaustive]
#[derive(Error, Debug)]
pub enum RMeshError {
    #[error(transparent)]
    NonUTF8(#[from] FromUtf8Error),
    #[error("Error while trying to write data: {0}")]
    BinRwError(#[from] binrw::Error),
}
