//! Safe and idiomatic Rust wrapper for FFplay
//!
//! This crate provides a high-level, safe interface to FFplay functionality,
//! allowing you to play multimedia files with various options.
//!
//! # Examples
//!
//! ## Basic playback
//! ```no_run
//! use ffplay_rs::FFplayBuilder;
//!
//! # async fn example() -> ffmpeg_common::Result<()> {
//! // Play a video file
//! let mut player = FFplayBuilder::play("video.mp4")
//!     .spawn()
//!     .await?;
//!
//! // Wait for playback to complete
//! player.wait().await?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Advanced usage
//! ```no_run
//! use ffplay_rs::{FFplayBuilder, ShowMode};
//! use ffplay_rs::playback::SyncType;
//! use ffmpeg_common::{Duration, StreamSpecifier};
//!
//! # async fn example() -> ffmpeg_common::Result<()> {
//! // Play with custom options
//! let mut player = FFplayBuilder::new()?
//!     .input("https://example.com/stream.m3u8")
//!     .size(1280, 720)
//!     .fullscreen(false)
//!     .window_title("My Stream")
//!     .seek(Duration::from_secs(30))
//!     .duration(Duration::from_secs(120))
//!     .volume(75)
//!     .audio_stream(StreamSpecifier::Index(1))
//!     .sync(SyncType::Audio)
//!     .autoexit(true)
//!     .spawn()
//!     .await?;
//!
//! // Kill the player after some time
//! tokio::time::sleep(std::time::Duration::from_secs(10)).await;
//! player.kill().await?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Audio visualization
//! ```no_run
//! use ffplay_rs::{FFplayBuilder, ShowMode};
//!
//! # async fn example() -> ffmpeg_common::Result<()> {
//! // Play audio with waveform visualization
//! let mut player = FFplayBuilder::play("audio.mp3")
//!     .show_mode(ShowMode::Waves)
//!     .window_title("Audio Player")
//!     .spawn()
//!     .await?;
//!
//! player.wait().await?;
//! # Ok(())
//! # }
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::must_use_candidate)]

pub mod builder;
pub mod display;
pub mod playback;
pub mod types;

// Re-export main types
pub use builder::{FFplayBuilder, FFplayProcess};
pub use display::DisplayOptions;
pub use playback::{PlaybackOptions, SyncType};
pub use types::{
    HwAccelOptions, KeyBinding, MouseAction, PlaybackState, ShowMode, VisualizationType,
    VulkanOptions, WindowState,
};

// Re-export from common
pub use ffmpeg_common::{
    get_version, Capabilities, Duration, Error, LogLevel, MediaPath, Result, StreamSpecifier,
    StreamType, Version,
};

/// Prelude module for convenient imports
pub mod prelude {
    pub use crate::{
        FFplayBuilder, ShowMode, SyncType,
        display::presets as display_presets,
        playback::presets as playback_presets,
    };
    pub use ffmpeg_common::{Duration, MediaPath, Result, StreamSpecifier};
}

/// Play a media file with default settings
pub async fn play(path: impl Into<MediaPath>) -> Result<FFplayProcess> {
    FFplayBuilder::play(path).spawn().await
}

/// Play in fullscreen
pub async fn play_fullscreen(path: impl Into<MediaPath>) -> Result<FFplayProcess> {
    FFplayBuilder::play_fullscreen(path).spawn().await
}

/// Play audio only
pub async fn play_audio(path: impl Into<MediaPath>) -> Result<FFplayProcess> {
    FFplayBuilder::play_audio(path).spawn().await
}

/// Get FFplay capabilities
pub async fn capabilities() -> Result<Capabilities> {
    Capabilities::detect("ffplay").await
}

/// Check if FFplay is available
pub async fn is_available() -> bool {
    ffmpeg_common::process::find_executable("ffplay").is_ok()
}

/// Get FFplay version
pub async fn version() -> Result<Version> {
    get_version("ffplay").await
}

/// Common playback scenarios
pub mod scenarios {
    use super::*;
    use crate::prelude::*;

    /// Play a video stream with low latency
    pub fn stream_low_latency(url: impl Into<MediaPath>) -> FFplayBuilder {
        FFplayBuilder::play(url)
            .framedrop(true)
            .infbuf(true)
            .sync(SyncType::External)
            .fast(true)
    }

    /// Play with hardware acceleration
    pub fn with_hw_accel(path: impl Into<MediaPath>) -> FFplayBuilder {
        let mut builder = FFplayBuilder::play(path);

        #[cfg(target_os = "linux")]
        {
            builder = builder.raw_args(["-hwaccel", "vaapi"]);
        }
        #[cfg(target_os = "macos")]
        {
            builder = builder.raw_args(["-hwaccel", "videotoolbox"]);
        }
        #[cfg(target_os = "windows")]
        {
            builder = builder.raw_args(["-hwaccel", "d3d11va"]);
        }

        builder
    }

