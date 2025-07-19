use serde::{Deserialize, Serialize};
use std::fmt;
use std::path::PathBuf;
use std::str::FromStr;
use std::time::Duration as StdDuration;

use crate::error::{Error, Result};

/// Represents a duration in FFmpeg format (HH:MM:SS.MS or seconds)
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Duration(StdDuration);

impl Duration {
    /// Create a new duration from seconds
    pub fn from_secs(secs: u64) -> Self {
        Self(StdDuration::from_secs(secs))
    }

    /// Create a new duration from milliseconds
    pub fn from_millis(millis: u64) -> Self {
        Self(StdDuration::from_millis(millis))
    }

    /// Get the duration as seconds
    pub fn as_secs(&self) -> u64 {
        self.0.as_secs()
    }

    /// Get the duration as milliseconds
    pub fn as_millis(&self) -> u128 {
        self.0.as_millis()
    }

    /// Convert to FFmpeg time format (HH:MM:SS.MS)
    pub fn to_ffmpeg_format(&self) -> String {
        let total_secs = self.0.as_secs();
        let hours = total_secs / 3600;
        let minutes = (total_secs % 3600) / 60;
        let seconds = total_secs % 60;
        let millis = self.0.subsec_millis();

        if millis > 0 {
            format!("{:02}:{:02}:{:02}.{:03}", hours, minutes, seconds, millis)
        } else {
            format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
        }
    }

    /// Parse from FFmpeg time format
    pub fn from_ffmpeg_format(s: &str) -> Result<Self> {
        // Handle pure seconds format
        if let Ok(secs) = s.parse::<f64>() {
            return Ok(Self(StdDuration::from_secs_f64(secs)));
        }

        // Handle HH:MM:SS[.MS] format
        let parts: Vec<&str> = s.split(':').collect();
        if parts.len() != 3 {
            return Err(Error::ParseError(format!("Invalid time format: {}", s)));
        }

        let hours: u64 = parts[0].parse()
            .map_err(|_| Error::ParseError(format!("Invalid hours: {}", parts[0])))?;
        let minutes: u64 = parts[1].parse()
            .map_err(|_| Error::ParseError(format!("Invalid minutes: {}", parts[1])))?;

        let (seconds, millis) = if parts[2].contains('.') {
            let sec_parts: Vec<&str> = parts[2].split('.').collect();
            let secs: u64 = sec_parts[0].parse()
                .map_err(|_| Error::ParseError(format!("Invalid seconds: {}", sec_parts[0])))?;
            let ms: u64 = sec_parts[1].parse()
                .map_err(|_| Error::ParseError(format!("Invalid milliseconds: {}", sec_parts[1])))?;
            (secs, ms)
        } else {
            let secs: u64 = parts[2].parse()
                .map_err(|_| Error::ParseError(format!("Invalid seconds: {}", parts[2])))?;
            (secs, 0)
        };

        let total_millis = (hours * 3600 + minutes * 60 + seconds) * 1000 + millis;
        Ok(Self(StdDuration::from_millis(total_millis)))
    }
}

impl From<StdDuration> for Duration {
    fn from(d: StdDuration) -> Self {
        Self(d)
    }
}

impl From<Duration> for StdDuration {
    fn from(d: Duration) -> Self {
        d.0
    }
}

impl fmt::Display for Duration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_ffmpeg_format())
    }
}

impl FromStr for Duration {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Self::from_ffmpeg_format(s)
    }
}

/// Represents a size in bytes with SI/binary prefixes support
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Size(u64);

impl Size {
    /// Create a new size in bytes
    pub fn from_bytes(bytes: u64) -> Self {
        Self(bytes)
    }

    /// Create from kilobytes
    pub fn from_kb(kb: u64) -> Self {
        Self(kb * 1000)
    }

    /// Create from megabytes
    pub fn from_mb(mb: u64) -> Self {
        Self(mb * 1_000_000)
    }

    /// Create from gigabytes
    pub fn from_gb(gb: u64) -> Self {
        Self(gb * 1_000_000_000)
    }

    /// Create from kibibytes
    pub fn from_kib(kib: u64) -> Self {
        Self(kib * 1024)
    }

    /// Create from mebibytes
    pub fn from_mib(mib: u64) -> Self {
        Self(mib * 1024 * 1024)
    }

    /// Create from gibibytes
    pub fn from_gib(gib: u64) -> Self {
        Self(gib * 1024 * 1024 * 1024)
    }

    /// Get size in bytes
    pub fn as_bytes(&self) -> u64 {
        self.0
    }

