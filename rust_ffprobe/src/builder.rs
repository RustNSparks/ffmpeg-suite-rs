use ffmpeg_common::{
    CommandBuilder, Error, LogLevel, MediaPath, Process, ProcessConfig, Result, StreamSpecifier,
};
use std::path::PathBuf;
use std::time::Duration;
use tracing::info;

use crate::format::OutputFormat;
use crate::parsers::{parse_output, ProbeResult};
use crate::types::{ProbeSection, ReadInterval};

/// FFprobe command builder
#[derive(Debug, Clone)]
pub struct FFprobeBuilder {
    /// Path to ffprobe executable
    executable: PathBuf,
    /// Input file or URL
    input: Option<MediaPath>,
    /// Output format
    output_format: OutputFormat,
    /// Sections to show
    show_sections: Vec<ProbeSection>,
    /// Specific entries to show
    show_entries: Option<String>,
    /// Stream selection
    select_streams: Option<StreamSpecifier>,
    /// Whether to show data
    show_data: bool,
    /// Hash algorithm for data
    show_data_hash: Option<String>,
    /// Count frames
    count_frames: bool,
    /// Count packets
    count_packets: bool,
    /// Read intervals
    read_intervals: Vec<ReadInterval>,
    /// Show private data
    show_private_data: bool,
    /// Log level
    log_level: Option<LogLevel>,
    /// Pretty print
    pretty: bool,
    /// Unit display
    unit: bool,
    /// Prefix display
    prefix: bool,
    /// Byte binary prefix
    byte_binary_prefix: bool,
    /// Sexagesimal format
    sexagesimal: bool,
    /// Additional options
    options: Vec<(String, String)>,
    /// Process timeout
    timeout: Option<Duration>,
}

impl FFprobeBuilder {
    /// Create a new FFprobe command builder
    pub fn new() -> Result<Self> {
        let executable = ffmpeg_common::process::find_executable("ffprobe")?;
        Ok(Self {
            executable,
            input: None,
            output_format: OutputFormat::Json,
            show_sections: Vec::new(),
            show_entries: None,
            select_streams: None,
            show_data: false,
            show_data_hash: None,
            count_frames: false,
            count_packets: false,
            read_intervals: Vec::new(),
            show_private_data: true,
            log_level: None,
            pretty: false,
            unit: false,
            prefix: false,
            byte_binary_prefix: false,
            sexagesimal: false,
            options: Vec::new(),
            timeout: None,
        })
    }

    /// Create a builder with a custom FFprobe executable path
    pub fn with_executable(path: impl Into<PathBuf>) -> Self {
        Self {
            executable: path.into(),
            input: None,
            output_format: OutputFormat::Json,
            show_sections: Vec::new(),
            show_entries: None,
            select_streams: None,
            show_data: false,
            show_data_hash: None,
            count_frames: false,
            count_packets: false,
            read_intervals: Vec::new(),
            show_private_data: true,
            log_level: None,
            pretty: false,
            unit: false,
            prefix: false,
            byte_binary_prefix: false,
            sexagesimal: false,
            options: Vec::new(),
            timeout: None,
        }
    }

    /// Set input file or URL
    pub fn input(mut self, input: impl Into<MediaPath>) -> Self {
        self.input = Some(input.into());
        self
    }

    /// Set output format
    pub fn output_format(mut self, format: OutputFormat) -> Self {
        self.output_format = format;
        self
    }

    /// Show format information
    pub fn show_format(mut self) -> Self {
        if !self.show_sections.contains(&ProbeSection::Format) {
            self.show_sections.push(ProbeSection::Format);
        }
        self
    }

    /// Show stream information
    pub fn show_streams(mut self) -> Self {
        if !self.show_sections.contains(&ProbeSection::Streams) {
            self.show_sections.push(ProbeSection::Streams);
        }
        self
    }

    /// Show packet information
    pub fn show_packets(mut self) -> Self {
        if !self.show_sections.contains(&ProbeSection::Packets) {
            self.show_sections.push(ProbeSection::Packets);
        }
        self
    }

    /// Show frame information
    pub fn show_frames(mut self) -> Self {
        if !self.show_sections.contains(&ProbeSection::Frames) {
            self.show_sections.push(ProbeSection::Frames);
        }
        self
    }

