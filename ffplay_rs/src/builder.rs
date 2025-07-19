use ffmpeg_common::{
    CommandBuilder, Duration, Error, LogLevel, MediaPath, Process, ProcessConfig, Result,
    StreamSpecifier,
};
use std::path::PathBuf;
use std::time::Duration as StdDuration;
use tracing::info;

use crate::display::DisplayOptions;
use crate::playback::{PlaybackOptions, SyncType};
use crate::types::ShowMode;

/// FFplay command builder
#[derive(Debug, Clone)]
pub struct FFplayBuilder {
    /// Path to ffplay executable
    executable: PathBuf,
    /// Input file or URL
    input: Option<MediaPath>,
    /// Display options
    display: DisplayOptions,
    /// Playback options
    playback: PlaybackOptions,
    /// Log level
    log_level: Option<LogLevel>,
    /// Additional raw arguments
    raw_args: Vec<String>,
    /// Process timeout
    timeout: Option<StdDuration>,
}

impl FFplayBuilder {
    /// Create a new FFplay command builder
    pub fn new() -> Result<Self> {
        let executable = ffmpeg_common::process::find_executable("ffplay")?;
        Ok(Self {
            executable,
            input: None,
            display: DisplayOptions::default(),
            playback: PlaybackOptions::default(),
            log_level: None,
            raw_args: Vec::new(),
            timeout: None,
        })
    }

    /// Create a builder with a custom FFplay executable path
    pub fn with_executable(path: impl Into<PathBuf>) -> Self {
        Self {
            executable: path.into(),
            input: None,
            display: DisplayOptions::default(),
            playback: PlaybackOptions::default(),
            log_level: None,
            raw_args: Vec::new(),
            timeout: None,
        }
    }

    /// Set input file or URL
    pub fn input(mut self, input: impl Into<MediaPath>) -> Self {
        self.input = Some(input.into());
        self
    }

    // Display options delegation

    /// Set window width
    pub fn width(mut self, width: u32) -> Self {
        self.display = self.display.width(width);
        self
    }

    /// Set window height
    pub fn height(mut self, height: u32) -> Self {
        self.display = self.display.height(height);
        self
    }

    /// Set window size
    pub fn size(mut self, width: u32, height: u32) -> Self {
        self.display = self.display.size(width, height);
        self
    }

    /// Start in fullscreen mode
    pub fn fullscreen(mut self, enable: bool) -> Self {
        self.display = self.display.fullscreen(enable);
        self
    }

    /// Set window title
    pub fn window_title(mut self, title: impl Into<String>) -> Self {
        self.display = self.display.window_title(title);
        self
    }

    /// Set window position
    pub fn window_position(mut self, x: i32, y: i32) -> Self {
        self.display = self.display.position(x, y);
        self
    }

    /// Borderless window
    pub fn borderless(mut self, enable: bool) -> Self {
        self.display = self.display.borderless(enable);
        self
    }

    /// Always on top
    pub fn always_on_top(mut self, enable: bool) -> Self {
        self.display = self.display.always_on_top(enable);
        self
    }

    /// Disable display
    pub fn no_display(mut self, enable: bool) -> Self {
        self.display = self.display.no_display(enable);
        self
    }

    /// Set show mode
    pub fn show_mode(mut self, mode: ShowMode) -> Self {
        self.display = self.display.show_mode(mode);
        self
    }

    // Playback options delegation

    /// Disable audio
    pub fn no_audio(mut self, enable: bool) -> Self {
        self.playback = self.playback.no_audio(enable);
        self
    }

    /// Disable video
    pub fn no_video(mut self, enable: bool) -> Self {
        self.playback = self.playback.no_video(enable);
        self
    }

    /// Disable subtitles
    pub fn no_subtitles(mut self, enable: bool) -> Self {
        self.playback = self.playback.no_subtitles(enable);
        self
    }

    /// Seek to position
    pub fn seek(mut self, position: Duration) -> Self {
        self.playback = self.playback.seek(position);
        self
    }

    /// Set duration to play
    pub fn duration(mut self, duration: Duration) -> Self {
        self.playback = self.playback.duration(duration);
        self
    }

    /// Loop playback
    pub fn loop_count(mut self, count: i32) -> Self {
        self.playback = self.playback.loop_count(count);
        self
    }

