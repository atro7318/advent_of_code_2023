use thiserror::Error;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Regex(#[from] regex::Error),
    #[error("Provided: {0}. Cannot parse the provided input into a single digit number.")]
    InputCannotBeParsed(String),
}