    /// Parse size from string with optional suffix
    pub fn parse(s: &str) -> Result<Self> {
        let s = s.trim();

        // Extract number and suffix
        let (num_str, suffix) = s
            .find(|c: char| c.is_alphabetic())
            .map(|i| s.split_at(i))
            .unwrap_or((s, ""));

        let number: f64 = num_str.parse()
            .map_err(|_| Error::ParseError(format!("Invalid number: {}", num_str)))?;

        let multiplier = match suffix.to_uppercase().as_str() {
            "" | "B" => 1.0,
            "K" | "KB" => 1_000.0,
            "M" | "MB" => 1_000_000.0,
            "G" | "GB" => 1_000_000_000.0,
            "KI" | "KIB" => 1_024.0,
            "MI" | "MIB" => 1_048_576.0,
            "GI" | "GIB" => 1_073_741_824.0,
            _ => return Err(Error::ParseError(format!("Invalid size suffix: {}", suffix))),
        };

        Ok(Self((number * multiplier) as u64))
    }
}

impl fmt::Display for Size {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for Size {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Self::parse(s)
    }
}

/// Represents a stream specifier in FFmpeg
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StreamSpecifier {
    /// Stream by index
    Index(usize),
    /// Stream by type (v, a, s, d, t)
    Type(StreamType),
    /// Stream by type and index
    TypeIndex(StreamType, usize),
    /// All streams
    All,
    /// Program ID
    Program(usize),
    /// Stream ID
    StreamId(String),
    /// Metadata key/value
    Metadata { key: String, value: Option<String> },
    /// Usable streams
    Usable,
}

impl StreamSpecifier {
    /// Convert to FFmpeg command-line format
    pub fn to_string(&self) -> String {
        match self {
            Self::Index(i) => i.to_string(),
            Self::Type(t) => t.to_string(),
            Self::TypeIndex(t, i) => format!("{}:{}", t, i),
            Self::All => String::new(),
            Self::Program(id) => format!("p:{}", id),
            Self::StreamId(id) => format!("#{}", id),
            Self::Metadata { key, value } => {
                if let Some(val) = value {
                    format!("m:{}:{}", key, val)
                } else {
                    format!("m:{}", key)
                }
            }
            Self::Usable => "u".to_string(),
        }
    }
}

impl fmt::Display for StreamSpecifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

/// Stream type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StreamType {
    Video,
    VideoNoAttached,
    Audio,
    Subtitle,
    Data,
    Attachment,
}

impl StreamType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Video => "v",
            Self::VideoNoAttached => "V",
            Self::Audio => "a",
            Self::Subtitle => "s",
            Self::Data => "d",
            Self::Attachment => "t",
        }
    }
}

impl fmt::Display for StreamType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Log level for FFmpeg tools
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogLevel {
    Quiet,
    Panic,
    Fatal,
    Error,
    Warning,
    Info,
    Verbose,
    Debug,
    Trace,
}

impl LogLevel {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Quiet => "quiet",
            Self::Panic => "panic",
            Self::Fatal => "fatal",
            Self::Error => "error",
            Self::Warning => "warning",
            Self::Info => "info",
            Self::Verbose => "verbose",
            Self::Debug => "debug",
            Self::Trace => "trace",
        }
    }

    pub fn as_number(&self) -> i32 {
        match self {
            Self::Quiet => -8,
            Self::Panic => 0,
            Self::Fatal => 8,
            Self::Error => 16,
            Self::Warning => 24,
            Self::Info => 32,
            Self::Verbose => 40,
            Self::Debug => 48,
            Self::Trace => 56,
        }
    }
}

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Pixel format
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PixelFormat(String);