    /// Set volume (0-100)
    pub fn volume(mut self, volume: u8) -> Self {
        self.playback = self.playback.volume(volume);
        self
    }

    /// Enable fast mode
    pub fn fast(mut self, enable: bool) -> Self {
        self.playback = self.playback.fast(enable);
        self
    }

    /// Set sync type
    pub fn sync(mut self, sync_type: SyncType) -> Self {
        self.playback = self.playback.sync(sync_type);
        self
    }

    /// Enable autoexit
    pub fn autoexit(mut self, enable: bool) -> Self {
        self.playback = self.playback.autoexit(enable);
        self
    }

    /// Exit on key down
    pub fn exitonkeydown(mut self, enable: bool) -> Self {
        self.playback = self.playback.exitonkeydown(enable);
        self
    }

    /// Exit on mouse down
    pub fn exitonmousedown(mut self, enable: bool) -> Self {
        self.playback = self.playback.exitonmousedown(enable);
        self
    }

    /// Select audio stream
    pub fn audio_stream(mut self, spec: StreamSpecifier) -> Self {
        self.playback = self.playback.audio_stream(spec);
        self
    }

    /// Select video stream
    pub fn video_stream(mut self, spec: StreamSpecifier) -> Self {
        self.playback = self.playback.video_stream(spec);
        self
    }

    /// Select subtitle stream
    pub fn subtitle_stream(mut self, spec: StreamSpecifier) -> Self {
        self.playback = self.playback.subtitle_stream(spec);
        self
    }

    /// Set video filter
    pub fn video_filter(mut self, filter: impl Into<String>) -> Self {
        self.playback = self.playback.video_filter(filter);
        self
    }

    /// Set audio filter
    pub fn audio_filter(mut self, filter: impl Into<String>) -> Self {
        self.playback = self.playback.audio_filter(filter);
        self
    }

    /// Enable frame dropping
    pub fn framedrop(mut self, enable: bool) -> Self {
        self.playback = self.playback.framedrop(enable);
        self
    }

    /// Enable infinite buffer
    pub fn infbuf(mut self, enable: bool) -> Self {
        self.playback = self.playback.infbuf(enable);
        self
    }

    // Builder-specific options

    /// Set log level
    pub fn log_level(mut self, level: LogLevel) -> Self {
        self.log_level = Some(level);
        self
    }

    /// Add raw command line arguments
    pub fn raw_args(mut self, args: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.raw_args.extend(args.into_iter().map(Into::into));
        self
    }

    /// Set process timeout
    pub fn timeout(mut self, duration: StdDuration) -> Self {
        self.timeout = Some(duration);
        self
    }

    /// Validate the command
    fn validate(&self) -> Result<()> {
        if self.input.is_none() {
            return Err(Error::InvalidArgument("No input specified".to_string()));
        }
        Ok(())
    }

    /// Build command line arguments
    pub fn build_args(&self) -> Result<Vec<String>> {
        self.validate()?;

        let mut cmd = CommandBuilder::new();

        // Log level
        if let Some(level) = self.log_level {
            cmd = cmd.option("-loglevel", level.as_str());
        }

        // Display options
        cmd = cmd.args(self.display.build_args());

        // Playback options
        cmd = cmd.args(self.playback.build_args());

        // Raw arguments
        cmd = cmd.args(&self.raw_args);

        // Input file
        if let Some(ref input) = self.input {
            cmd = cmd.option("-i", input.as_str());
        }

        Ok(cmd.build())
    }

    /// Spawn FFplay process
    pub async fn spawn(self) -> Result<FFplayProcess> {
        let args = self.build_args()?;
        info!("Spawning FFplay with args: {:?}", args);

        let mut config = ProcessConfig::new(&self.executable)
            .capture_stdout(false)
            .capture_stderr(true);

        if let Some(timeout) = self.timeout {
            config = config.timeout(timeout);
        }

        let process = Process::spawn(config, args).await?;

        Ok(FFplayProcess { process })
    }

    /// Get the command that would be executed
    pub fn command(&self) -> Result<String> {
        let args = self.build_args()?;
        Ok(format!(
            "{} {}",
            self.executable.display(),
            args.join(" ")
        ))
    }
}

impl Default for FFplayBuilder {
    fn default() -> Self {
        Self::new().expect("FFplay executable not found")
    }
}

