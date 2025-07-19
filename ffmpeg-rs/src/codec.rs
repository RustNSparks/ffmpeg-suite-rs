use ffmpeg_common::{Codec, CommandBuilder, PixelFormat, Result, SampleFormat};
use std::collections::HashMap;

/// Codec configuration options
#[derive(Debug, Clone)]
pub struct CodecOptions {
    /// Codec to use
    codec: Codec,
    /// Bitrate
    bitrate: Option<String>,
    /// Quality scale (CRF for x264/x265, q for others)
    quality: Option<u8>,
    /// Pixel format (video)
    pixel_format: Option<PixelFormat>,
    /// Sample format (audio)
    sample_format: Option<SampleFormat>,
    /// Frame rate (video)
    framerate: Option<f64>,
    /// Size (video)
    size: Option<(u32, u32)>,
    /// Sample rate (audio)
    sample_rate: Option<u32>,
    /// Channels (audio)
    channels: Option<u32>,
    /// Channel layout (audio)
    channel_layout: Option<String>,
    /// Profile
    profile: Option<String>,
    /// Level
    level: Option<String>,
    /// GOP size
    gop_size: Option<u32>,
    /// B-frames
    b_frames: Option<u32>,
    /// Reference frames
    ref_frames: Option<u32>,
    /// Custom options
    options: HashMap<String, String>,
}

impl CodecOptions {
    /// Create new codec options
    pub fn new(codec: Codec) -> Self {
        Self {
            codec,
            bitrate: None,
            quality: None,
            pixel_format: None,
            sample_format: None,
            framerate: None,
            size: None,
            sample_rate: None,
            channels: None,
            channel_layout: None,
            profile: None,
            level: None,
            gop_size: None,
            b_frames: None,
            ref_frames: None,
            options: HashMap::new(),
        }
    }

    /// Set bitrate
    pub fn bitrate(mut self, bitrate: impl Into<String>) -> Self {
        self.bitrate = Some(bitrate.into());
        self
    }

    /// Set quality (CRF/QP)
    pub fn quality(mut self, quality: u8) -> Self {
        self.quality = Some(quality);
        self
    }

    /// Set pixel format
    pub fn pixel_format(mut self, format: PixelFormat) -> Self {
        self.pixel_format = Some(format);
        self
    }

    /// Set sample format
    pub fn sample_format(mut self, format: SampleFormat) -> Self {
        self.sample_format = Some(format);
        self
    }

    /// Set frame rate
    pub fn framerate(mut self, fps: f64) -> Self {
        self.framerate = Some(fps);
        self
    }

    /// Set video size
    pub fn size(mut self, width: u32, height: u32) -> Self {
        self.size = Some((width, height));
        self
    }

    /// Set sample rate
    pub fn sample_rate(mut self, rate: u32) -> Self {
        self.sample_rate = Some(rate);
        self
    }

    /// Set number of channels
    pub fn channels(mut self, channels: u32) -> Self {
        self.channels = Some(channels);
        self
    }

    /// Set channel layout
    pub fn channel_layout(mut self, layout: impl Into<String>) -> Self {
        self.channel_layout = Some(layout.into());
        self
    }

    /// Set profile
    pub fn profile(mut self, profile: impl Into<String>) -> Self {
        self.profile = Some(profile.into());
        self
    }

    /// Set level
    pub fn level(mut self, level: impl Into<String>) -> Self {
        self.level = Some(level.into());
        self
    }

    /// Set GOP size (keyframe interval)
    pub fn gop_size(mut self, size: u32) -> Self {
        self.gop_size = Some(size);
        self
    }

    /// Set number of B-frames
    pub fn b_frames(mut self, count: u32) -> Self {
        self.b_frames = Some(count);
        self
    }

    /// Set number of reference frames
    pub fn ref_frames(mut self, count: u32) -> Self {
        self.ref_frames = Some(count);
        self
    }

    /// Add custom codec option
    pub fn option(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.options.insert(key.into(), value.into());
        self
    }

