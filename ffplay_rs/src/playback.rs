use ffmpeg_common::{CommandBuilder, Duration, StreamSpecifier};

/// Sync type for audio/video synchronization
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SyncType {
    /// Sync to audio clock (default)
    Audio,
    /// Sync to video clock
    Video,
    /// Sync to external clock
    External,
}

impl SyncType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Audio => "audio",
            Self::Video => "video",
            Self::External => "ext",
        }
    }
}

/// Playback options for FFplay
#[derive(Debug, Clone)]
pub struct PlaybackOptions {
    /// Disable audio
    no_audio: bool,
    /// Disable video
    no_video: bool,
    /// Disable subtitles
    no_subtitles: bool,
    /// Start position
    start_position: Option<Duration>,
    /// Duration to play
    duration: Option<Duration>,
    /// Loop count (-1 for infinite)
    loop_count: Option<i32>,
    /// Volume (0-100)
    volume: Option<u8>,
    /// Format to force
    format: Option<String>,
    /// Seek by bytes
    seek_by_bytes: bool,
    /// Seek interval
    seek_interval: Option<f64>,
    /// Fast mode
    fast: bool,
    /// Generate PTS
    genpts: bool,
    /// Sync type
    sync_type: Option<SyncType>,
    /// Audio stream specifier
    audio_stream: Option<StreamSpecifier>,
    /// Video stream specifier
    video_stream: Option<StreamSpecifier>,
    /// Subtitle stream specifier
    subtitle_stream: Option<StreamSpecifier>,
    /// Auto exit when done
    autoexit: bool,
    /// Exit on key down
    exitonkeydown: bool,
    /// Exit on mouse down
    exitonmousedown: bool,
    /// Video codec
    video_codec: Option<String>,
    /// Audio codec
    audio_codec: Option<String>,
    /// Subtitle codec
    subtitle_codec: Option<String>,
    /// Auto rotate
    autorotate: bool,
    /// Frame drop
    framedrop: bool,
    /// Infinite buffer
    infbuf: bool,
    /// Video filters
    video_filters: Option<String>,
    /// Audio filters
    audio_filters: Option<String>,
    /// Show statistics
    stats: bool,
    /// Filter threads
    filter_threads: Option<u32>,
}

impl Default for PlaybackOptions {
    fn default() -> Self {
        Self {
            no_audio: false,
            no_video: false,
            no_subtitles: false,
            start_position: None,
            duration: None,
            loop_count: None,
            volume: None,
            format: None,
            seek_by_bytes: false,
            seek_interval: None,
            fast: false,
            genpts: false,
            sync_type: None,
            audio_stream: None,
            video_stream: None,
            subtitle_stream: None,
            autoexit: false,
            exitonkeydown: false,
            exitonmousedown: false,
            video_codec: None,
            audio_codec: None,
            subtitle_codec: None,
            autorotate: true,
            framedrop: true,
            infbuf: false,
            video_filters: None,
            audio_filters: None,
            stats: true,
            filter_threads: None,
        }
    }
}

impl PlaybackOptions {
    /// Create new playback options
    pub fn new() -> Self {
        Self::default()
    }

    /// Disable audio
    pub fn no_audio(mut self, enable: bool) -> Self {
        self.no_audio = enable;
        self
    }

    /// Disable video
    pub fn no_video(mut self, enable: bool) -> Self {
        self.no_video = enable;
        self
    }

    /// Disable subtitles
    pub fn no_subtitles(mut self, enable: bool) -> Self {
        self.no_subtitles = enable;
        self
    }

    /// Set start position
    pub fn seek(mut self, position: Duration) -> Self {
        self.start_position = Some(position);
        self
    }

    /// Set duration to play
    pub fn duration(mut self, duration: Duration) -> Self {
        self.duration = Some(duration);
        self
    }

    /// Set loop count (-1 for infinite)
    pub fn loop_count(mut self, count: i32) -> Self {
        self.loop_count = Some(count);
        self
    }

