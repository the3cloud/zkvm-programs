use core::fmt::{self, Display};

#[derive(Debug)]
pub enum Error {
    InvalidForwardValue,
    TryFromSliceError,
    InvalidBytesLength,
    AlloySolTypesError(alloy::sol_types::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub type Result<T> = core::result::Result<T, Error>;
