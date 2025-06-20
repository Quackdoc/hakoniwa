/// Error handling with the Result type.
pub type Result<T> = std::result::Result<T, Error>;

/// Error types.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    ProcessError(#[from] ProcessErrorKind),

    #[doc(hidden)]
    #[error("{0}")]
    Unexpected(String),
}

#[derive(thiserror::Error, Debug)]
pub enum ProcessErrorKind {
    #[error(transparent)]
    BincodeDecodeError(#[from] bincode::error::DecodeError),
    #[error(transparent)]
    NixError(#[from] nix::Error),
    #[error("setup network failed")]
    SetupNetworkFailed,
    #[error(transparent)]
    StdIoError(#[from] std::io::Error),
    #[error("std thread panic")]
    StdThreadPanic,
    #[error("child exit status gone")]
    ChildExitStatusGone,
}