    /// Build command line arguments
    pub fn build_args(&self, stream_type: &str) -> Vec<String> {
        let mut cmd = CommandBuilder::new();

        // Codec
        cmd = cmd.option(format!("-c:{}", stream_type), self.codec.as_str());

        // Skip other options for copy codec
        if self.codec.as_str() == "copy" {
            return cmd.build();
        }

        // Bitrate
        if let Some(ref bitrate) = self.bitrate {
            cmd = cmd.option(format!("-b:{}", stream_type), bitrate);
        }

        // Quality
        if let Some(quality) = self.quality {
            match self.codec.as_str() {
                "libx264" | "libx265" | "libvpx" | "libvpx-vp9" => {
                    cmd = cmd.option("-crf", quality);
                }
                _ => {
                    cmd = cmd.option(format!("-q:{}", stream_type), quality);
                }
            }
        }

        // Video options
        if stream_type == "v" {
            if let Some(ref pix_fmt) = self.pixel_format {
                cmd = cmd.option("-pix_fmt", pix_fmt.as_str());
            }

            if let Some(fps) = self.framerate {
                cmd = cmd.option("-r", fps);
            }

            if let Some((width, height)) = self.size {
                cmd = cmd.option("-s", format!("{}x{}", width, height));
            }

            if let Some(gop) = self.gop_size {
                cmd = cmd.option("-g", gop);
            }

            if let Some(bf) = self.b_frames {
                cmd = cmd.option("-bf", bf);
            }

            if let Some(refs) = self.ref_frames {
                cmd = cmd.option("-refs", refs);
            }
        }

        // Audio options
        if stream_type == "a" {
            if let Some(ref sample_fmt) = self.sample_format {
                cmd = cmd.option("-sample_fmt", sample_fmt.as_str());
            }

            if let Some(rate) = self.sample_rate {
                cmd = cmd.option("-ar", rate);
            }

            if let Some(channels) = self.channels {
                cmd = cmd.option("-ac", channels);
            }

            if let Some(ref layout) = self.channel_layout {
                cmd = cmd.option("-channel_layout", layout);
            }
        }

        // Profile and level
        if let Some(ref profile) = self.profile {
            cmd = cmd.option("-profile:v", profile);
        }

        if let Some(ref level) = self.level {
            cmd = cmd.option("-level", level);
        }

        // Custom options
        for (key, value) in &self.options {
            cmd = cmd.option(format!("-{}", key), value);
        }

        cmd.build()
    }
}

/// Preset codec configurations
pub mod presets {
    use super::*;

    /// H.264 codec presets
    pub mod h264 {
        use super::*;

        /// YouTube recommended settings
        pub fn youtube_1080p() -> CodecOptions {
            CodecOptions::new(Codec::h264())
                .bitrate("8000k")
                .option("maxrate", "10000k")
                .option("bufsize", "10000k")
                .profile("high")
                .level("4.0")
                .pixel_format(PixelFormat::yuv420p())
                .gop_size(30)
                .b_frames(2)
        }

        /// High quality archival
        pub fn archival() -> CodecOptions {
            CodecOptions::new(Codec::h264())
                .quality(18)
                .profile("high")
                .level("4.1")
                .pixel_format(PixelFormat::yuv420p())
                .option("preset", "slow")
        }

        /// Fast encoding for live streaming
        pub fn streaming() -> CodecOptions {
            CodecOptions::new(Codec::h264())
                .bitrate("2500k")
                .option("maxrate", "2500k")
                .option("bufsize", "5000k")
                .profile("main")
                .pixel_format(PixelFormat::yuv420p())
                .option("preset", "veryfast")
                .option("tune", "zerolatency")
                .gop_size(60)
                .b_frames(0)
        }

        /// Web-compatible baseline profile
        pub fn web_compatible() -> CodecOptions {
            CodecOptions::new(Codec::h264())
                .quality(23)
                .profile("baseline")
                .level("3.0")
                .pixel_format(PixelFormat::yuv420p())
                .option("preset", "medium")
                .option("movflags", "+faststart")
        }
    }

    /// H.265/HEVC codec presets
    pub mod h265 {
        use super::*;

        /// High quality 4K
        pub fn uhd_4k() -> CodecOptions {
            CodecOptions::new(Codec::h265())
                .quality(20)
                .profile("main")
                .level("5.1")
                .pixel_format(PixelFormat::yuv420p())
                .option("preset", "slow")
        }

        /// Efficient 1080p
        pub fn efficient_1080p() -> CodecOptions {
            CodecOptions::new(Codec::h265())
                .quality(24)
                .profile("main")
                .level("4.0")
                .pixel_format(PixelFormat::yuv420p())
                .option("preset", "medium")
        }
    }

    /// VP9 codec presets
    pub mod vp9 {
        use super::*;

        /// YouTube VP9 encoding
        pub fn youtube() -> CodecOptions {
            CodecOptions::new(Codec::vp9())
                .bitrate("0")  // VBR mode
                .quality(31)
                .option("deadline", "good")
                .option("cpu-used", "2")
                .option("tile-columns", "2")
                .option("tile-rows", "2")
                .option("threads", "8")
        }

        /// High quality WebM
        pub fn high_quality_webm() -> CodecOptions {
            CodecOptions::new(Codec::vp9())
                .quality(20)
                .option("deadline", "best")
                .option("cpu-used", "0")
                .pixel_format(PixelFormat::yuv420p())
        }
    }

    /// AV1 codec presets
    pub mod av1 {
        use super::*;

        /// SVT-AV1 efficient encoding
        pub fn svt_efficient() -> CodecOptions {
            CodecOptions::new(Codec::new("libsvtav1"))
                .quality(35)
                .option("preset", "8")
                .pixel_format(PixelFormat::yuv420p())
        }

