use ffmpeg_common::{
    process::stream_progress, CommandBuilder, Duration, Error, LogLevel, MediaPath, Process,
    ProcessConfig, ProcessOutput, Progress, Result, StreamSpecifier,
};
use std::fmt::Debug;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration as StdDuration;
use tracing::info;

use crate::filter::{AudioFilter, VideoFilter};
use crate::input::Input;
use crate::output::Output;
use crate::stream::StreamMap;

/// FFmpeg command builder
pub struct FFmpegBuilder {
    /// Path to ffmpeg executable
    executable: PathBuf,
    /// Global options (before -i)
    global_options: CommandBuilder,
    /// Input specifications
    inputs: Vec<Input>,
    /// Output specifications
    outputs: Vec<Output>,
    /// Stream mappings
    stream_maps: Vec<StreamMap>,
    /// Video filters
    video_filters: Vec<VideoFilter>,
    /// Audio filters
    audio_filters: Vec<AudioFilter>,
    /// Complex filter graph
    filter_complex: Option<String>,
    /// Log level
    log_level: Option<LogLevel>,
    /// Whether to overwrite output files
    overwrite: bool,
    /// Whether to never overwrite output files
    no_overwrite: bool,
    /// Time limit for encoding
    time_limit: Option<Duration>,
    /// File size limit
    file_size_limit: Option<u64>,
    /// Number of threads
    threads: Option<u32>,
    /// Hardware acceleration
    hwaccel: Option<String>,
    /// Additional raw arguments
    raw_args: Vec<String>,
    /// Progress callback, wrapped in an Arc for clonability.
    progress_callback: Option<Arc<dyn Fn(Progress) + Send + Sync>>,
    /// Process timeout
    timeout: Option<StdDuration>,
}

// Manual implementation of Debug to handle the non-Debug progress_callback field.
impl Debug for FFmpegBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FFmpegBuilder")
            .field("executable", &self.executable)
            .field("global_options", &self.global_options)
            .field("inputs", &self.inputs)
            .field("outputs", &self.outputs)
            .field("stream_maps", &self.stream_maps)
            .field("video_filters", &self.video_filters)
            .field("audio_filters", &self.audio_filters)
            .field("filter_complex", &self.filter_complex)
            .field("log_level", &self.log_level)
            .field("overwrite", &self.overwrite)
            .field("no_overwrite", &self.no_overwrite)
            .field("time_limit", &self.time_limit)
            .field("file_size_limit", &self.file_size_limit)
            .field("threads", &self.threads)
            .field("hwaccel", &self.hwaccel)
            .field("raw_args", &self.raw_args)
            .field(
                "progress_callback",
                // Print a placeholder for the function pointer.
                &self.progress_callback.as_ref().map(|_| "<function>"),
            )
            .field("timeout", &self.timeout)
            .finish()
    }
}

// Manual implementation of Clone to handle the non-Clone progress_callback field.
impl Clone for FFmpegBuilder {
    fn clone(&self) -> Self {
        Self {
            executable: self.executable.clone(),
            global_options: self.global_options.clone(),
            inputs: self.inputs.clone(),
            outputs: self.outputs.clone(),
            stream_maps: self.stream_maps.clone(),
            video_filters: self.video_filters.clone(),
            audio_filters: self.audio_filters.clone(),
            filter_complex: self.filter_complex.clone(),
            log_level: self.log_level,
            overwrite: self.overwrite,
            no_overwrite: self.no_overwrite,
            time_limit: self.time_limit,
            file_size_limit: self.file_size_limit,
            threads: self.threads,
            hwaccel: self.hwaccel.clone(),
            raw_args: self.raw_args.clone(),
            // Cloning an Arc just increments the reference count.
            progress_callback: self.progress_callback.clone(),
            timeout: self.timeout,
        }
    }
}