impl PixelFormat {
    pub fn new(format: impl Into<String>) -> Self {
        Self(format.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    // Common pixel formats
    pub fn yuv420p() -> Self {
        Self("yuv420p".to_string())
    }

    pub fn yuv422p() -> Self {
        Self("yuv422p".to_string())
    }

    pub fn yuv444p() -> Self {
        Self("yuv444p".to_string())
    }

    pub fn rgb24() -> Self {
        Self("rgb24".to_string())
    }

    pub fn bgr24() -> Self {
        Self("bgr24".to_string())
    }

    pub fn rgba() -> Self {
        Self("rgba".to_string())
    }

    pub fn bgra() -> Self {
        Self("bgra".to_string())
    }

    pub fn gray() -> Self {
        Self("gray".to_string())
    }

    pub fn nv12() -> Self {
        Self("nv12".to_string())
    }
}

impl fmt::Display for PixelFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Audio sample format
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SampleFormat(String);

impl SampleFormat {
    pub fn new(format: impl Into<String>) -> Self {
        Self(format.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    // Common sample formats
    pub fn u8() -> Self {
        Self("u8".to_string())
    }

    pub fn s16() -> Self {
        Self("s16".to_string())
    }

    pub fn s32() -> Self {
        Self("s32".to_string())
    }

    pub fn flt() -> Self {
        Self("flt".to_string())
    }

    pub fn dbl() -> Self {
        Self("dbl".to_string())
    }

    pub fn u8p() -> Self {
        Self("u8p".to_string())
    }

    pub fn s16p() -> Self {
        Self("s16p".to_string())
    }

    pub fn s32p() -> Self {
        Self("s32p".to_string())
    }

    pub fn fltp() -> Self {
        Self("fltp".to_string())
    }

    pub fn dblp() -> Self {
        Self("dblp".to_string())
    }
}

impl fmt::Display for SampleFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Codec name
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Codec(String);

impl Codec {
    pub fn new(codec: impl Into<String>) -> Self {
        Self(codec.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    // Common video codecs
    pub fn h264() -> Self {
        Self("h264".to_string())
    }

    pub fn h265() -> Self {
        Self("h265".to_string())
    }

    pub fn vp9() -> Self {
        Self("vp9".to_string())
    }

    pub fn av1() -> Self {
        Self("av1".to_string())
    }

    pub fn mpeg2video() -> Self {
        Self("mpeg2video".to_string())
    }

    pub fn mpeg4() -> Self {
        Self("mpeg4".to_string())
    }

    // Common audio codecs
    pub fn aac() -> Self {
        Self("aac".to_string())
    }

    pub fn mp3() -> Self {
        Self("mp3".to_string())
    }

    pub fn opus() -> Self {
        Self("opus".to_string())
    }

    pub fn flac() -> Self {
        Self("flac".to_string())
    }

    pub fn ac3() -> Self {
        Self("ac3".to_string())
    }

    pub fn pcm_s16le() -> Self {
        Self("pcm_s16le".to_string())
    }

    // Copy codec
    pub fn copy() -> Self {
        Self("copy".to_string())
    }
}

impl fmt::Display for Codec {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Input or output file/URL
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MediaPath {
    path: PathBuf,
    is_url: bool,
}

impl MediaPath {
    /// Create from a file path
    pub fn from_path(path: impl Into<PathBuf>) -> Self {
        Self {
            path: path.into(),
            is_url: false,
        }
    }

    /// Create from a URL
    pub fn from_url(url: impl Into<String>) -> Self {
        Self {
            path: PathBuf::from(url.into()),
            is_url: true,
        }
    }

    /// Parse from string, auto-detecting URLs
    pub fn parse(s: impl AsRef<str>) -> Self {
        let s = s.as_ref();
        if s.contains("://") || s.starts_with("rtmp") || s.starts_with("rtsp") {
            Self::from_url(s)
        } else {
            Self::from_path(s)
        }
    }

    /// Get as string for command line
    pub fn as_str(&self) -> &str {
        self.path.to_str().unwrap_or("")
    }

    /// Check if this is a URL
    pub fn is_url(&self) -> bool {
        self.is_url
    }

    /// Check if this is a file path
    pub fn is_file(&self) -> bool {
        !self.is_url
    }

    /// Get the path
    pub fn path(&self) -> &PathBuf {
        &self.path
    }
}

impl fmt::Display for MediaPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl From<PathBuf> for MediaPath {
    fn from(path: PathBuf) -> Self {
        Self::from_path(path)
    }
}

impl From<&str> for MediaPath {
    fn from(s: &str) -> Self {
        Self::parse(s)
    }
}

impl From<String> for MediaPath {
    fn from(s: String) -> Self {
        Self::parse(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_duration_parsing() {
        assert_eq!(Duration::from_ffmpeg_format("10").unwrap().as_secs(), 10);
        assert_eq!(Duration::from_ffmpeg_format("01:30:00").unwrap().as_secs(), 5400);
        assert_eq!(Duration::from_ffmpeg_format("00:00:30.500").unwrap().as_millis(), 30500);
    }

    #[test]
    fn test_duration_formatting() {
        assert_eq!(Duration::from_secs(90).to_ffmpeg_format(), "00:01:30");
        assert_eq!(Duration::from_millis(30500).to_ffmpeg_format(), "00:00:30.500");
    }

    #[test]
    fn test_size_parsing() {
        assert_eq!(Size::parse("1024").unwrap().as_bytes(), 1024);
        assert_eq!(Size::parse("10K").unwrap().as_bytes(), 10_000);
        assert_eq!(Size::parse("10KB").unwrap().as_bytes(), 10_000);
        assert_eq!(Size::parse("10KiB").unwrap().as_bytes(), 10_240);
        assert_eq!(Size::parse("1.5M").unwrap().as_bytes(), 1_500_000);
    }

    #[test]
    fn test_stream_specifier() {
        assert_eq!(StreamSpecifier::Index(1).to_string(), "1");
        assert_eq!(StreamSpecifier::Type(StreamType::Audio).to_string(), "a");
        assert_eq!(StreamSpecifier::TypeIndex(StreamType::Video, 0).to_string(), "v:0");
        assert_eq!(StreamSpecifier::Program(1).to_string(), "p:1");
    }

    #[test]
    fn test_media_path() {
        let file = MediaPath::parse("/path/to/file.mp4");
        assert!(file.is_file());
        assert!(!file.is_url());

        let url = MediaPath::parse("https://example.com/video.mp4");
        assert!(url.is_url());
        assert!(!url.is_file());

        let rtmp = MediaPath::parse("rtmp://server/live/stream");
        assert!(rtmp.is_url());
    }
}