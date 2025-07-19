//! Safe and idiomatic Rust wrapper for FFmpeg
//!
//! This crate provides a high-level, safe interface to FFmpeg functionality,
//! allowing you to transcode, filter, and manipulate multimedia files.
//!
//! # Examples
//!
//! ## Simple conversion
//! ```no_run
//! use ffmpeg_rs::FFmpegBuilder;
//!
//! # async fn example() -> ffmpeg_common::Result<()> {
//! // Convert a video file
//! let output = FFmpegBuilder::convert("input.mp4", "output.webm")
//!     .run()
//!     .await?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Complex transcoding
//! ```no_run
//! use ffmpeg_rs::{FFmpegBuilder, Input, Output};
//! use ffmpeg_rs::codec::presets;
//! use ffmpeg_rs::filter::VideoFilter;
//! use ffmpeg_common::{Codec, Duration};
//!
//! # async fn example() -> ffmpeg_common::Result<()> {
//! let output = FFmpegBuilder::new()?
//!     .input(
//!         Input::new("input.mp4")
//!             .seek(Duration::from_secs(10))
//!             .duration(Duration::from_secs(30))
//!     )
//!     .output(
//!         Output::new("output.mp4")
//!             .video_codec_opts(presets::h264::youtube_1080p())
//!             .audio_codec(Codec::aac())
//!             .metadata("title", "My Video")
//!             .faststart()
//!     )
//!     .video_filter(VideoFilter::scale(1920, 1080))
//!     .overwrite()
//!     .on_progress(|p| {
//!         println!("Progress: {:?}", p);
//!     })
//!     .run()
//!     .await?;
//! # Ok(())
//! # }
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::must_use_candidate)]

pub mod builder;
pub mod codec;
pub mod filter;
pub mod format;
pub mod input;
pub mod output;
pub mod stream;

// Re-export main types
pub use builder::{FFmpegBuilder, FFmpegProcess};
pub use codec::CodecOptions;
pub use filter::{AudioFilter, FilterGraph, VideoFilter};
pub use format::FormatOptions;
pub use input::{ConcatInput, DeviceInput, Input, StreamInput};
pub use output::{ImageSequenceOutput, MultiOutput, Output};
pub use stream::{StreamDisposition, StreamMap, StreamMetadata, StreamSelection};

// Re-export from common
pub use ffmpeg_common::{
    get_version, Capabilities, Codec, Duration, Error, LogLevel, MediaPath, PixelFormat, Progress,
    Result, SampleFormat, Size, StreamSpecifier, StreamType, Version,
};

/// Prelude module for convenient imports
pub mod prelude {
    pub use crate::{
        FFmpegBuilder, Input, Output,
        codec::{CodecOptions, presets},
        filter::{AudioFilter, VideoFilter, chains},
        format::formats,
        stream::patterns,
    };
    pub use ffmpeg_common::{
        Codec, Duration, LogLevel, MediaPath, PixelFormat, Result, SampleFormat, StreamType,
    };
}

/// Get FFmpeg capabilities
pub async fn capabilities() -> Result<Capabilities> {
    Capabilities::detect("ffmpeg").await
}

/// Check if FFmpeg is available
pub async fn is_available() -> bool {
    ffmpeg_common::process::find_executable("ffmpeg").is_ok()
}

/// Get FFmpeg version
pub async fn version() -> Result<Version> {
    get_version("ffmpeg").await
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::*;

    #[test]
    fn test_simple_conversion() {
        let builder = FFmpegBuilder::convert("input.mp4", "output.webm");
        let args = builder.build_args().unwrap();

        assert!(args.contains(&"-i".to_string()));
        assert!(args.contains(&"input.mp4".to_string()));
        assert!(args.contains(&"output.webm".to_string()));
    }

    #[test]
    fn test_complex_build() {
        let builder = FFmpegBuilder::new()
            .unwrap()
            .input(
                Input::new("input.mp4")
                    .seek(Duration::from_secs(10))
            )
            .output(
                Output::new("output.mp4")
                    .video_codec(Codec::h264())
                    .audio_codec(Codec::aac())
                    .metadata("title", "Test Video")
            )
            .video_filter(VideoFilter::scale(1280, 720))
            .overwrite();

        let args = builder.build_args().unwrap();

        // Check input options
        assert!(args.contains(&"-ss".to_string()));
        assert!(args.contains(&"00:00:10".to_string()));

        // Check output options
        assert!(args.contains(&"-c:v".to_string()));
        assert!(args.contains(&"h264".to_string()));
        assert!(args.contains(&"-c:a".to_string()));
        assert!(args.contains(&"aac".to_string()));

        // Check filter
        assert!(args.contains(&"-vf".to_string()));
        assert!(args.iter().any(|arg| arg.contains("scale")));

        // Check overwrite
        assert!(args.contains(&"-y".to_string()));
    }

    #[test]
    fn test_preset_usage() {
        let builder = FFmpegBuilder::new()
            .unwrap()
            .input_path("input.mp4")
            .output(
                Output::new("output.mp4")
                    .video_codec_opts(presets::h264::youtube_1080p())
                    .audio_codec_opts(presets::audio::aac_high_quality())
            );

        let args = builder.build_args().unwrap();

        // Should have YouTube preset options
        assert!(args.iter().any(|arg| arg.contains("8000k")));
        assert!(args.contains(&"-profile:v".to_string()));
        assert!(args.contains(&"high".to_string()));
    }
}