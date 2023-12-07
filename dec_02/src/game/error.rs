use thiserror::Error;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Grab(#[from] crate::game::grab::error::Error),
    #[error("Game ID not found.")]
    ParseId,
    #[error(transparent)]
    ParseInt(#[from] std::num::ParseIntError),
    #[error(transparent)]
    Regex(#[from] regex::Error),
}
