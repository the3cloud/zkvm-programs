use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("invalid forward value")]
    InvalidForwardValue,

    #[error("try from slice error")]
    TryFromSliceError,

    #[error("invalid bytes length")]
    InvalidBytesLength,

    #[error(transparent)]
    SignatureError(alloy::primitives::SignatureError),

    #[error(transparent)]
    AlloySolError(alloy::sol_types::Error),

    #[error("must set origin")]
    MustSetOrigin,
}

pub type Result<T> = core::result::Result<T, Error>;
