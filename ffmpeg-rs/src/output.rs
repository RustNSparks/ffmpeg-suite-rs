use ffmpeg_common::{
    Codec, CommandBuilder, Duration, MediaPath, PixelFormat, Result, SampleFormat, Size,
};
use std::collections::HashMap;
use std::time::Duration as StdDuration;

use crate::codec::CodecOptions;
use crate::format::FormatOptions;

/// Output specification for FFmpeg
#[derive(Debug, Clone)]
pub struct Output {
    /// Destination path or URL
    destination: MediaPath,
    /// Format options
    format_options: FormatOptions,
    /// Video codec options
    video_codec: Option<CodecOptions>,
    /// Audio codec options
    audio_codec: Option<CodecOptions>,
    /// Subtitle codec options
    subtitle_codec: Option<CodecOptions>,
    /// Duration limit
    duration: Option<Duration>,
    /// File size limit
    file_size_limit: Option<Size>,
    /// Number of frames to output
    frames: Option<u64>,
    /// Metadata
    metadata: HashMap<String, String>,
    /// Stream metadata
    stream_metadata: HashMap<String, HashMap<String, String>>,
    /// Movflags for MP4
    movflags: Option<String>,
    /// Preset
    preset: Option<String>,
    /// Tune
    tune: Option<String>,
    /// Custom options
    options: HashMap<String, String>,
    /// Disable video
    no_video: bool,
    /// Disable audio
    no_audio: bool,
    /// Disable subtitles
    no_subtitles: bool,
    /// Copy timestamps
    copy_timestamps: bool,
    /// Avoid negative timestamps
    avoid_negative_ts: Option<String>,
    /// Start time
    start_time: Option<Duration>,
}

impl Output {
    /// Create a new output
    pub fn new(destination: impl Into<MediaPath>) -> Self {
        Self {
            destination: destination.into(),
            format_options: FormatOptions::new(),
            video_codec: None,
            audio_codec: None,
            subtitle_codec: None,
            duration: None,
            file_size_limit: None,
            frames: None,
            metadata: HashMap::new(),
            stream_metadata: HashMap::new(),
            movflags: None,
            preset: None,
            tune: None,
            options: HashMap::new(),
            no_video: false,
            no_audio: false,
            no_subtitles: false,
            copy_timestamps: false,
            avoid_negative_ts: None,
            start_time: None,
        }
    }

    /// Set output format
    pub fn format(mut self, format: impl Into<String>) -> Self {
        self.format_options = self.format_options.format(format);
        self
    }

    /// Set video codec
    pub fn video_codec(mut self, codec: Codec) -> Self {
        self.video_codec = Some(CodecOptions::new(codec));
        self
    }

    /// Set video codec with options
    pub fn video_codec_opts(mut self, options: CodecOptions) -> Self {
        self.video_codec = Some(options);
        self
    }

    /// Set audio codec
    pub fn audio_codec(mut self, codec: Codec) -> Self {
        self.audio_codec = Some(CodecOptions::new(codec));
        self
    }

    /// Set audio codec with options
    pub fn audio_codec_opts(mut self, options: CodecOptions) -> Self {
        self.audio_codec = Some(options);
        self
    }

    /// Set subtitle codec
    pub fn subtitle_codec(mut self, codec: Codec) -> Self {
        self.subtitle_codec = Some(CodecOptions::new(codec));
        self
    }

    /// Copy all codecs
    pub fn copy_codecs(self) -> Self {
        self.video_codec(Codec::copy())
            .audio_codec(Codec::copy())
            .subtitle_codec(Codec::copy())
    }

    /// Set duration limit
    pub fn duration(mut self, duration: Duration) -> Self {
        self.duration = Some(duration);
        self
    }

    /// Set file size limit
    pub fn file_size_limit(mut self, size: Size) -> Self {
        self.file_size_limit = Some(size);
        self
    }

    /// Set number of frames to output
    pub fn frames(mut self, count: u64) -> Self {
        self.frames = Some(count);
        self
    }