impl FFmpegBuilder {
    /// Create a new FFmpeg command builder
    pub fn new() -> Result<Self> {
        let executable = ffmpeg_common::process::find_executable("ffmpeg")?;
        Ok(Self {
            executable,
            global_options: CommandBuilder::new(),
            inputs: Vec::new(),
            outputs: Vec::new(),
            stream_maps: Vec::new(),
            video_filters: Vec::new(),
            audio_filters: Vec::new(),
            filter_complex: None,
            log_level: None,
            overwrite: false,
            no_overwrite: false,
            time_limit: None,
            file_size_limit: None,
            threads: None,
            hwaccel: None,
            raw_args: Vec::new(),
            progress_callback: None,
            timeout: None,
        })
    }

    /// Create a builder with a custom FFmpeg executable path
    pub fn with_executable(path: impl Into<PathBuf>) -> Self {
        Self {
            executable: path.into(),
            global_options: CommandBuilder::new(),
            inputs: Vec::new(),
            outputs: Vec::new(),
            stream_maps: Vec::new(),
            video_filters: Vec::new(),
            audio_filters: Vec::new(),
            filter_complex: None,
            log_level: None,
            overwrite: false,
            no_overwrite: false,
            time_limit: None,
            file_size_limit: None,
            threads: None,
            hwaccel: None,
            raw_args: Vec::new(),
            progress_callback: None,
            timeout: None,
        }
    }

    /// Add an input
    pub fn input(mut self, input: Input) -> Self {
        self.inputs.push(input);
        self
    }

    /// Add an input from a path
    pub fn input_path(self, path: impl Into<MediaPath>) -> Self {
        self.input(Input::new(path))
    }

    /// Add an output
    pub fn output(mut self, output: Output) -> Self {
        self.outputs.push(output);
        self
    }

    /// Add an output to a path
    pub fn output_path(self, path: impl Into<MediaPath>) -> Self {
        self.output(Output::new(path))
    }

    /// Map streams from input to output
    pub fn map(mut self, map: StreamMap) -> Self {
        self.stream_maps.push(map);
        self
    }

    /// Map all streams from an input
    pub fn map_all_from_input(self, input_index: usize) -> Self {
        self.map(StreamMap::from_input(input_index))
    }

    /// Map a specific stream
    pub fn map_stream(self, input_index: usize, stream_spec: StreamSpecifier) -> Self {
        self.map(StreamMap::specific(input_index, stream_spec))
    }

    /// Add a video filter
    pub fn video_filter(mut self, filter: VideoFilter) -> Self {
        self.video_filters.push(filter);
        self
    }

    /// Add an audio filter
    pub fn audio_filter(mut self, filter: AudioFilter) -> Self {
        self.audio_filters.push(filter);
        self
    }

    /// Set complex filter graph
    pub fn filter_complex(mut self, graph: impl Into<String>) -> Self {
        self.filter_complex = Some(graph.into());
        self
    }

    /// Set log level
    pub fn log_level(mut self, level: LogLevel) -> Self {
        self.log_level = Some(level);
        self
    }

    /// Enable overwriting output files
    pub fn overwrite(mut self) -> Self {
        self.overwrite = true;
        self.no_overwrite = false;
        self
    }

    /// Disable overwriting output files
    pub fn no_overwrite(mut self) -> Self {
        self.no_overwrite = true;
        self.overwrite = false;
        self
    }

    /// Set time limit for encoding
    pub fn time_limit(mut self, duration: Duration) -> Self {
        self.time_limit = Some(duration);
        self
    }

    /// Set file size limit
    pub fn file_size_limit(mut self, bytes: u64) -> Self {
        self.file_size_limit = Some(bytes);
        self
    }

    /// Set number of threads
    pub fn threads(mut self, count: u32) -> Self {
        self.threads = Some(count);
        self
    }

    /// Enable hardware acceleration
    pub fn hwaccel(mut self, method: impl Into<String>) -> Self {
        self.hwaccel = Some(method.into());
        self
    }

    /// Add raw command line arguments
    pub fn raw_args(mut self, args: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.raw_args.extend(args.into_iter().map(Into::into));
        self
    }