    /// Set volume (0-100)
    pub fn volume(mut self, volume: u8) -> Self {
        self.volume = Some(volume.min(100));
        self
    }

    /// Force format
    pub fn format(mut self, format: impl Into<String>) -> Self {
        self.format = Some(format.into());
        self
    }

    /// Enable seek by bytes
    pub fn seek_by_bytes(mut self, enable: bool) -> Self {
        self.seek_by_bytes = enable;
        self
    }

    /// Set seek interval in seconds
    pub fn seek_interval(mut self, seconds: f64) -> Self {
        self.seek_interval = Some(seconds);
        self
    }

    /// Enable fast mode
    pub fn fast(mut self, enable: bool) -> Self {
        self.fast = enable;
        self
    }

    /// Enable PTS generation
    pub fn genpts(mut self, enable: bool) -> Self {
        self.genpts = enable;
        self
    }

    /// Set sync type
    pub fn sync(mut self, sync_type: SyncType) -> Self {
        self.sync_type = Some(sync_type);
        self
    }

    /// Select audio stream
    pub fn audio_stream(mut self, spec: StreamSpecifier) -> Self {
        self.audio_stream = Some(spec);
        self
    }

    /// Select video stream
    pub fn video_stream(mut self, spec: StreamSpecifier) -> Self {
        self.video_stream = Some(spec);
        self
    }

    /// Select subtitle stream
    pub fn subtitle_stream(mut self, spec: StreamSpecifier) -> Self {
        self.subtitle_stream = Some(spec);
        self
    }

    /// Enable auto exit
    pub fn autoexit(mut self, enable: bool) -> Self {
        self.autoexit = enable;
        self
    }

    /// Exit on key down
    pub fn exitonkeydown(mut self, enable: bool) -> Self {
        self.exitonkeydown = enable;
        self
    }

    /// Exit on mouse down
    pub fn exitonmousedown(mut self, enable: bool) -> Self {
        self.exitonmousedown = enable;
        self
    }

    /// Set video codec
    pub fn video_codec(mut self, codec: impl Into<String>) -> Self {
        self.video_codec = Some(codec.into());
        self
    }

    /// Set audio codec
    pub fn audio_codec(mut self, codec: impl Into<String>) -> Self {
        self.audio_codec = Some(codec.into());
        self
    }

    /// Set subtitle codec
    pub fn subtitle_codec(mut self, codec: impl Into<String>) -> Self {
        self.subtitle_codec = Some(codec.into());
        self
    }

    /// Enable auto rotation
    pub fn autorotate(mut self, enable: bool) -> Self {
        self.autorotate = enable;
        self
    }

    /// Enable frame dropping
    pub fn framedrop(mut self, enable: bool) -> Self {
        self.framedrop = enable;
        self
    }

    /// Enable infinite buffer
    pub fn infbuf(mut self, enable: bool) -> Self {
        self.infbuf = enable;
        self
    }

    /// Set video filter
    pub fn video_filter(mut self, filter: impl Into<String>) -> Self {
        self.video_filters = Some(filter.into());
        self
    }

    /// Set audio filter
    pub fn audio_filter(mut self, filter: impl Into<String>) -> Self {
        self.audio_filters = Some(filter.into());
        self
    }

    /// Show statistics
    pub fn stats(mut self, enable: bool) -> Self {
        self.stats = enable;
        self
    }

    /// Set filter threads
    pub fn filter_threads(mut self, threads: u32) -> Self {
        self.filter_threads = Some(threads);
        self
    }