    /// Create a video wall (multiple instances)
    pub async fn video_wall(
        paths: Vec<impl Into<MediaPath>>,
        grid_width: u32,
        grid_height: u32,
        window_width: u32,
        window_height: u32,
    ) -> Result<Vec<FFplayProcess>> {
        let mut players = Vec::new();

        for (i, path) in paths.into_iter().enumerate() {
            let row = i as u32 / grid_width;
            let col = i as u32 % grid_width;

            let x = col * window_width;
            let y = row * window_height;

            let player = FFplayBuilder::play(path)
                .size(window_width, window_height)
                .window_position(x as i32, y as i32)
                .borderless(true)
                .no_audio(i > 0) // Only first instance plays audio
                .spawn()
                .await?;

            players.push(player);
        }

        Ok(players)
    }

    /// Play with subtitle overlay
    pub fn with_subtitles(
        video: impl Into<MediaPath>,
        subtitle_file: impl Into<String>,
    ) -> FFplayBuilder {
        FFplayBuilder::play(video)
            .video_filter(format!("subtitles={}", subtitle_file.into()))
    }

    /// Play with custom aspect ratio
    pub fn with_aspect_ratio(path: impl Into<MediaPath>, ratio: &str) -> FFplayBuilder {
        FFplayBuilder::play(path)
            .video_filter(format!("setdar={}", ratio))
    }

    /// Play with deinterlacing
    pub fn deinterlaced(path: impl Into<MediaPath>) -> FFplayBuilder {
        FFplayBuilder::play(path)
            .video_filter("yadif")
    }

    /// Benchmark decoder performance
    pub fn benchmark(path: impl Into<MediaPath>) -> FFplayBuilder {
        FFplayBuilder::play(path)
            .no_display(true)
            .autoexit(true)
            .raw_args(["-benchmark"])
    }

    /// Loop a short video as animated wallpaper
    pub fn animated_wallpaper(path: impl Into<MediaPath>) -> FFplayBuilder {
        FFplayBuilder::play(path)
            .fullscreen(true)
            .loop_count(-1)
            .no_audio(true)
            .exitonkeydown(false)
            .exitonmousedown(false)
    }
}

/// Helper utilities
pub mod utils {
    use super::*;

    /// Get default window size based on video aspect ratio
    pub fn calculate_window_size(
        video_width: u32,
        video_height: u32,
        max_width: u32,
        max_height: u32,
    ) -> (u32, u32) {
        let video_aspect = video_width as f64 / video_height as f64;
        let max_aspect = max_width as f64 / max_height as f64;

        if video_aspect > max_aspect {
            // Video is wider than max area
            let height = (max_width as f64 / video_aspect) as u32;
            (max_width, height)
        } else {
            // Video is taller than max area
            let width = (max_height as f64 * video_aspect) as u32;
            (width, max_height)
        }
    }

    /// Create filter string for picture-in-picture
    pub fn pip_filter(
        main_video: &str,
        pip_video: &str,
        pip_scale: f32,
        pip_position: &str,
    ) -> String {
        format!(
            "[0:v][1:v]scale=iw*{}:ih*{}[pip];[0:v][pip]overlay={}",
            pip_scale, pip_scale, pip_position
        )
    }

    /// Create filter for side-by-side comparison
    pub fn side_by_side_filter() -> &'static str {
        "[0:v]pad=iw*2:ih[bg];[bg][1:v]overlay=W/2:0"
    }

    /// Get key bindings help text
    pub fn get_help_text() -> String {
        let bindings = types::get_key_bindings();
        let mut help = String::from("FFplay Key Bindings:\n\n");

        for (key, desc) in bindings {
            help.push_str(&format!("{:<20} {}\n", key, desc));
        }

        help
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::*;

    #[test]
    fn test_basic_play() {
        let builder = FFplayBuilder::play("test.mp4");
        let command = builder.command().unwrap();
        assert!(command.contains("ffplay"));
        assert!(command.contains("test.mp4"));
    }

    #[test]
    fn test_scenarios() {
        let low_latency = scenarios::stream_low_latency("rtmp://example.com/live");
        let args = low_latency.build_args().unwrap();
        assert!(args.contains(&"-framedrop".to_string()));
        assert!(args.contains(&"-infbuf".to_string()));
        assert!(args.contains(&"-sync".to_string()));
        assert!(args.contains(&"ext".to_string()));

        let deinterlaced = scenarios::deinterlaced("interlaced.mp4");
        let args = deinterlaced.build_args().unwrap();
        assert!(args.contains(&"-vf".to_string()));
        assert!(args.contains(&"yadif".to_string()));
    }

    #[test]
    fn test_utils() {
        // Test window size calculation
        let (w, h) = utils::calculate_window_size(1920, 1080, 1280, 720);
        assert_eq!(w, 1280);
        assert_eq!(h, 720);

        let (w, h) = utils::calculate_window_size(1080, 1920, 1280, 720);
        assert_eq!(w, 405);
        assert_eq!(h, 720);

        // Test help text
        let help = utils::get_help_text();
        assert!(help.contains("FFplay Key Bindings"));
        assert!(help.contains("Quit"));
    }
}