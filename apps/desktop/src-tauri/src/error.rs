use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ScoopError {
    #[error("{0}")]
    Message(String),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Db(#[from] rusqlite::Error),
    #[error(transparent)]
    Image(#[from] image::ImageError),
    #[error(transparent)]
    Serde(#[from] serde_json::Error),
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),
}

impl Serialize for ScoopError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

pub type ScoopResult<T> = Result<T, ScoopError>;

impl ScoopError {
    pub fn msg(s: impl Into<String>) -> Self {
        Self::Message(s.into())
    }
}
