use thiserror::Error;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Game(#[from] crate::game::error::Error),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}