/// Handle to a running FFplay process
pub struct FFplayProcess {
    process: Process,
}

impl FFplayProcess {
    /// Wait for the process to complete
    pub async fn wait(self) -> Result<std::process::ExitStatus> {
        let output = self.process.wait().await?;
        Ok(output.status)
    }

    /// Kill the process
    pub async fn kill(&mut self) -> Result<()> {
        self.process.kill().await
    }

    /// Get the process ID
    pub fn id(&self) -> Option<u32> {
        self.process.id()
    }

    /// Try to wait without blocking
    pub fn try_wait(&mut self) -> Result<Option<std::process::ExitStatus>> {
        self.process.try_wait()
    }
}

/// Convenience functions for common playback scenarios
impl FFplayBuilder {
    /// Play a media file with default settings
    pub fn play(input: impl Into<MediaPath>) -> Self {
        Self::new().unwrap().input(input)
    }

    /// Play in fullscreen
    pub fn play_fullscreen(input: impl Into<MediaPath>) -> Self {
        Self::play(input).fullscreen(true)
    }

    /// Play audio only
    pub fn play_audio(input: impl Into<MediaPath>) -> Self {
        Self::play(input).no_video(true).no_display(true)
    }

    /// Play video only (no audio)
    pub fn play_video_only(input: impl Into<MediaPath>) -> Self {
        Self::play(input).no_audio(true)
    }

    /// Play with minimal UI
    pub fn play_minimal(input: impl Into<MediaPath>) -> Self {
        Self::play(input)
            .borderless(true)
            .exitonkeydown(true)
            .exitonmousedown(true)
    }

    /// Preview mode (first 10 seconds, auto-exit)
    pub fn preview(input: impl Into<MediaPath>) -> Self {
        Self::play(input)
            .duration(Duration::from_secs(10))
            .autoexit(true)
    }

    /// Slideshow mode for images
    pub fn slideshow(pattern: impl Into<MediaPath>) -> Self {
        Self::play(pattern)
            .loop_count(-1)
            .raw_args(["-framerate", "1"])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_playback() {
        let builder = FFplayBuilder::play("video.mp4");
        let args = builder.build_args().unwrap();

        assert!(args.contains(&"-i".to_string()));
        assert!(args.contains(&"video.mp4".to_string()));
    }

    #[test]
    fn test_display_options() {
        let builder = FFplayBuilder::play("video.mp4")
            .size(1280, 720)
            .fullscreen(true)
            .window_title("My Video");

        let args = builder.build_args().unwrap();
        assert!(args.contains(&"-x".to_string()));
        assert!(args.contains(&"1280".to_string()));
        assert!(args.contains(&"-y".to_string()));
        assert!(args.contains(&"720".to_string()));
        assert!(args.contains(&"-fs".to_string()));
        assert!(args.contains(&"-window_title".to_string()));
        assert!(args.contains(&"My Video".to_string()));
    }

    #[test]
    fn test_playback_options() {
        let builder = FFplayBuilder::play("video.mp4")
            .seek(Duration::from_secs(30))
            .duration(Duration::from_secs(60))
            .volume(50)
            .loop_count(3);

        let args = builder.build_args().unwrap();
        assert!(args.contains(&"-ss".to_string()));
        assert!(args.contains(&"00:00:30".to_string()));
        assert!(args.contains(&"-t".to_string()));
        assert!(args.contains(&"00:01:00".to_string()));
        assert!(args.contains(&"-volume".to_string()));
        assert!(args.contains(&"50".to_string()));
        assert!(args.contains(&"-loop".to_string()));
        assert!(args.contains(&"3".to_string()));
    }

    #[test]
    fn test_convenience_functions() {
        let fullscreen = FFplayBuilder::play_fullscreen("video.mp4");
        let args = fullscreen.build_args().unwrap();
        assert!(args.contains(&"-fs".to_string()));

        let audio_only = FFplayBuilder::play_audio("audio.mp3");
        let args = audio_only.build_args().unwrap();
        assert!(args.contains(&"-vn".to_string()));
        assert!(args.contains(&"-nodisp".to_string()));

        let preview = FFplayBuilder::preview("video.mp4");
        let args = preview.build_args().unwrap();
        assert!(args.contains(&"-t".to_string()));
        assert!(args.contains(&"-autoexit".to_string()));
    }
}