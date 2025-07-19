use std::io;
use std::process::ExitStatus;
use thiserror::Error;

/// Result type for FFmpeg suite operations
pub type Result<T> = std::result::Result<T, Error>;

/// Main error type for FFmpeg suite operations
#[derive(Error, Debug)]
pub enum Error {
    /// FFmpeg/FFprobe/FFplay executable not found
    #[error("Executable not found: {0}")]
    ExecutableNotFound(String),

    /// IO error occurred
    #[error("IO error: {0}")]
    Io(#[from] io::Error),

    /// Process execution failed
    #[error("Process execution failed: {message}")]
    ProcessFailed {
        message: String,
        exit_status: Option<ExitStatus>,
        stderr: Option<String>,
    },

    /// Invalid argument provided
    #[error("Invalid argument: {0}")]
    InvalidArgument(String),

    /// Parse error occurred
    #[error("Parse error: {0}")]
    ParseError(String),

    /// Timeout occurred
    #[error("Operation timed out after {0:?}")]
    Timeout(std::time::Duration),

    /// Feature not supported
    #[error("Feature not supported: {0}")]
    Unsupported(String),

    /// Invalid output from FFmpeg tool
    #[error("Invalid output: {0}")]
    InvalidOutput(String),

    /// Multiple errors occurred
    #[error("Multiple errors occurred")]
    Multiple(Vec<Error>),

    /// Generic error with context
    #[error("{context}: {source}")]
    WithContext {
        context: String,
        #[source]
        source: Box<Error>,
    },
}

impl Error {
    /// Add context to an error
    pub fn context<S: Into<String>>(self, context: S) -> Self {
        Error::WithContext {
            context: context.into(),
            source: Box::new(self),
        }
    }

    /// Create a process failed error
    pub fn process_failed(message: impl Into<String>, exit_status: Option<ExitStatus>, stderr: Option<String>) -> Self {
        Error::ProcessFailed {
            message: message.into(),
            exit_status,
            stderr,
        }
    }

    /// Check if this is a timeout error
    pub fn is_timeout(&self) -> bool {
        matches!(self, Error::Timeout(_))
    }

    /// Check if this is an IO error
    pub fn is_io(&self) -> bool {
        matches!(self, Error::Io(_))
    }
}

/// Extension trait for adding context to Results
pub trait ResultExt<T> {
    fn context<S: Into<String>>(self, context: S) -> Result<T>;
}

impl<T> ResultExt<T> for Result<T> {
    fn context<S: Into<String>>(self, context: S) -> Result<T> {
        self.map_err(|e| e.context(context))
    }
}

/// Builder for creating detailed error messages
pub struct ErrorBuilder {
    message: String,
    details: Vec<String>,
}

impl ErrorBuilder {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            details: Vec::new(),
        }
    }

    pub fn detail(mut self, detail: impl Into<String>) -> Self {
        self.details.push(detail.into());
        self
    }

    pub fn build(self) -> Error {
        let mut message = self.message;
        if !self.details.is_empty() {
            message.push_str("\nDetails:\n");
            for detail in self.details {
                message.push_str(&format!("  - {}\n", detail));
            }
        }
        Error::InvalidArgument(message)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_context() {
        let error = Error::Io(io::Error::new(io::ErrorKind::NotFound, "file not found"));
        let with_context = error.context("Failed to read input file");

        match with_context {
            Error::WithContext { context, source } => {
                assert_eq!(context, "Failed to read input file");
                assert!(matches!(*source, Error::Io(_)));
            }
            _ => panic!("Expected WithContext error"),
        }
    }

    #[test]
    fn test_error_builder() {
        let error = ErrorBuilder::new("Invalid codec")
            .detail("Codec 'invalid' is not supported")
            .detail("Use 'ffmpeg -codecs' to see available codecs")
            .build();

        match error {
            Error::InvalidArgument(msg) => {
                assert!(msg.contains("Invalid codec"));
                assert!(msg.contains("Details:"));
                assert!(msg.contains("Codec 'invalid' is not supported"));
            }
            _ => panic!("Expected InvalidArgument error"),
        }
    }
}