    /// Build command line arguments
    pub fn build_args(&self) -> Vec<String> {
        let mut cmd = CommandBuilder::new();

        // Media type disabling
        if self.no_audio {
            cmd = cmd.flag("-an");
        }
        if self.no_video {
            cmd = cmd.flag("-vn");
        }
        if self.no_subtitles {
            cmd = cmd.flag("-sn");
        }

        // Timing
        if let Some(ref pos) = self.start_position {
            cmd = cmd.option("-ss", pos.to_ffmpeg_format());
        }
        if let Some(ref dur) = self.duration {
            cmd = cmd.option("-t", dur.to_ffmpeg_format());
        }

        // Loop
        if let Some(count) = self.loop_count {
            cmd = cmd.option("-loop", count);
        }

        // Volume
        if let Some(vol) = self.volume {
            cmd = cmd.option("-volume", vol);
        }

        // Format
        if let Some(ref fmt) = self.format {
            cmd = cmd.option("-f", fmt);
        }

        // Seeking
        if self.seek_by_bytes {
            cmd = cmd.flag("-bytes");
        }
        if let Some(interval) = self.seek_interval {
            cmd = cmd.option("-seek_interval", interval);
        }

        // Performance
        if self.fast {
            cmd = cmd.flag("-fast");
        }
        if self.genpts {
            cmd = cmd.flag("-genpts");
        }

        // Sync
        if let Some(ref sync) = self.sync_type {
            cmd = cmd.option("-sync", sync.as_str());
        }

        // Stream selection
        if let Some(ref spec) = self.audio_stream {
            cmd = cmd.option("-ast", spec.to_string());
        }
        if let Some(ref spec) = self.video_stream {
            cmd = cmd.option("-vst", spec.to_string());
        }
        if let Some(ref spec) = self.subtitle_stream {
            cmd = cmd.option("-sst", spec.to_string());
        }

        // Exit behavior
        if self.autoexit {
            cmd = cmd.flag("-autoexit");
        }
        if self.exitonkeydown {
            cmd = cmd.flag("-exitonkeydown");
        }
        if self.exitonmousedown {
            cmd = cmd.flag("-exitonmousedown");
        }

        // Codecs
        if let Some(ref codec) = self.video_codec {
            cmd = cmd.option("-vcodec", codec);
        }
        if let Some(ref codec) = self.audio_codec {
            cmd = cmd.option("-acodec", codec);
        }
        if let Some(ref codec) = self.subtitle_codec {
            cmd = cmd.option("-scodec", codec);
        }

        // Features
        if !self.autorotate {
            cmd = cmd.flag("-noautorotate");
        }
        if !self.framedrop {
            cmd = cmd.flag("-noframedrop");
        }
        if self.infbuf {
            cmd = cmd.flag("-infbuf");
        }

        // Filters
        if let Some(ref filter) = self.video_filters {
            cmd = cmd.option("-vf", filter);
        }
        if let Some(ref filter) = self.audio_filters {
            cmd = cmd.option("-af", filter);
        }

        // Stats
        if !self.stats {
            cmd = cmd.flag("-nostats");
        }

        // Filter threads
        if let Some(threads) = self.filter_threads {
            cmd = cmd.option("-filter_threads", threads);
        }

        cmd.build()
    }
}

/// Preset playback configurations
pub mod presets {
    use super::*;
    use ffmpeg_common::StreamType;

    /// Default playback settings
    pub fn default() -> PlaybackOptions {
        PlaybackOptions::default()
    }

    /// Audio-only playback
    pub fn audio_only() -> PlaybackOptions {
        PlaybackOptions::new()
            .no_video(true)
            .no_subtitles(true)
    }

    /// Video-only playback (no audio)
    pub fn video_only() -> PlaybackOptions {
        PlaybackOptions::new()
            .no_audio(true)
            .no_subtitles(true)
    }

    /// Low latency streaming
    pub fn low_latency() -> PlaybackOptions {
        PlaybackOptions::new()
            .fast(true)
            .sync(SyncType::External)
            .infbuf(true)
            .framedrop(true)
    }

    /// Preview mode
    pub fn preview() -> PlaybackOptions {
        PlaybackOptions::new()
            .duration(Duration::from_secs(30))
            .autoexit(true)
            .stats(false)
    }

