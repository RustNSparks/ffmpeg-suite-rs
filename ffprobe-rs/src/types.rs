use ffmpeg_common::Duration;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

/// Sections that can be probed
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProbeSection {
    Format,
    Streams,
    Packets,
    Frames,
    Programs,
    Chapters,
    Error,
}

impl ProbeSection {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Format => "format",
            Self::Streams => "streams",
            Self::Packets => "packets",
            Self::Frames => "frames",
            Self::Programs => "programs",
            Self::Chapters => "chapters",
            Self::Error => "error",
        }
    }
}

/// Read interval specification
#[derive(Debug, Clone)]
pub struct ReadInterval {
    /// Start position (None = from beginning)
    pub start: Option<IntervalPosition>,
    /// End position (None = to end)
    pub end: Option<IntervalPosition>,
}

#[derive(Debug, Clone)]
pub enum IntervalPosition {
    /// Absolute position
    Absolute(Duration),
    /// Relative offset from current position
    Relative(Duration),
    /// Number of packets
    Packets(u64),
}

impl ReadInterval {
    /// Create interval from start to end
    pub fn new(start: Option<IntervalPosition>, end: Option<IntervalPosition>) -> Self {
        Self { start, end }
    }

    /// Read from beginning to position
    pub fn to(end: IntervalPosition) -> Self {
        Self {
            start: None,
            end: Some(end),
        }
    }

    /// Read from position to end
    pub fn from(start: IntervalPosition) -> Self {
        Self {
            start: Some(start),
            end: None,
        }
    }

    /// Read entire input
    pub fn all() -> Self {
        Self {
            start: None,
            end: None,
        }
    }

    /// Convert to FFprobe format
    pub fn to_string(&self) -> String {
        let mut result = String::new();

        if let Some(ref start) = self.start {
            match start {
                IntervalPosition::Absolute(d) => result.push_str(&d.to_ffmpeg_format()),
                IntervalPosition::Relative(d) => result.push_str(&format!("+{}", d.to_ffmpeg_format())),
                IntervalPosition::Packets(n) => result.push_str(&format!("#{}", n)),
            }
        }

        result.push('%');

        if let Some(ref end) = self.end {
            match end {
                IntervalPosition::Absolute(d) => result.push_str(&d.to_ffmpeg_format()),
                IntervalPosition::Relative(d) => result.push_str(&format!("+{}", d.to_ffmpeg_format())),
                IntervalPosition::Packets(n) => result.push_str(&format!("#{}", n)),
            }
        }

        result
    }
}

impl fmt::Display for ReadInterval {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

/// Main probe result structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProbeResult {
    /// Format information
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<FormatInfo>,

    /// Stream information
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub streams: Vec<StreamInfo>,

    /// Packet information
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub packets: Vec<PacketInfo>,

    /// Frame information
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub frames: Vec<FrameInfo>,

    /// Program information
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub programs: Vec<ProgramInfo>,

    /// Chapter information
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub chapters: Vec<ChapterInfo>,

    /// Error information
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorInfo>,
}

impl ProbeResult {
    /// Get video streams
    pub fn video_streams(&self) -> Vec<&StreamInfo> {
        self.streams
            .iter()
            .filter(|s| s.codec_type == Some("video".to_string()))
            .collect()
    }

    /// Get audio streams
    pub fn audio_streams(&self) -> Vec<&StreamInfo> {
        self.streams
            .iter()
            .filter(|s| s.codec_type == Some("audio".to_string()))
            .collect()
    }

    /// Get subtitle streams
    pub fn subtitle_streams(&self) -> Vec<&StreamInfo> {
        self.streams
            .iter()
            .filter(|s| s.codec_type == Some("subtitle".to_string()))
            .collect()
    }

    /// Get the primary video stream
    pub fn primary_video_stream(&self) -> Option<&StreamInfo> {
        self.video_streams().into_iter().next()
    }

    /// Get the primary audio stream
    pub fn primary_audio_stream(&self) -> Option<&StreamInfo> {
        self.audio_streams().into_iter().next()
    }

    /// Get total duration
    pub fn duration(&self) -> Option<f64> {
        self.format.as_ref()?.duration.as_ref()?.parse().ok()
    }

