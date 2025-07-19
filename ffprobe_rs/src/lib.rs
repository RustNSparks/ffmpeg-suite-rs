//! Safe and idiomatic Rust wrapper for FFprobe
//!
//! This crate provides a high-level, safe interface to FFprobe functionality,
//! allowing you to extract metadata and information from multimedia files.
//!
//! # Examples
//!
//! ## Basic usage
//! ```no_run
//! use ffprobe_rs::FFprobeBuilder;
//!
//! # async fn example() -> ffmpeg_common::Result<()> {
//! // Probe a file for format and stream information
//! let result = FFprobeBuilder::probe("video.mp4")
//!     .run()
//!     .await?;
//!
//! // Access format information
//! if let Some(format) = &result.format {
//!     println!("Format: {}", format.format_name.as_deref().unwrap_or("unknown"));
//!     println!("Duration: {} seconds", result.duration().unwrap_or(0.0));
//! }
//!
//! // Access stream information
//! for stream in &result.streams {
//!     match stream.codec_type.as_deref() {
//!         Some("video") => {
//!             println!("Video: {}x{}", 
//!                 stream.width.unwrap_or(0), 
//!                 stream.height.unwrap_or(0)
//!             );
//!         }
//!         Some("audio") => {
//!             println!("Audio: {} Hz, {} channels",
//!                 stream.sample_rate.as_deref().unwrap_or("?"),
//!                 stream.channels.unwrap_or(0)
//!             );
//!         }
//!         _ => {}
//!     }
//! }
//! # Ok(())
//! # }
//! ```
//!
//! ## Advanced usage
//! ```no_run
//! use ffprobe_rs::{FFprobeBuilder, OutputFormat};
//! use ffprobe_rs::format::presets;
//! use ffmpeg_common::StreamSpecifier;
//!
//! # async fn example() -> ffmpeg_common::Result<()> {
//! // Detailed probe with specific options
//! let result = FFprobeBuilder::new()?
//!     .input("https://example.com/stream.m3u8")
//!     .show_format()
//!     .show_streams()
//!     .show_chapters()
//!     .count_frames(true)
//!     .select_streams(StreamSpecifier::Type(ffmpeg_common::StreamType::Video))
//!     .output_format(OutputFormat::Json)
//!     .pretty(true)
//!     .run()
//!     .await?;
//!
//! // Get primary video stream
//! if let Some(video) = result.primary_video_stream() {
//!     println!("Video codec: {}", video.codec_name.as_deref().unwrap_or("unknown"));
//!     println!("Frame rate: {:.2} fps", video.frame_rate().unwrap_or(0.0));
//!     println!("Bit rate: {} bps", video.bit_rate_bps().unwrap_or(0));
//! }
//! # Ok(())
//! # }
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::must_use_candidate)]

pub mod builder;
pub mod format;
pub mod parsers;
pub mod types;

// Re-export main types
pub use builder::FFprobeBuilder;
pub use format::{EscapeMode, OutputFormat, StringValidation, WriterOptions};
pub use types::{
    ChapterInfo, ErrorInfo, FormatInfo, FrameInfo, IntervalPosition, PacketInfo, ProbeResult,
    ProbeSection, ProgramInfo, ReadInterval, StreamInfo,
};

// Re-export from common
pub use ffmpeg_common::{
    get_version, Capabilities, Duration, Error, LogLevel, MediaPath, Result, StreamSpecifier,
    StreamType, Version,
};

/// Prelude module for convenient imports
pub mod prelude {
    pub use crate::{
        FFprobeBuilder, OutputFormat, ProbeResult, StreamInfo,
        format::presets,
    };
    pub use ffmpeg_common::{Duration, MediaPath, Result, StreamSpecifier, StreamType};
}

/// Quick probe a file for basic information
pub async fn probe(path: impl Into<MediaPath>) -> Result<ProbeResult> {
    FFprobeBuilder::probe(path).run().await
}