    /// Show program information
    pub fn show_programs(mut self) -> Self {
        if !self.show_sections.contains(&ProbeSection::Programs) {
            self.show_sections.push(ProbeSection::Programs);
        }
        self
    }

    /// Show chapter information
    pub fn show_chapters(mut self) -> Self {
        if !self.show_sections.contains(&ProbeSection::Chapters) {
            self.show_sections.push(ProbeSection::Chapters);
        }
        self
    }

    /// Show error information
    pub fn show_error(mut self) -> Self {
        if !self.show_sections.contains(&ProbeSection::Error) {
            self.show_sections.push(ProbeSection::Error);
        }
        self
    }

    /// Show specific entries
    pub fn show_entries(mut self, entries: impl Into<String>) -> Self {
        self.show_entries = Some(entries.into());
        self
    }

    /// Select specific streams
    pub fn select_streams(mut self, spec: StreamSpecifier) -> Self {
        self.select_streams = Some(spec);
        self
    }

    /// Show data dump
    pub fn show_data(mut self, enable: bool) -> Self {
        self.show_data = enable;
        self
    }

    /// Show data hash
    pub fn show_data_hash(mut self, algorithm: impl Into<String>) -> Self {
        self.show_data_hash = Some(algorithm.into());
        self
    }

    /// Count frames
    pub fn count_frames(mut self, enable: bool) -> Self {
        self.count_frames = enable;
        self
    }

    /// Count packets
    pub fn count_packets(mut self, enable: bool) -> Self {
        self.count_packets = enable;
        self
    }

    /// Add read interval
    pub fn read_interval(mut self, interval: ReadInterval) -> Self {
        self.read_intervals.push(interval);
        self
    }

    /// Show private data
    pub fn show_private_data(mut self, enable: bool) -> Self {
        self.show_private_data = enable;
        self
    }

    /// Set log level
    pub fn log_level(mut self, level: LogLevel) -> Self {
        self.log_level = Some(level);
        self
    }

    /// Enable pretty printing
    pub fn pretty(mut self, enable: bool) -> Self {
        self.pretty = enable;
        self
    }

    /// Show units
    pub fn unit(mut self, enable: bool) -> Self {
        self.unit = enable;
        self
    }

    /// Show prefixes
    pub fn prefix(mut self, enable: bool) -> Self {
        self.prefix = enable;
        self
    }

    /// Use byte binary prefix
    pub fn byte_binary_prefix(mut self, enable: bool) -> Self {
        self.byte_binary_prefix = enable;
        self
    }

    /// Use sexagesimal time format
    pub fn sexagesimal(mut self, enable: bool) -> Self {
        self.sexagesimal = enable;
        self
    }

    /// Add custom option
    pub fn option(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.options.push((key.into(), value.into()));
        self
    }

    /// Set process timeout
    pub fn timeout(mut self, duration: Duration) -> Self {
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

        // Output format
        cmd = cmd.option("-print_format", self.output_format.as_str());

        // Show sections
        for section in &self.show_sections {
            cmd = cmd.flag(format!("-show_{}", section.as_str()));
        }

        // Show entries
        if let Some(ref entries) = self.show_entries {
            cmd = cmd.option("-show_entries", entries);
        }

        // Stream selection
        if let Some(ref spec) = self.select_streams {
            cmd = cmd.option("-select_streams", spec.to_string());
        }

        // Data options
        if self.show_data {
            cmd = cmd.flag("-show_data");
        }

        if let Some(ref hash) = self.show_data_hash {
            cmd = cmd.option("-show_data_hash", hash);
        }

        // Counting
        if self.count_frames {
            cmd = cmd.flag("-count_frames");
        }

        if self.count_packets {
            cmd = cmd.flag("-count_packets");
        }

        // Read intervals
        if !self.read_intervals.is_empty() {
            let intervals = self
                .read_intervals
                .iter()
                .map(|i| i.to_string())
                .collect::<Vec<_>>()
                .join(",");
            cmd = cmd.option("-read_intervals", intervals);
        }

        // Private data
        if !self.show_private_data {
            cmd = cmd.flag("-noprivate");
        }

        // Display options
        if self.pretty {
            cmd = cmd.flag("-pretty");
        } else {
            if self.unit {
                cmd = cmd.flag("-unit");
            }
            if self.prefix {
                cmd = cmd.flag("-prefix");
            }
            if self.byte_binary_prefix {
                cmd = cmd.flag("-byte_binary_prefix");
            }
            if self.sexagesimal {
                cmd = cmd.flag("-sexagesimal");
            }
        }

        // Custom options
        for (key, value) in &self.options {
            cmd = cmd.option(key, value);
        }

        // Input file
        if let Some(ref input) = self.input {
            cmd = cmd.arg(input.as_str());
        }

        Ok(cmd.build())
    }

