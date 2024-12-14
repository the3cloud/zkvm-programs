use core::fmt::{self, Display};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    InvalidForwardValue,
    TryFromSliceError,
    InvalidBytesLength,
    SignatureError(alloy::primitives::SignatureError),
    AlloySolError(alloy::sol_types::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub type Result<T> = core::result::Result<T, Error>;