        /// libaom-av1 high quality
        pub fn aom_high_quality() -> CodecOptions {
            CodecOptions::new(Codec::new("libaom-av1"))
                .quality(30)
                .option("cpu-used", "4")
                .option("tile-columns", "2")
                .option("tile-rows", "2")
                .pixel_format(PixelFormat::yuv420p())
        }
    }

    /// Audio codec presets
    pub mod audio {
        use super::*;

        /// AAC for general use
        pub fn aac_standard() -> CodecOptions {
            CodecOptions::new(Codec::aac())
                .bitrate("128k")
                .sample_rate(44100)
                .channels(2)
        }

        /// High quality AAC
        pub fn aac_high_quality() -> CodecOptions {
            CodecOptions::new(Codec::aac())
                .bitrate("256k")
                .sample_rate(48000)
                .channels(2)
                .profile("aac_low")
        }

        /// Opus for streaming
        pub fn opus_streaming() -> CodecOptions {
            CodecOptions::new(Codec::opus())
                .bitrate("96k")
                .sample_rate(48000)
                .option("frame_duration", "20")
                .option("application", "audio")
        }

        /// FLAC lossless
        pub fn flac_lossless() -> CodecOptions {
            CodecOptions::new(Codec::flac())
                .option("compression_level", "8")
        }

        /// MP3 compatible
        pub fn mp3_compatible() -> CodecOptions {
            CodecOptions::new(Codec::mp3())
                .bitrate("192k")
                .sample_rate(44100)
                .channels(2)
                .option("q:a", "2")
        }
    }
}

/// Hardware acceleration codec options
pub mod hardware {
    use super::*;

    /// NVIDIA NVENC H.264
    pub fn nvenc_h264() -> CodecOptions {
        CodecOptions::new(Codec::new("h264_nvenc"))
            .option("preset", "p4")
            .option("tune", "hq")
            .option("rc", "vbr")
            .option("cq", "23")
            .option("b:v", "0")
            .profile("high")
            .pixel_format(PixelFormat::yuv420p())
    }

    /// NVIDIA NVENC H.265
    pub fn nvenc_h265() -> CodecOptions {
        CodecOptions::new(Codec::new("hevc_nvenc"))
            .option("preset", "p4")
            .option("tune", "hq")
            .option("rc", "vbr")
            .option("cq", "25")
            .profile("main")
            .pixel_format(PixelFormat::yuv420p())
    }

    /// Intel Quick Sync H.264
    pub fn qsv_h264() -> CodecOptions {
        CodecOptions::new(Codec::new("h264_qsv"))
            .option("preset", "medium")
            .option("global_quality", "23")
            .profile("high")
            .pixel_format(PixelFormat::nv12())
    }

    /// AMD AMF H.264
    pub fn amf_h264() -> CodecOptions {
        CodecOptions::new(Codec::new("h264_amf"))
            .option("usage", "transcoding")
            .option("quality", "balanced")
            .option("rc", "vbr_peak")
            .bitrate("5000k")
            .option("maxrate", "6000k")
    }

    /// Apple VideoToolbox H.264
    pub fn videotoolbox_h264() -> CodecOptions {
        CodecOptions::new(Codec::new("h264_videotoolbox"))
            .option("realtime", "1")
            .profile("high")
            .bitrate("5000k")
    }

    /// VAAPI H.264 (Linux)
    pub fn vaapi_h264() -> CodecOptions {
        CodecOptions::new(Codec::new("h264_vaapi"))
            .option("rc_mode", "VBR")
            .bitrate("5000k")
            .option("maxrate", "6000k")
            .profile("high")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::presets::*;

    #[test]
    fn test_codec_options() {
        let options = CodecOptions::new(Codec::h264())
            .bitrate("5000k")
            .quality(23)
            .profile("high")
            .size(1920, 1080)
            .framerate(30.0);

        let args = options.build_args("v");
        assert!(args.contains(&"-c:v".to_string()));
        assert!(args.contains(&"h264".to_string()));
        assert!(args.contains(&"-b:v".to_string()));
        assert!(args.contains(&"5000k".to_string()));
        assert!(args.contains(&"-crf".to_string()));
        assert!(args.contains(&"23".to_string()));
    }

    #[test]
    fn test_presets() {
        let youtube = h264::youtube_1080p();
        let args = youtube.build_args("v");
        assert!(args.contains(&"-profile:v".to_string()));
        assert!(args.contains(&"high".to_string()));

        let aac = audio::aac_standard();
        let args = aac.build_args("a");
        assert!(args.contains(&"-c:a".to_string()));
        assert!(args.contains(&"aac".to_string()));
        assert!(args.contains(&"-b:a".to_string()));
        assert!(args.contains(&"128k".to_string()));
    }

    #[test]
    fn test_hardware_codecs() {
        let nvenc = hardware::nvenc_h264();
        let args = nvenc.build_args("v");
        assert!(args.contains(&"h264_nvenc".to_string()));
        assert!(args.contains(&"-preset".to_string()));
        assert!(args.contains(&"p4".to_string()));
    }
}