    /// Add metadata
    pub fn metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }

    /// Add stream-specific metadata
    pub fn stream_metadata(
        mut self,
        stream_spec: impl Into<String>,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Self {
        let stream_spec = stream_spec.into();
        self.stream_metadata
            .entry(stream_spec)
            .or_default()
            .insert(key.into(), value.into());
        self
    }

    /// Set movflags for MP4
    pub fn movflags(mut self, flags: impl Into<String>) -> Self {
        self.movflags = Some(flags.into());
        self
    }

    /// Enable faststart for MP4 (move moov atom to beginning)
    pub fn faststart(self) -> Self {
        self.movflags("faststart")
    }

    /// Set encoding preset
    pub fn preset(mut self, preset: impl Into<String>) -> Self {
        self.preset = Some(preset.into());
        self
    }

    /// Set encoding tune
    pub fn tune(mut self, tune: impl Into<String>) -> Self {
        self.tune = Some(tune.into());
        self
    }

    /// Add custom option
    pub fn option(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.options.insert(key.into(), value.into());
        self
    }

    /// Disable video output
    pub fn no_video(mut self) -> Self {
        self.no_video = true;
        self
    }

    /// Disable audio output
    pub fn no_audio(mut self) -> Self {
        self.no_audio = true;
        self
    }

    /// Disable subtitle output
    pub fn no_subtitles(mut self) -> Self {
        self.no_subtitles = true;
        self
    }

    /// Copy timestamps
    pub fn copy_timestamps(mut self, enable: bool) -> Self {
        self.copy_timestamps = enable;
        self
    }

    /// Avoid negative timestamps
    pub fn avoid_negative_ts(mut self, mode: impl Into<String>) -> Self {
        self.avoid_negative_ts = Some(mode.into());
        self
    }

    /// Set start time
    pub fn start_time(mut self, time: Duration) -> Self {
        self.start_time = Some(time);
        self
    }

    /// Configure for streaming
    pub fn for_streaming(self) -> Self {
        self.format("mp4")
            .movflags("frag_keyframe+empty_moov+default_base_moof")
            .option("g", "52")  // GOP size
            .option("keyint_min", "25")
    }

    /// Configure for HLS output
    pub fn for_hls(self, segment_duration: u32) -> Self {
        self.format("hls")
            .option("hls_time", segment_duration.to_string())
            .option("hls_playlist_type", "vod")
            .option("hls_segment_filename", "segment_%03d.ts")
    }

    /// Build command line arguments
    pub fn build_args(&self) -> Vec<String> {
        let mut cmd = CommandBuilder::new();

        // Format options
        cmd = cmd.args(self.format_options.build_args());

        // Video codec
        if let Some(ref codec) = self.video_codec {
            cmd = cmd.args(codec.build_args("v"));
        }

        // Audio codec
        if let Some(ref codec) = self.audio_codec {
            cmd = cmd.args(codec.build_args("a"));
        }

        // Subtitle codec
        if let Some(ref codec) = self.subtitle_codec {
            cmd = cmd.args(codec.build_args("s"));
        }

        // Duration and limits
        if let Some(duration) = self.duration {
            cmd = cmd.option("-t", duration.to_ffmpeg_format());
        }

        if let Some(ref size) = self.file_size_limit {
            cmd = cmd.option("-fs", size.as_bytes());
        }

        if let Some(frames) = self.frames {
            cmd = cmd.option("-frames:v", frames);
        }

        // Metadata
        for (key, value) in &self.metadata {
            cmd = cmd.option("-metadata", format!("{}={}", key, value));
        }

        // Stream metadata
        for (stream_spec, metadata) in &self.stream_metadata {
            for (key, value) in metadata {
                cmd = cmd.option(
                    format!("-metadata:s:{}", stream_spec),
                    format!("{}={}", key, value),
                );
            }
        }

        // MP4 specific
        if let Some(ref flags) = self.movflags {
            cmd = cmd.option("-movflags", flags);
        }

        // Preset and tune
        if let Some(ref preset) = self.preset {
            cmd = cmd.option("-preset", preset);
        }

        if let Some(ref tune) = self.tune {
            cmd = cmd.option("-tune", tune);
        }

        // Disable streams
        if self.no_video {
            cmd = cmd.flag("-vn");
        }

        if self.no_audio {
            cmd = cmd.flag("-an");
        }

        if self.no_subtitles {
            cmd = cmd.flag("-sn");
        }

        // Timestamp options
        if self.copy_timestamps {
            cmd = cmd.flag("-copyts");
        }

        if let Some(ref mode) = self.avoid_negative_ts {
            cmd = cmd.option("-avoid_negative_ts", mode);
        }

        if let Some(start) = self.start_time {
            cmd = cmd.option("-ss", start.to_ffmpeg_format());
        }

        // Custom options
        for (key, value) in &self.options {
            cmd = cmd.option(key, value);
        }

        // Output file
        cmd = cmd.arg(self.destination.as_str());

        cmd.build()
    }
}

/// Builder for multi-output scenarios
#[derive(Debug, Clone)]
pub struct MultiOutput {
    outputs: Vec<Output>,
}

impl MultiOutput {
    /// Create a new multi-output builder
    pub fn new() -> Self {
        Self {
            outputs: Vec::new(),
        }
    }

    /// Add an output
    pub fn add_output(mut self, output: Output) -> Self {
        self.outputs.push(output);
        self
    }

