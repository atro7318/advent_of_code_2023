use thiserror::Error;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Calibrate(#[from] crate::calibrate::error::Error),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}