/// Probe only format information
pub async fn probe_format(path: impl Into<MediaPath>) -> Result<ProbeResult> {
    FFprobeBuilder::probe_format(path).run().await
}

/// Probe only stream information
pub async fn probe_streams(path: impl Into<MediaPath>) -> Result<ProbeResult> {
    FFprobeBuilder::probe_streams(path).run().await
}

/// Get FFprobe capabilities
pub async fn capabilities() -> Result<Capabilities> {
    Capabilities::detect("ffprobe").await
}

/// Check if FFprobe is available
pub async fn is_available() -> bool {
    ffmpeg_common::process::find_executable("ffprobe").is_ok()
}

/// Get FFprobe version
pub async fn version() -> Result<Version> {
    get_version("ffprobe").await
}

/// Helper functions for common probe operations
pub mod helpers {
    use super::*;

    /// Get video dimensions from a file
    pub async fn get_dimensions(path: impl Into<MediaPath>) -> Result<Option<(u32, u32)>> {
        let result = probe_streams(path).await?;
        Ok(result.primary_video_stream().unwrap().resolution())
    }

    /// Get duration in seconds
    pub async fn get_duration(path: impl Into<MediaPath>) -> Result<Option<f64>> {
        let result = probe_format(path).await?;
        Ok(result.duration())
    }

    /// Get format name
    pub async fn get_format(path: impl Into<MediaPath>) -> Result<Option<String>> {
        let result = probe_format(path).await?;
        Ok(result.format_name().map(String::from))
    }

    /// Get video codec
    pub async fn get_video_codec(path: impl Into<MediaPath>) -> Result<Option<String>> {
        let result = probe_streams(path).await?;
        Ok(result
            .primary_video_stream()
            .unwrap()
            .codec_name
            .clone())
    }

    /// Get audio codec
    pub async fn get_audio_codec(path: impl Into<MediaPath>) -> Result<Option<String>> {
        let result = probe_streams(path).await?;
        Ok(result
            .primary_audio_stream()
            .unwrap()
            .codec_name
            .clone())
    }

    /// Check if file has video
    pub async fn has_video(path: impl Into<MediaPath>) -> Result<bool> {
        let result = probe_streams(path).await?;
        Ok(!result.video_streams().is_empty())
    }

    /// Check if file has audio
    pub async fn has_audio(path: impl Into<MediaPath>) -> Result<bool> {
        let result = probe_streams(path).await?;
        Ok(!result.audio_streams().is_empty())
    }

    /// Check if file has subtitles
    pub async fn has_subtitles(path: impl Into<MediaPath>) -> Result<bool> {
        let result = probe_streams(path).await?;
        Ok(!result.subtitle_streams().is_empty())
    }

    /// Get all metadata tags
    pub async fn get_metadata(path: impl Into<MediaPath>) -> Result<std::collections::HashMap<String, String>> {
        let result = probe_format(path).await?;
        Ok(result
            .format
            .map(|f| f.tags)
            .unwrap_or_default())
    }

    /// Get stream count
    pub async fn get_stream_count(path: impl Into<MediaPath>) -> Result<usize> {
        let result = probe_streams(path).await?;
        Ok(result.streams.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::*;

    #[test]
    fn test_builder_creation() {
        let builder = FFprobeBuilder::probe("test.mp4");
        let command = builder.command().unwrap();
        assert!(command.contains("ffprobe"));
        assert!(command.contains("test.mp4"));
        assert!(command.contains("-show_format"));
        assert!(command.contains("-show_streams"));
    }

    #[test]
    fn test_output_format_selection() {
        let builder = FFprobeBuilder::probe("test.mp4")
            .output_format(OutputFormat::Xml);
        let command = builder.command().unwrap();
        assert!(command.contains("-print_format xml"));
    }

    #[test]
    fn test_stream_selection() {
        let builder = FFprobeBuilder::probe_stream(
            "test.mp4",
            StreamSpecifier::Type(StreamType::Audio),
        );
        let command = builder.command().unwrap();
        assert!(command.contains("-select_streams a"));
    }
}