    /// Create adaptive streaming outputs (multiple qualities)
    pub fn adaptive_streaming(base_path: impl AsRef<str>) -> Self {
        let base = base_path.as_ref();

        Self::new()
            // 1080p
            .add_output(
                Output::new(format!("{}_1080p.mp4", base))
                    .video_codec_opts(
                        CodecOptions::new(Codec::h264())
                            .bitrate("5000k")
                            .option("maxrate", "5350k")
                            .option("bufsize", "7500k")
                            .size(1920, 1080)
                    )
                    .audio_codec_opts(
                        CodecOptions::new(Codec::aac())
                            .bitrate("192k")
                    )
                    .preset("slow")
            )
            // 720p
            .add_output(
                Output::new(format!("{}_720p.mp4", base))
                    .video_codec_opts(
                        CodecOptions::new(Codec::h264())
                            .bitrate("2800k")
                            .option("maxrate", "3000k")
                            .option("bufsize", "4200k")
                            .size(1280, 720)
                    )
                    .audio_codec_opts(
                        CodecOptions::new(Codec::aac())
                            .bitrate("128k")
                    )
                    .preset("slow")
            )
            // 480p
            .add_output(
                Output::new(format!("{}_480p.mp4", base))
                    .video_codec_opts(
                        CodecOptions::new(Codec::h264())
                            .bitrate("1400k")
                            .option("maxrate", "1500k")
                            .option("bufsize", "2100k")
                            .size(854, 480)
                    )
                    .audio_codec_opts(
                        CodecOptions::new(Codec::aac())
                            .bitrate("128k")
                    )
                    .preset("slow")
            )
    }

    /// Get the outputs
    pub fn into_outputs(self) -> Vec<Output> {
        self.outputs
    }
}

impl Default for MultiOutput {
    fn default() -> Self {
        Self::new()
    }
}

/// Builder for image sequence output
#[derive(Debug, Clone)]
pub struct ImageSequenceOutput {
    /// Base path with pattern
    pattern: String,
    /// Image format
    format: String,
    /// Frame rate
    framerate: Option<f64>,
    /// Quality (for JPEG)
    quality: Option<u8>,
    /// Start number
    start_number: Option<u32>,
}

impl ImageSequenceOutput {
    /// Create a new image sequence output
    pub fn new(pattern: impl Into<String>) -> Self {
        Self {
            pattern: pattern.into(),
            format: "image2".to_string(),
            framerate: None,
            quality: None,
            start_number: None,
        }
    }

    /// Set output format (jpeg, png, etc.)
    pub fn image_format(mut self, format: impl Into<String>) -> Self {
        self.format = format.into();
        self
    }

    /// Set frame rate
    pub fn framerate(mut self, fps: f64) -> Self {
        self.framerate = Some(fps);
        self
    }

    /// Set JPEG quality (2-31, lower is better)
    pub fn quality(mut self, q: u8) -> Self {
        self.quality = Some(q.clamp(2, 31));
        self
    }

    /// Set start number
    pub fn start_number(mut self, num: u32) -> Self {
        self.start_number = Some(num);
        self
    }

    /// Convert to Output
    pub fn into_output(self) -> Output {
        let mut output = Output::new(self.pattern).format("image2");

        if let Some(fps) = self.framerate {
            output = output.option("r", fps.to_string());
        }

        if let Some(q) = self.quality {
            output = output.video_codec_opts(
                CodecOptions::new(Codec::new("mjpeg"))
                    .quality(q)
            );
        }

        if let Some(num) = self.start_number {
            output = output.option("start_number", num.to_string());
        }

        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_output_builder() {
        let output = Output::new("output.mp4")
            .format("mp4")
            .video_codec(Codec::h264())
            .audio_codec(Codec::aac())
            .metadata("title", "My Video")
            .faststart();

        let args = output.build_args();
        assert!(args.contains(&"-f".to_string()));
        assert!(args.contains(&"mp4".to_string()));
        assert!(args.contains(&"-c:v".to_string()));
        assert!(args.contains(&"h264".to_string()));
        assert!(args.contains(&"-c:a".to_string()));
        assert!(args.contains(&"aac".to_string()));
        assert!(args.contains(&"-metadata".to_string()));
        assert!(args.contains(&"title=My Video".to_string()));
        assert!(args.contains(&"-movflags".to_string()));
        assert!(args.contains(&"faststart".to_string()));
    }

    #[test]
    fn test_streaming_output() {
        let output = Output::new("output.mp4").for_streaming();
        let args = output.build_args();

        assert!(args.contains(&"-movflags".to_string()));
        assert!(args.iter().any(|arg| arg.contains("frag_keyframe")));
    }

    #[test]
    fn test_image_sequence() {
        let output = ImageSequenceOutput::new("frame_%04d.jpg")
            .quality(5)
            .framerate(1.0)
            .into_output();

        let args = output.build_args();
        assert!(args.contains(&"-f".to_string()));
        assert!(args.contains(&"image2".to_string()));
        assert!(args.contains(&"-r".to_string()));
        assert!(args.contains(&"1".to_string()));
    }
}