/// Enum listing possible errors from ipctl.
#[derive(Debug)]
pub enum Error {
    Tonic(tonic::transport::Error),
}

/// A specialized [`Result`] type for ipctl.
pub type Result<T> = std::result::Result<T, Error>;

impl From<tonic::transport::Error> for Error {
    fn from(value: tonic::transport::Error) -> Self {
        Error::Tonic(value)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use self::Error::*;
        match self {
            Tonic(e) => e.fmt(f),
        }
    }
}

impl std::error::Error for Error {}