    /// Get format name
    pub fn format_name(&self) -> Option<&str> {
        self.format.as_ref()?.format_name.as_deref()
    }

    /// Get format long name
    pub fn format_long_name(&self) -> Option<&str> {
        self.format.as_ref()?.format_long_name.as_deref()
    }
}

/// Format information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormatInfo {
    /// Filename
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,

    /// Number of streams
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nb_streams: Option<u32>,

    /// Number of programs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nb_programs: Option<u32>,

    /// Format name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format_name: Option<String>,

    /// Format long name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format_long_name: Option<String>,

    /// Start time in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,

    /// Duration in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,

    /// File size in bytes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,

    /// Bit rate
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bit_rate: Option<String>,

    /// Probe score
    #[serde(skip_serializing_if = "Option::is_none")]
    pub probe_score: Option<u32>,

    /// Tags/metadata
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub tags: HashMap<String, String>,
}

/// Stream information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamInfo {
    /// Stream index
    pub index: u32,

    /// Codec name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub codec_name: Option<String>,

    /// Codec long name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub codec_long_name: Option<String>,

    /// Profile
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile: Option<String>,

    /// Codec type (video/audio/subtitle/data)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub codec_type: Option<String>,

    /// Codec tag string
    #[serde(skip_serializing_if = "Option::is_none")]
    pub codec_tag_string: Option<String>,

    /// Codec tag
    #[serde(skip_serializing_if = "Option::is_none")]
    pub codec_tag: Option<String>,

    // Video specific
    /// Width
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<u32>,

    /// Height
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<u32>,

    /// Coded width
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coded_width: Option<u32>,

    /// Coded height
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coded_height: Option<u32>,

    /// Has B frames
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_b_frames: Option<u32>,

    /// Sample aspect ratio
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample_aspect_ratio: Option<String>,

    /// Display aspect ratio
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_aspect_ratio: Option<String>,

    /// Pixel format
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pix_fmt: Option<String>,

    /// Level
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<i32>,

    /// Color range
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color_range: Option<String>,

    /// Color space
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color_space: Option<String>,

    /// Color transfer
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color_transfer: Option<String>,

    /// Color primaries
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color_primaries: Option<String>,

    /// Chroma location
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chroma_location: Option<String>,

    /// Field order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_order: Option<String>,

    /// References
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refs: Option<u32>,

    /// Is AVC
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_avc: Option<String>,

    /// NAL length size
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nal_length_size: Option<String>,

    // Audio specific
    /// Sample format
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample_fmt: Option<String>,

    /// Sample rate
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample_rate: Option<String>,

    /// Number of channels
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channels: Option<u32>,

    /// Channel layout
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_layout: Option<String>,

    /// Bits per sample
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bits_per_sample: Option<u32>,

    // Common
    /// Stream ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Frame rate ratio
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r_frame_rate: Option<String>,

    /// Average frame rate
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avg_frame_rate: Option<String>,

    /// Time base
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_base: Option<String>,

    /// Start PTS
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_pts: Option<i64>,

    /// Start time
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,

    /// Duration timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_ts: Option<i64>,

    /// Duration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,

    /// Bit rate
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bit_rate: Option<String>,

    /// Max bit rate
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_bit_rate: Option<String>,

    /// Bits per raw sample
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bits_per_raw_sample: Option<u32>,

    /// Number of frames
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nb_frames: Option<String>,

    /// Number of read frames
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nb_read_frames: Option<String>,

    /// Number of read packets
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nb_read_packets: Option<String>,

    /// Disposition flags
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disposition: Option<HashMap<String, u8>>,

    /// Tags/metadata
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub tags: HashMap<String, String>,
}

impl StreamInfo {
    /// Check if this is a video stream
    pub fn is_video(&self) -> bool {
        self.codec_type.as_deref() == Some("video")
    }

    /// Check if this is an audio stream
    pub fn is_audio(&self) -> bool {
        self.codec_type.as_deref() == Some("audio")
    }

    /// Check if this is a subtitle stream
    pub fn is_subtitle(&self) -> bool {
        self.codec_type.as_deref() == Some("subtitle")
    }

