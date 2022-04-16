use thiserror::Error;

pub type Result<T> = std::result::Result<T, DisqError>;

#[derive(Debug, Error)]
pub enum DisqError {
    #[error(
        "Encountered an error communicating with Discord via http. Root cause was: {source:?}"
    )]
    HttpError { source: serenity::Error },
}
