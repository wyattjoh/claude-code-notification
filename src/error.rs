use thiserror::Error;

pub type NotificationResult<T> = Result<T, NotificationError>;

#[derive(Error, Debug)]
pub enum NotificationError {
    #[error("Failed to parse JSON input: {0}")]
    JsonParse(#[from] serde_json::Error),

    #[error("Failed to read input: {0}")]
    Io(#[from] std::io::Error),

    #[error("Failed to send notification: {0}")]
    Notification(#[from] notify_rust::error::Error),

    #[error("Invalid notification input: {0}")]
    InvalidInput(String),
}

impl NotificationError {
    pub fn invalid_input<S: Into<String>>(msg: S) -> Self {
        NotificationError::InvalidInput(msg.into())
    }
}