    /// Run FFprobe and parse the output
    pub async fn run(self) -> Result<ProbeResult> {
        let args = self.build_args()?;
        info!("Running FFprobe with args: {:?}", args);

        let mut config = ProcessConfig::new(&self.executable)
            .capture_stdout(true)
            .capture_stderr(true);

        if let Some(timeout) = self.timeout {
            config = config.timeout(timeout);
        }

        let output = Process::spawn(config, args).await?.wait().await?;

        if !output.success() {
            return Err(Error::process_failed(
                "FFprobe failed",
                Some(output.status),
                output.stderr_str(),
            ));
        }

        let stdout = output
            .stdout_str()
            .ok_or_else(|| Error::InvalidOutput("No output from ffprobe".to_string()))?;

        parse_output(&stdout, self.output_format)
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

impl Default for FFprobeBuilder {
    fn default() -> Self {
        Self::new().expect("FFprobe executable not found")
    }
}

/// Quick probe functions
impl FFprobeBuilder {
    /// Quick probe for basic format and stream info
    pub fn probe(input: impl Into<MediaPath>) -> Self {
        Self::new()
            .unwrap()
            .input(input)
            .show_format()
            .show_streams()
    }

    /// Probe only format info
    pub fn probe_format(input: impl Into<MediaPath>) -> Self {
        Self::new()
            .unwrap()
            .input(input)
            .show_format()
    }

    /// Probe only stream info
    pub fn probe_streams(input: impl Into<MediaPath>) -> Self {
        Self::new()
            .unwrap()
            .input(input)
            .show_streams()
    }

    /// Detailed probe with everything
    pub fn probe_detailed(input: impl Into<MediaPath>) -> Self {
        Self::new()
            .unwrap()
            .input(input)
            .show_format()
            .show_streams()
            .show_chapters()
            .show_programs()
            .count_frames(true)
            .count_packets(true)
    }

    /// Probe with specific stream selection
    pub fn probe_stream(input: impl Into<MediaPath>, stream: StreamSpecifier) -> Self {
        Self::new()
            .unwrap()
            .input(input)
            .show_streams()
            .select_streams(stream)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ffmpeg_common::StreamType;

    #[test]
    fn test_basic_probe() {
        let builder = FFprobeBuilder::probe("input.mp4");
        let args = builder.build_args().unwrap();

        assert!(args.contains(&"-print_format".to_string()));
        assert!(args.contains(&"json".to_string()));
        assert!(args.contains(&"-show_format".to_string()));
        assert!(args.contains(&"-show_streams".to_string()));
        assert!(args.contains(&"input.mp4".to_string()));
    }

    #[test]
    fn test_output_format() {
        let builder = FFprobeBuilder::new()
            .unwrap()
            .input("input.mp4")
            .output_format(OutputFormat::Xml)
            .show_format();

        let args = builder.build_args().unwrap();
        assert!(args.contains(&"xml".to_string()));
    }

    #[test]
    fn test_stream_selection() {
        let builder = FFprobeBuilder::probe_stream(
            "input.mp4",
            StreamSpecifier::Type(StreamType::Audio),
        );

        let args = builder.build_args().unwrap();
        assert!(args.contains(&"-select_streams".to_string()));
        assert!(args.contains(&"a".to_string()));
    }

    #[test]
    fn test_display_options() {
        let builder = FFprobeBuilder::new()
            .unwrap()
            .input("input.mp4")
            .pretty(true);

        let args = builder.build_args().unwrap();
        assert!(args.contains(&"-pretty".to_string()));
    }
}