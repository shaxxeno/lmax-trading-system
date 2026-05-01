use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Configuration error {0}")]
    Config(String),

    #[error("Feed error {0}")]
    Feed(String),

    #[error("Strategy error {0}")]
    Strategy(String),

    #[error("Risk error {0}")]
    Risk(String),

    #[error("Execution error {0}")]
    Execution(String),

    #[error("Unexpected error {0}")]
    Unexpected(String),
}
