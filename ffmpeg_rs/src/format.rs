use ffmpeg_common::{CommandBuilder, Result};
use std::collections::HashMap;

/// Format options for input/output
#[derive(Debug, Clone, Default)]
pub struct FormatOptions {
    /// Format name
    format: Option<String>,
    /// Format-specific options
    options: HashMap<String, String>,
    /// Muxer flags
    flags: Vec<String>,
}

impl FormatOptions {
    /// Create new format options
    pub fn new() -> Self {
        Self::default()
    }

    /// Set format
    pub fn format(mut self, format: impl Into<String>) -> Self {
        self.format = Some(format.into());
        self
    }

    /// Add a format option
    pub fn option(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.options.insert(key.into(), value.into());
        self
    }

    /// Add a muxer flag
    pub fn flag(mut self, flag: impl Into<String>) -> Self {
        self.flags.push(flag.into());
        self
    }

    /// Build command line arguments
    pub fn build_args(&self) -> Vec<String> {
        let mut cmd = CommandBuilder::new();

        if let Some(ref format) = self.format {
            cmd = cmd.option("-f", format);
        }

        for (key, value) in &self.options {
            cmd = cmd.option(format!("-{}", key), value);
        }

        if !self.flags.is_empty() {
            cmd = cmd.option("-flags", self.flags.join("+"));
        }

        cmd.build()
    }
}

/// Container format configurations
pub mod formats {
    use super::*;

    /// MP4 format options
    pub struct Mp4;

    impl Mp4 {
        /// Standard MP4
        pub fn standard() -> FormatOptions {
            FormatOptions::new()
                .format("mp4")
                .option("movflags", "+faststart")
        }

        /// Fragmented MP4 for streaming
        pub fn fragmented() -> FormatOptions {
            FormatOptions::new()
                .format("mp4")
                .option("movflags", "frag_keyframe+empty_moov+default_base_moof")
                .option("frag_duration", "1000000")
        }

        /// DASH-compatible MP4
        pub fn dash() -> FormatOptions {
            FormatOptions::new()
                .format("mp4")
                .option("movflags", "dash+delay_moov")
                .option("use_timeline", "1")
                .option("use_template", "1")
        }

        /// MP4 for progressive download
        pub fn progressive() -> FormatOptions {
            FormatOptions::new()
                .format("mp4")
                .option("movflags", "+faststart+separate_moof+disable_chpl")
                .option("brand", "mp42")
        }
    }

    /// MKV format options
    pub struct Mkv;

    impl Mkv {
        /// Standard MKV
        pub fn standard() -> FormatOptions {
            FormatOptions::new()
                .format("matroska")
        }

        /// Live streaming MKV
        pub fn streaming() -> FormatOptions {
            FormatOptions::new()
                .format("matroska")
                .option("live", "1")
                .option("cluster_time_limit", "2000")
        }
    }

    /// WebM format options
    pub struct WebM;

    impl WebM {
        /// Standard WebM
        pub fn standard() -> FormatOptions {
            FormatOptions::new()
                .format("webm")
        }

        /// DASH WebM
        pub fn dash() -> FormatOptions {
            FormatOptions::new()
                .format("webm")
                .option("dash", "1")
                .option("cluster_time_limit", "5000")
                .option("cluster_size_limit", "5M")
        }

        /// Live WebM
        pub fn live() -> FormatOptions {
            FormatOptions::new()
                .format("webm")
                .option("live", "1")
                .option("chunk_start_index", "1")
        }
    }

    /// HLS format options
    pub struct Hls;

    impl Hls {
        /// Standard HLS
        pub fn standard() -> FormatOptions {
            FormatOptions::new()
                .format("hls")
                .option("hls_time", "10")
                .option("hls_list_size", "0")
                .option("hls_segment_type", "mpegts")
        }

        /// Live HLS
        pub fn live() -> FormatOptions {
            FormatOptions::new()
                .format("hls")
                .option("hls_time", "2")
                .option("hls_list_size", "5")
                .option("hls_flags", "delete_segments+append_list")
                .option("hls_segment_type", "mpegts")
        }

        /// HLS with fMP4 segments
        pub fn fmp4() -> FormatOptions {
            FormatOptions::new()
                .format("hls")
                .option("hls_segment_type", "fmp4")
                .option("hls_fmp4_init_filename", "init.mp4")
                .option("hls_time", "10")
        }

        /// Event HLS
        pub fn event() -> FormatOptions {
            FormatOptions::new()
                .format("hls")
                .option("hls_playlist_type", "event")
                .option("hls_time", "10")
                .option("hls_list_size", "0")
        }
    }

    /// DASH format options
    pub struct Dash;

    impl Dash {
        /// Standard DASH
        pub fn standard() -> FormatOptions {
            FormatOptions::new()
                .format("dash")
                .option("seg_duration", "4")
                .option("use_timeline", "1")
                .option("use_template", "1")
        }

        /// Live DASH
        pub fn live() -> FormatOptions {
            FormatOptions::new()
                .format("dash")
                .option("seg_duration", "2")
                .option("use_timeline", "1")
                .option("use_template", "1")
                .option("streaming", "1")
                .option("window_size", "5")
                .option("extra_window_size", "2")
        }

        /// Low latency DASH
        pub fn low_latency() -> FormatOptions {
            FormatOptions::new()
                .format("dash")
                .option("seg_duration", "1")
                .option("ldash", "1")
                .option("streaming", "1")
                .option("use_timeline", "0")
                .option("frag_type", "duration")
                .option("frag_duration", "1")
        }
    }

    /// RTMP format options
    pub struct Rtmp;

