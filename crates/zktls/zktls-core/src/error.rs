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
    SignatureError(alloy_primitives::SignatureError),

    #[error("must set origin")]
    MustSetOrigin,

    #[error("invalid normalize v value")]
    InvalidNormalizeV,
}

pub type Result<T> = core::result::Result<T, Error>;
