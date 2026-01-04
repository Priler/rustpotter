//! Error types for the Rustpotter library.

use std::fmt;

/// Main error type for Rustpotter operations.
#[derive(Debug)]
pub enum RustpotterError {
    AudioFormat(String),
    WakewordLoad(String),
    WakewordCreation(String),
    Model(String),
    Io(std::io::Error),
    Resampler(String),
    Config(String),
}

impl fmt::Display for RustpotterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RustpotterError::AudioFormat(msg) => write!(f, "Audio format error: {}", msg),
            RustpotterError::WakewordLoad(msg) => write!(f, "Wakeword loading error: {}", msg),
            RustpotterError::WakewordCreation(msg) => write!(f, "Wakeword creation error: {}", msg),
            RustpotterError::Model(msg) => write!(f, "Model error: {}", msg),
            RustpotterError::Io(err) => write!(f, "I/O error: {}", err),
            RustpotterError::Resampler(msg) => write!(f, "Resampler error: {}", msg),
            RustpotterError::Config(msg) => write!(f, "Configuration error: {}", msg),
        }
    }
}

impl std::error::Error for RustpotterError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            RustpotterError::Io(err) => Some(err),
            _ => None,
        }
    }
}

impl From<std::io::Error> for RustpotterError {
    fn from(err: std::io::Error) -> Self {
        RustpotterError::Io(err)
    }
}

impl From<String> for RustpotterError {
    fn from(msg: String) -> Self {
        RustpotterError::WakewordLoad(msg)
    }
}

impl From<&str> for RustpotterError {
    fn from(msg: &str) -> Self {
        RustpotterError::WakewordLoad(msg.to_string())
    }
}

pub type Result<T> = std::result::Result<T, RustpotterError>;