    /// Set progress callback
    pub fn on_progress<F>(mut self, callback: F) -> Self
    where
        F: Fn(Progress) + Send + Sync + 'static,
    {
        self.progress_callback = Some(Arc::new(callback));
        self
    }

    /// Set process timeout
    pub fn timeout(mut self, duration: StdDuration) -> Self {
        self.timeout = Some(duration);
        self
    }

    /// Validate the command
    fn validate(&self) -> Result<()> {
        if self.inputs.is_empty() {
            return Err(Error::InvalidArgument("No inputs specified".to_string()));
        }
        if self.outputs.is_empty() {
            return Err(Error::InvalidArgument("No outputs specified".to_string()));
        }
        Ok(())
    }

    /// Build the command line arguments
    pub fn build_args(&self) -> Result<Vec<String>> {
        self.validate()?;

        let mut cmd = CommandBuilder::new();

        // Global options
        if let Some(level) = self.log_level {
            cmd = cmd.option("-loglevel", level.as_str());
        }

        if self.overwrite {
            cmd = cmd.flag("-y");
        } else if self.no_overwrite {
            cmd = cmd.flag("-n");
        }

        if let Some(ref hwaccel) = self.hwaccel {
            cmd = cmd.option("-hwaccel", hwaccel);
        }

        if let Some(threads) = self.threads {
            cmd = cmd.option("-threads", threads);
        }

        // Add global options
        cmd = cmd.args(self.global_options.clone().build());

        // Input files
        for input in &self.inputs {
            cmd = cmd.args(input.build_args());
        }

        // Filters
        if !self.video_filters.is_empty() {
            let filter_str = self
                .video_filters
                .iter()
                .map(|f| f.to_string())
                .collect::<Vec<_>>()
                .join(",");
            cmd = cmd.option("-vf", filter_str);
        }

        if !self.audio_filters.is_empty() {
            let filter_str = self
                .audio_filters
                .iter()
                .map(|f| f.to_string())
                .collect::<Vec<_>>()
                .join(",");
            cmd = cmd.option("-af", filter_str);
        }

        if let Some(ref complex) = self.filter_complex {
            cmd = cmd.option("-filter_complex", complex);
        }

        // Stream mappings
        for map in &self.stream_maps {
            cmd = cmd.option("-map", map.to_string());
        }

        // Time and size limits
        if let Some(duration) = self.time_limit {
            cmd = cmd.option("-t", duration.to_ffmpeg_format());
        }

        if let Some(size) = self.file_size_limit {
            cmd = cmd.option("-fs", size);
        }

        // Raw arguments
        cmd = cmd.args(&self.raw_args);

        // Output files
        for output in &self.outputs {
            cmd = cmd.args(output.build_args());
        }

        Ok(cmd.build())
    }

    /// Run the FFmpeg command
    pub async fn run(self) -> Result<ProcessOutput> {
        let args = self.build_args()?;
        info!("Running FFmpeg with args: {:?}", args);

        let mut config = ProcessConfig::new(&self.executable)
            .capture_stdout(true)
            .capture_stderr(true);

        if let Some(timeout) = self.timeout {
            config = config.timeout(timeout);
        }

        let mut process = Process::spawn(config, args).await?;

        // Handle progress callback if set
        if let Some(callback) = self.progress_callback {
            if let Some(stderr) = process.stderr() {
                let stderr = tokio::io::BufReader::new(stderr);
                tokio::spawn(stream_progress(stderr, move |progress| {
                    callback(progress)
                }));
            }
        }

        process.wait().await?.into_result()
    }

