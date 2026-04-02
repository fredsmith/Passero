use serde::Serialize;

#[derive(Debug, thiserror::Error)]
pub enum PasseroError {
    #[error("pass command failed: {0}")]
    PassError(String),
    #[error("GPG error: {0}")]
    GpgError(String),
    #[error("Git error: {0}")]
    GitError(String),
    #[error("Config error: {0}")]
    ConfigError(String),
    #[error("TOTP error: {0}")]
    TotpError(String),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

impl Serialize for PasseroError {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

pub type Result<T> = std::result::Result<T, PasseroError>;