    /// Get the language tag
    pub fn language(&self) -> Option<&str> {
        self.tags.get("language").map(|s| s.as_str())
    }

    /// Get the title tag
    pub fn title(&self) -> Option<&str> {
        self.tags.get("title").map(|s| s.as_str())
    }

    /// Get resolution as (width, height)
    pub fn resolution(&self) -> Option<(u32, u32)> {
        match (self.width, self.height) {
            (Some(w), Some(h)) => Some((w, h)),
            _ => None,
        }
    }

    /// Get frame rate as f64
    pub fn frame_rate(&self) -> Option<f64> {
        self.avg_frame_rate
            .as_ref()
            .or(self.r_frame_rate.as_ref())
            .and_then(|r| parse_rational(r))
    }

    /// Get sample rate as u32
    pub fn sample_rate_hz(&self) -> Option<u32> {
        self.sample_rate.as_ref()?.parse().ok()
    }

    /// Get duration as f64 seconds
    pub fn duration_seconds(&self) -> Option<f64> {
        self.duration.as_ref()?.parse().ok()
    }

    /// Get bit rate as u64
    pub fn bit_rate_bps(&self) -> Option<u64> {
        self.bit_rate.as_ref()?.parse().ok()
    }
}

/// Packet information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PacketInfo {
    /// Codec type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub codec_type: Option<String>,

    /// Stream index
    pub stream_index: u32,

    /// Presentation timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pts: Option<i64>,

    /// Presentation time
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pts_time: Option<String>,

    /// Decoding timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dts: Option<i64>,

    /// Decoding time
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dts_time: Option<String>,

    /// Duration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,

    /// Duration time
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_time: Option<String>,

    /// Size in bytes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,

    /// Position
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pos: Option<String>,

    /// Flags
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<String>,

    /// Data (if show_data enabled)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,

    /// Data hash (if show_data_hash enabled)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_hash: Option<String>,
}

/// Frame information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrameInfo {
    /// Media type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_type: Option<String>,

    /// Stream index
    pub stream_index: u32,

    /// Key frame flag
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_frame: Option<u8>,

    /// Presentation timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pts: Option<i64>,

    /// Presentation time
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pts_time: Option<String>,

    /// Packet timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pkt_pts: Option<i64>,

    /// Packet time
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pkt_pts_time: Option<String>,

    /// Packet DTS
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pkt_dts: Option<i64>,

    /// Packet DTS time
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pkt_dts_time: Option<String>,

    /// Best effort timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    pub best_effort_timestamp: Option<i64>,

    /// Best effort time
    #[serde(skip_serializing_if = "Option::is_none")]
    pub best_effort_timestamp_time: Option<String>,

    /// Packet duration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pkt_duration: Option<i64>,

    /// Packet duration time
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pkt_duration_time: Option<String>,

    /// Packet position
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pkt_pos: Option<String>,

    /// Packet size
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pkt_size: Option<String>,

    // Video specific
    /// Width
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<u32>,

    /// Height
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<u32>,

    /// Pixel format
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pix_fmt: Option<String>,

    /// Picture type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pict_type: Option<String>,

    /// Coded picture number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coded_picture_number: Option<u32>,

    /// Display picture number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_picture_number: Option<u32>,

    /// Interlaced frame
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interlaced_frame: Option<u8>,

    /// Top field first
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_field_first: Option<u8>,

    /// Repeat picture
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repeat_pict: Option<u8>,

    // Audio specific
    /// Sample format
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample_fmt: Option<String>,

    /// Number of samples
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nb_samples: Option<u32>,

    /// Channels
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channels: Option<u32>,

    /// Channel layout
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_layout: Option<String>,
}

/// Program information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgramInfo {
    /// Program ID
    pub program_id: u32,

    /// Program number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program_num: Option<u32>,

    /// Number of streams
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nb_streams: Option<u32>,

    /// PMT PID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmt_pid: Option<u32>,

    /// PCR PID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pcr_pid: Option<u32>,

    /// Start PTS
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_pts: Option<i64>,

    /// Start time
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,

    /// End PTS
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_pts: Option<i64>,

    /// End time
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,

    /// Tags
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub tags: HashMap<String, String>,

    /// Streams in this program
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub streams: Vec<StreamInfo>,
}

