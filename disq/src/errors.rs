use thiserror::Error;

pub type Result<T> = std::result::Result<T, DisqError>;

#[derive(Debug, Error)]
pub enum DisqError {
    #[error(transparent)]
    DiscordError(#[from] serenity::Error),
}