    /// Interactive mode
    pub fn interactive() -> PlaybackOptions {
        PlaybackOptions::new()
            .exitonkeydown(false)
            .exitonmousedown(false)
            .stats(true)
    }

    /// Benchmark mode
    pub fn benchmark() -> PlaybackOptions {
        PlaybackOptions::new()
            .video_codec("rawvideo")
            .audio_codec("pcm_s16le")
            .fast(true)
            .autoexit(true)
    }

    /// Language selection (audio and subtitles)
    pub fn with_language(lang: &str) -> PlaybackOptions {
        PlaybackOptions::new()
            .audio_stream(StreamSpecifier::Metadata {
                key: "language".to_string(),
                value: Some(lang.to_string()),
            })
            .subtitle_stream(StreamSpecifier::Metadata {
                key: "language".to_string(),
                value: Some(lang.to_string()),
            })
    }

    /// Loop forever
    pub fn loop_forever() -> PlaybackOptions {
        PlaybackOptions::new()
            .loop_count(-1)
    }

    /// Muted playback
    pub fn muted() -> PlaybackOptions {
        PlaybackOptions::new()
            .volume(0)
    }

    /// Test pattern mode
    pub fn test_pattern() -> PlaybackOptions {
        PlaybackOptions::new()
            .format("lavfi")
            .video_filter("testsrc2=size=1280x720:rate=30")
            .audio_filter("sine=frequency=1000:sample_rate=48000")
    }
}

#[cfg(test)]
mod tests {
    use ffmpeg_common::StreamType;
    use super::*;

    #[test]
    fn test_playback_options() {
        let opts = PlaybackOptions::new()
            .seek(Duration::from_secs(10))
            .duration(Duration::from_secs(30))
            .volume(75)
            .loop_count(2);

        let args = opts.build_args();
        assert!(args.contains(&"-ss".to_string()));
        assert!(args.contains(&"00:00:10".to_string()));
        assert!(args.contains(&"-t".to_string()));
        assert!(args.contains(&"00:00:30".to_string()));
        assert!(args.contains(&"-volume".to_string()));
        assert!(args.contains(&"75".to_string()));
        assert!(args.contains(&"-loop".to_string()));
        assert!(args.contains(&"2".to_string()));
    }

    #[test]
    fn test_stream_selection() {
        let opts = PlaybackOptions::new()
            .audio_stream(StreamSpecifier::Index(1))
            .video_stream(StreamSpecifier::Type(StreamType::Video))
            .subtitle_stream(StreamSpecifier::Type(StreamType::Subtitle));

        let args = opts.build_args();
        assert!(args.contains(&"-ast".to_string()));
        assert!(args.contains(&"1".to_string()));
        assert!(args.contains(&"-vst".to_string()));
        assert!(args.contains(&"v".to_string()));
        assert!(args.contains(&"-sst".to_string()));
        assert!(args.contains(&"s".to_string()));
    }

    #[test]
    fn test_codecs() {
        let opts = PlaybackOptions::new()
            .video_codec("h264")
            .audio_codec("aac");

        let args = opts.build_args();
        assert!(args.contains(&"-vcodec".to_string()));
        assert!(args.contains(&"h264".to_string()));
        assert!(args.contains(&"-acodec".to_string()));
        assert!(args.contains(&"aac".to_string()));
    }

    #[test]
    fn test_presets() {
        let audio_only = presets::audio_only();
        let args = audio_only.build_args();
        assert!(args.contains(&"-vn".to_string()));

        let low_latency = presets::low_latency();
        let args = low_latency.build_args();
        assert!(args.contains(&"-fast".to_string()));
        assert!(args.contains(&"-sync".to_string()));
        assert!(args.contains(&"ext".to_string()));

        let preview = presets::preview();
        let args = preview.build_args();
        assert!(args.contains(&"-t".to_string()));
        assert!(args.contains(&"-autoexit".to_string()));
    }
}