    /// Run the command and return immediately with a process handle
    pub async fn spawn(self) -> Result<FFmpegProcess> {
        let args = self.build_args()?;
        info!("Spawning FFmpeg with args: {:?}", args);

        let mut config = ProcessConfig::new(&self.executable)
            .capture_stdout(true)
            .capture_stderr(true)
            .pipe_stdin(true);

        if let Some(timeout) = self.timeout {
            config = config.timeout(timeout);
        }

        let process = Process::spawn(config, args).await?;

        Ok(FFmpegProcess {
            process,
            progress_callback: self.progress_callback,
        })
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

impl Default for FFmpegBuilder {
    fn default() -> Self {
        Self::new().expect("FFmpeg executable not found")
    }
}

/// Handle to a running FFmpeg process
pub struct FFmpegProcess {
    process: Process,
    progress_callback: Option<Arc<dyn Fn(Progress) + Send + Sync>>,
}

impl FFmpegProcess {
    /// Wait for the process to complete
    pub async fn wait(mut self) -> Result<ProcessOutput> {
        // Handle progress callback if set
        if let Some(callback) = self.progress_callback {
            if let Some(stderr) = self.process.stderr() {
                let stderr = tokio::io::BufReader::new(stderr);
                tokio::spawn(stream_progress(stderr, move |progress| {
                    callback(progress)
                }));
            }
        }

        self.process.wait().await?.into_result()
    }

    /// Kill the process
    pub async fn kill(&mut self) -> Result<()> {
        self.process.kill().await
    }

    /// Get stdin handle for piping data
    pub fn stdin(&mut self) -> Option<tokio::process::ChildStdin> {
        self.process.stdin()
    }

    /// Get stdout handle
    pub fn stdout(&mut self) -> Option<tokio::process::ChildStdout> {
        self.process.stdout()
    }

    /// Try to wait without blocking
    pub fn try_wait(&mut self) -> Result<Option<std::process::ExitStatus>> {
        self.process.try_wait()
    }
}

/// Convenience functions for common FFmpeg operations
impl FFmpegBuilder {
    /// Create a simple conversion from input to output
    pub fn convert(input: impl Into<MediaPath>, output: impl Into<MediaPath>) -> Self {
        Self::new()
            .unwrap()
            .input_path(input)
            .output_path(output)
            .overwrite()
    }

    /// Extract audio from a video file
    pub fn extract_audio(
        input: impl Into<MediaPath>,
        output: impl Into<MediaPath>,
    ) -> Self {
        Self::new()
            .unwrap()
            .input_path(input)
            .output_path(output)
            .map_stream(0, StreamSpecifier::Type(ffmpeg_common::StreamType::Audio))
            .overwrite()
    }

    /// Extract video without audio
    pub fn extract_video(
        input: impl Into<MediaPath>,
        output: impl Into<MediaPath>,
    ) -> Self {
        Self::new()
            .unwrap()
            .input_path(input)
            .output_path(output)
            .map_stream(0, StreamSpecifier::Type(ffmpeg_common::StreamType::Video))
            .raw_args(["-an"])
            .overwrite()
    }

    /// Create a thumbnail at a specific time
    pub fn thumbnail(
        input: impl Into<MediaPath>,
        output: impl Into<MediaPath>,
        time: Duration,
    ) -> Self {
        Self::new()
            .unwrap()
            .input(Input::new(input).seek(time))
            .output(Output::new(output).frames(1))
            .overwrite()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_conversion() {
        let builder = FFmpegBuilder::new()
            .unwrap()
            .input_path("input.mp4")
            .output_path("output.mp4")
            .overwrite();

        let args = builder.build_args().unwrap();
        assert!(args.contains(&"-y".to_string()));
        assert!(args.contains(&"-i".to_string()));
        assert!(args.contains(&"input.mp4".to_string()));
        assert!(args.contains(&"output.mp4".to_string()));
    }

    #[test]
    fn test_extract_audio() {
        let builder = FFmpegBuilder::extract_audio("video.mp4", "audio.mp3");
        let args = builder.build_args().unwrap();

        assert!(args.contains(&"-map".to_string()));
        assert!(args.contains(&"0:a".to_string()));
    }

    #[test]
    fn test_validation() {
        let builder = FFmpegBuilder::new().unwrap();
        assert!(builder.build_args().is_err());

        let builder = FFmpegBuilder::new().unwrap().input_path("input.mp4");
        assert!(builder.build_args().is_err());
    }
}
