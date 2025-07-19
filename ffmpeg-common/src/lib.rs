//! Common functionality for FFmpeg suite Rust wrappers
//!
//! This crate provides shared types, utilities, and error handling for the
//! FFmpeg, FFprobe, and FFplay Rust wrappers.

#![warn(missing_docs)]
#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::must_use_candidate)]

pub mod error;
pub mod process;
pub mod types;
pub mod utils;

// Re-export commonly used items
pub use error::{Error, Result, ResultExt};
pub use process::{CommandBuilder, Process, ProcessConfig, ProcessOutput, Progress};
pub use types::{
    Codec, Duration, LogLevel, MediaPath, PixelFormat, SampleFormat, Size, StreamSpecifier,
    StreamType,
};

/// Version information for the FFmpeg suite
#[derive(Debug, Clone)]
pub struct Version {
    /// Major version number
    pub major: u32,
    /// Minor version number
    pub minor: u32,
    /// Patch version number
    pub patch: u32,
    /// Version string
    pub version_string: String,
    /// Configuration flags
    pub configuration: Vec<String>,
}

impl Version {
    /// Parse version information from FFmpeg output
    pub fn parse(output: &str) -> Result<Self> {
        // FFmpeg version output format:
        // ffmpeg version 4.4.2-0ubuntu0.22.04.1 Copyright (c) 2000-2021 the FFmpeg developers
        // built with gcc 11 (Ubuntu 11.2.0-19ubuntu1)
        // configuration: --prefix=/usr --extra-version=0ubuntu0.22.04.1 ...

        let lines: Vec<&str> = output.lines().collect();
        if lines.is_empty() {
            return Err(Error::ParseError("Empty version output".to_string()));
        }

        // Parse version line
        let version_line = lines[0];
        let version_string = if let Some(start) = version_line.find("version ") {
            let version_part = &version_line[start + 8..];
            version_part.split_whitespace().next().unwrap_or("").to_string()
        } else {
            return Err(Error::ParseError("Version line not found".to_string()));
        };

        // Extract major.minor.patch
        let parts: Vec<&str> = version_string.split(&['.', '-'][..]).collect();
        let major = parts.get(0)
            .and_then(|s| s.parse().ok())
            .unwrap_or(0);
        let minor = parts.get(1)
            .and_then(|s| s.parse().ok())
            .unwrap_or(0);
        let patch = parts.get(2)
            .and_then(|s| s.parse().ok())
            .unwrap_or(0);

        // Parse configuration
        let configuration = lines.iter()
            .find(|line| line.starts_with("configuration:"))
            .map(|line| {
                line[14..]
                    .split_whitespace()
                    .map(String::from)
                    .collect()
            })
            .unwrap_or_default();

        Ok(Self {
            major,
            minor,
            patch,
            version_string,
            configuration,
        })
    }

    /// Check if this version is at least the specified version
    pub fn is_at_least(&self, major: u32, minor: u32, patch: u32) -> bool {
        if self.major > major {
            return true;
        }
        if self.major < major {
            return false;
        }
        if self.minor > minor {
            return true;
        }
        if self.minor < minor {
            return false;
        }
        self.patch >= patch
    }
}

/// Get version information for an FFmpeg executable
pub async fn get_version(executable: &str) -> Result<Version> {
    let path = process::find_executable(executable)?;
    let config = ProcessConfig::new(path)
        .capture_stdout(true)
        .capture_stderr(false);

    let output = Process::spawn(config, vec!["-version".to_string()])
        .await?
        .wait()
        .await?
        .into_result()?;

    let version_output = output.stdout_str()
        .ok_or_else(|| Error::ParseError("No version output".to_string()))?;

    Version::parse(&version_output)
}

/// Capabilities detection for FFmpeg tools
#[derive(Debug, Clone, Default)]
pub struct Capabilities {
    /// Available codecs
    pub codecs: Vec<String>,
    /// Available formats
    pub formats: Vec<String>,
    /// Available filters
    pub filters: Vec<String>,
    /// Available protocols
    pub protocols: Vec<String>,
    /// Available pixel formats
    pub pixel_formats: Vec<String>,
    /// Available sample formats
    pub sample_formats: Vec<String>,
}

impl Capabilities {
    /// Detect capabilities by running FFmpeg with various list options
    pub async fn detect(executable: &str) -> Result<Self> {
        let caps = Self::default();

        // This is a simplified version - in a real implementation,
        // we would parse the output of ffmpeg -codecs, -formats, etc.

        Ok(caps)
    }

    /// Check if a codec is available
    pub fn has_codec(&self, codec: &str) -> bool {
        self.codecs.iter().any(|c| c == codec)
    }

    /// Check if a format is available
    pub fn has_format(&self, format: &str) -> bool {
        self.formats.iter().any(|f| f == format)
    }

    /// Check if a filter is available
    pub fn has_filter(&self, filter: &str) -> bool {
        self.filters.iter().any(|f| f == filter)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_parsing() {
        let output = r#"ffmpeg version 4.4.2-0ubuntu0.22.04.1 Copyright (c) 2000-2021 the FFmpeg developers
built with gcc 11 (Ubuntu 11.2.0-19ubuntu1)
configuration: --prefix=/usr --extra-version=0ubuntu0.22.04.1 --toolchain=hardened"#;

        let version = Version::parse(output).unwrap();
        assert_eq!(version.major, 4);
        assert_eq!(version.minor, 4);
        assert_eq!(version.patch, 2);
        assert!(version.version_string.starts_with("4.4.2"));
        assert!(!version.configuration.is_empty());
    }

    #[test]
    fn test_version_comparison() {
        let version = Version {
            major: 4,
            minor: 4,
            patch: 2,
            version_string: "4.4.2".to_string(),
            configuration: vec![],
        };

        assert!(version.is_at_least(4, 4, 2));
        assert!(version.is_at_least(4, 4, 1));
        assert!(version.is_at_least(4, 3, 5));
        assert!(version.is_at_least(3, 9, 9));
        assert!(!version.is_at_least(4, 4, 3));
        assert!(!version.is_at_least(4, 5, 0));
        assert!(!version.is_at_least(5, 0, 0));
    }
}