/// Chapter information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChapterInfo {
    /// Chapter ID
    pub id: i64,

    /// Time base
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_base: Option<String>,

    /// Start time
    pub start: i64,

    /// Start time string
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,

    /// End time
    pub end: i64,

    /// End time string
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,

    /// Tags
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub tags: HashMap<String, String>,
}

impl ChapterInfo {
    /// Get chapter title
    pub fn title(&self) -> Option<&str> {
        self.tags.get("title").map(|s| s.as_str())
    }
}

/// Error information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorInfo {
    /// Error code
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<i32>,

    /// Error string
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string: Option<String>,
}

/// Parse a rational number string (e.g., "30/1" or "30000/1001")
fn parse_rational(s: &str) -> Option<f64> {
    let parts: Vec<&str> = s.split('/').collect();
    if parts.len() == 2 {
        let num: f64 = parts[0].parse().ok()?;
        let den: f64 = parts[1].parse().ok()?;
        if den != 0.0 {
            Some(num / den)
        } else {
            None
        }
    } else {
        s.parse().ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_interval() {
        let interval = ReadInterval::all();
        assert_eq!(interval.to_string(), "%");

        let interval = ReadInterval::to(IntervalPosition::Absolute(Duration::from_secs(30)));
        assert_eq!(interval.to_string(), "%00:00:30");

        let interval = ReadInterval::from(IntervalPosition::Relative(Duration::from_secs(10)));
        assert_eq!(interval.to_string(), "+00:00:10%");

        let interval = ReadInterval::new(
            Some(IntervalPosition::Absolute(Duration::from_secs(10))),
            Some(IntervalPosition::Packets(100)),
        );
        assert_eq!(interval.to_string(), "00:00:10%#100");
    }

    #[test]
    fn test_stream_info_helpers() {
        let mut stream = StreamInfo {
            index: 0,
            codec_type: Some("video".to_string()),
            width: Some(1920),
            height: Some(1080),
            avg_frame_rate: Some("30/1".to_string()),
            bit_rate: Some("5000000".to_string()),
            tags: HashMap::new(),
            ..Default::default()
        };

        assert!(stream.is_video());
        assert!(!stream.is_audio());
        assert_eq!(stream.resolution(), Some((1920, 1080)));
        assert_eq!(stream.frame_rate(), Some(30.0));
        assert_eq!(stream.bit_rate_bps(), Some(5000000));

        stream.tags.insert("language".to_string(), "eng".to_string());
        assert_eq!(stream.language(), Some("eng"));
    }

    #[test]
    fn test_parse_rational() {
        assert_eq!(parse_rational("30/1"), Some(30.0));
        assert_eq!(parse_rational("30000/1001"), Some(29.97002997002997));
        assert_eq!(parse_rational("25"), Some(25.0));
        assert_eq!(parse_rational("0/1"), Some(0.0));
        assert_eq!(parse_rational("1/0"), None);
    }
}

impl Default for StreamInfo {
    fn default() -> Self {
        Self {
            index: 0,
            codec_name: None,
            codec_long_name: None,
            profile: None,
            codec_type: None,
            codec_tag_string: None,
            codec_tag: None,
            width: None,
            height: None,
            coded_width: None,
            coded_height: None,
            has_b_frames: None,
            sample_aspect_ratio: None,
            display_aspect_ratio: None,
            pix_fmt: None,
            level: None,
            color_range: None,
            color_space: None,
            color_transfer: None,
            color_primaries: None,
            chroma_location: None,
            field_order: None,
            refs: None,
            is_avc: None,
            nal_length_size: None,
            sample_fmt: None,
            sample_rate: None,
            channels: None,
            channel_layout: None,
            bits_per_sample: None,
            id: None,
            r_frame_rate: None,
            avg_frame_rate: None,
            time_base: None,
            start_pts: None,
            start_time: None,
            duration_ts: None,
            duration: None,
            bit_rate: None,
            max_bit_rate: None,
            bits_per_raw_sample: None,
            nb_frames: None,
            nb_read_frames: None,
            nb_read_packets: None,
            disposition: None,
            tags: HashMap::new(),
        }
    }
}