    impl Rtmp {
        /// RTMP output
        pub fn output() -> FormatOptions {
            FormatOptions::new()
                .format("flv")
                .option("flvflags", "no_duration_filesize")
        }

        /// RTMP with low latency
        pub fn low_latency() -> FormatOptions {
            FormatOptions::new()
                .format("flv")
                .option("flvflags", "no_duration_filesize+aac_seq_header_detect")
                .option("rtmp_buffer", "100")
                .option("rtmp_live", "live")
        }
    }

    /// Image sequence format options
    pub struct ImageSequence;

    impl ImageSequence {
        /// JPEG sequence
        pub fn jpeg() -> FormatOptions {
            FormatOptions::new()
                .format("image2")
                .option("update", "1")
        }

        /// PNG sequence
        pub fn png() -> FormatOptions {
            FormatOptions::new()
                .format("image2")
                .option("update", "1")
        }

        /// Animated GIF
        pub fn gif() -> FormatOptions {
            FormatOptions::new()
                .format("gif")
                .option("loop", "0")
        }
    }

    /// Audio format options
    pub struct Audio;

    impl Audio {
        /// MP3
        pub fn mp3() -> FormatOptions {
            FormatOptions::new()
                .format("mp3")
                .option("id3v2_version", "3")
        }

        /// AAC in ADTS
        pub fn aac() -> FormatOptions {
            FormatOptions::new()
                .format("adts")
        }

        /// FLAC
        pub fn flac() -> FormatOptions {
            FormatOptions::new()
                .format("flac")
        }

        /// OGG
        pub fn ogg() -> FormatOptions {
            FormatOptions::new()
                .format("ogg")
                .option("page_duration", "1000000")
        }

        /// WAV
        pub fn wav() -> FormatOptions {
            FormatOptions::new()
                .format("wav")
                .option("rf64", "auto")
        }
    }

    /// Raw format options
    pub struct Raw;

    impl Raw {
        /// Raw video
        pub fn video() -> FormatOptions {
            FormatOptions::new()
                .format("rawvideo")
        }

        /// Raw audio PCM
        pub fn audio_pcm() -> FormatOptions {
            FormatOptions::new()
                .format("s16le")
        }

        /// Raw H.264
        pub fn h264() -> FormatOptions {
            FormatOptions::new()
                .format("h264")
        }

        /// Raw H.265
        pub fn h265() -> FormatOptions {
            FormatOptions::new()
                .format("hevc")
        }
    }

    /// Null format (for testing)
    pub struct Null;

    impl Null {
        /// Null output
        pub fn output() -> FormatOptions {
            FormatOptions::new()
                .format("null")
        }
    }
}

/// Muxer-specific options
pub struct MuxerOptions {
    options: HashMap<String, String>,
}

impl MuxerOptions {
    /// Create new muxer options
    pub fn new() -> Self {
        Self {
            options: HashMap::new(),
        }
    }

    /// Set option
    pub fn option(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.options.insert(key.into(), value.into());
        self
    }

    /// Build into FormatOptions
    pub fn build(self) -> FormatOptions {
        let mut format_opts = FormatOptions::new();
        for (key, value) in self.options {
            format_opts = format_opts.option(key, value);
        }
        format_opts
    }
}

impl Default for MuxerOptions {
    fn default() -> Self {
        Self::new()
    }
}

/// Common muxer configurations
pub mod muxer_configs {
    use super::*;

    /// Configure for fast seeking
    pub fn fast_seeking() -> MuxerOptions {
        MuxerOptions::new()
            .option("movflags", "+faststart")
            .option("keyint_min", "1")
            .option("g", "30")
    }

    /// Configure for low latency
    pub fn low_latency() -> MuxerOptions {
        MuxerOptions::new()
            .option("flush_packets", "1")
            .option("flags", "+low_delay")
            .option("fflags", "+nobuffer+flush_packets")
    }

    /// Configure for archival
    pub fn archival() -> MuxerOptions {
        MuxerOptions::new()
            .option("write_id3v2", "1")
            .option("write_apetag", "0")
            .option("metadata_header_padding", "1024")
    }

    /// Configure for web compatibility
    pub fn web_compatible() -> MuxerOptions {
        MuxerOptions::new()
            .option("brand", "mp42")
            .option("movflags", "+faststart+separate_moof")
            .option("min_frag_duration", "1000000")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::formats::*;

    #[test]
    fn test_format_options() {
        let opts = FormatOptions::new()
            .format("mp4")
            .option("movflags", "+faststart")
            .flag("low_delay");

        let args = opts.build_args();
        assert!(args.contains(&"-f".to_string()));
        assert!(args.contains(&"mp4".to_string()));
        assert!(args.contains(&"-movflags".to_string()));
        assert!(args.contains(&"+faststart".to_string()));
    }

    #[test]
    fn test_mp4_formats() {
        let standard = Mp4::standard();
        let args = standard.build_args();
        assert!(args.contains(&"mp4".to_string()));

        let fragmented = Mp4::fragmented();
        let args = fragmented.build_args();
        assert!(args.iter().any(|arg| arg.contains("frag_keyframe")));
    }

    #[test]
    fn test_hls_formats() {
        let standard = Hls::standard();
        let args = standard.build_args();
        assert!(args.contains(&"hls".to_string()));
        assert!(args.contains(&"-hls_time".to_string()));

        let live = Hls::live();
        let args = live.build_args();
        assert!(args.contains(&"2".to_string()));
    }

    #[test]
    fn test_audio_formats() {
        let mp3 = Audio::mp3();
        let args = mp3.build_args();
        assert!(args.contains(&"mp3".to_string()));

        let flac = Audio::flac();
        let args = flac.build_args();
        assert!(args.contains(&"flac".to_string()));
    }
}