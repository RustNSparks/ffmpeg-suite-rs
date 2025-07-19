use ffmpeg_common::{CommandBuilder, Duration, MediaPath, PixelFormat, Result, Size, Error};
use std::collections::HashMap;
use std::time::Duration as StdDuration;

/// Input specification for FFmpeg
#[derive(Debug, Clone)]
pub struct Input {
    /// Source path or URL
    source: MediaPath,
    /// Format to force
    format: Option<String>,
    /// Seek to position before reading
    seek: Option<Duration>,
    /// Duration to read
    duration: Option<Duration>,
    /// Frame rate
    framerate: Option<f64>,
    /// Video size
    video_size: Option<(u32, u32)>,
    /// Pixel format
    pixel_format: Option<PixelFormat>,
    /// Audio sample rate
    sample_rate: Option<u32>,
    /// Audio channels
    channels: Option<u32>,
    /// Loop input
    loop_count: Option<i32>,
    /// Realtime input
    realtime: bool,
    /// Thread queue size
    thread_queue_size: Option<u32>,
    /// Custom options
    options: HashMap<String, String>,
    /// Decoder to use
    decoder: Option<String>,
    /// Hardware decoder
    hwaccel_device: Option<String>,
    /// Input buffer size
    buffer_size: Option<Size>,
    /// Discard threshold
    discard_threshold: Option<StdDuration>,
}

impl Input {
    /// Create a new input from a source
    pub fn new(source: impl Into<MediaPath>) -> Self {
        Self {
            source: source.into(),
            format: None,
            seek: None,
            duration: None,
            framerate: None,
            video_size: None,
            pixel_format: None,
            sample_rate: None,
            channels: None,
            loop_count: None,
            realtime: false,
            thread_queue_size: None,
            options: HashMap::new(),
            decoder: None,
            hwaccel_device: None,
            buffer_size: None,
            discard_threshold: None,
        }
    }

    /// Force input format
    pub fn format(mut self, format: impl Into<String>) -> Self {
        self.format = Some(format.into());
        self
    }

    /// Seek to position before reading
    pub fn seek(mut self, position: Duration) -> Self {
        self.seek = Some(position);
        self
    }

    /// Set duration to read
    pub fn duration(mut self, duration: Duration) -> Self {
        self.duration = Some(duration);
        self
    }

    /// Set input frame rate
    pub fn framerate(mut self, fps: f64) -> Self {
        self.framerate = Some(fps);
        self
    }

    /// Set video size (for raw video)
    pub fn video_size(mut self, width: u32, height: u32) -> Self {
        self.video_size = Some((width, height));
        self
    }

    /// Set pixel format (for raw video)
    pub fn pixel_format(mut self, format: PixelFormat) -> Self {
        self.pixel_format = Some(format);
        self
    }

    /// Set audio sample rate
    pub fn sample_rate(mut self, rate: u32) -> Self {
        self.sample_rate = Some(rate);
        self
    }

    /// Set number of audio channels
    pub fn channels(mut self, channels: u32) -> Self {
        self.channels = Some(channels);
        self
    }

    /// Loop input (0 = no loop, -1 = infinite)
    pub fn loop_input(mut self, count: i32) -> Self {
        self.loop_count = Some(count);
        self
    }

    /// Enable realtime input reading
    pub fn realtime(mut self, enable: bool) -> Self {
        self.realtime = enable;
        self
    }

    /// Set thread queue size
    pub fn thread_queue_size(mut self, size: u32) -> Self {
        self.thread_queue_size = Some(size);
        self
    }

    /// Add a custom option
    pub fn option(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.options.insert(key.into(), value.into());
        self
    }

    /// Set decoder to use
    pub fn decoder(mut self, decoder: impl Into<String>) -> Self {
        self.decoder = Some(decoder.into());
        self
    }

    /// Set hardware acceleration device
    pub fn hwaccel_device(mut self, device: impl Into<String>) -> Self {
        self.hwaccel_device = Some(device.into());
        self
    }

    /// Set input buffer size
    pub fn buffer_size(mut self, size: Size) -> Self {
        self.buffer_size = Some(size);
        self
    }

    /// Set discard threshold
    pub fn discard_threshold(mut self, threshold: StdDuration) -> Self {
        self.discard_threshold = Some(threshold);
        self
    }

    /// Build command line arguments for this input
    pub fn build_args(&self) -> Vec<String> {
        let mut cmd = CommandBuilder::new();

        // Format options (before -i)
        if let Some(ref format) = self.format {
            cmd = cmd.option("-f", format);
        }

        if let Some(seek) = self.seek {
            cmd = cmd.option("-ss", seek.to_ffmpeg_format());
        }

        if let Some(duration) = self.duration {
            cmd = cmd.option("-t", duration.to_ffmpeg_format());
        }

        if let Some(fps) = self.framerate {
            cmd = cmd.option("-framerate", fps);
        }

        if let Some((width, height)) = self.video_size {
            cmd = cmd.option("-video_size", format!("{}x{}", width, height));
        }

        if let Some(ref pix_fmt) = self.pixel_format {
            cmd = cmd.option("-pixel_format", pix_fmt.as_str());
        }

        if let Some(rate) = self.sample_rate {
            cmd = cmd.option("-ar", rate);
        }

        if let Some(channels) = self.channels {
            cmd = cmd.option("-ac", channels);
        }

        if let Some(loop_count) = self.loop_count {
            cmd = cmd.option("-stream_loop", loop_count);
        }

        if self.realtime {
            cmd = cmd.flag("-re");
        }

        if let Some(size) = self.thread_queue_size {
            cmd = cmd.option("-thread_queue_size", size);
        }

        if let Some(ref decoder) = self.decoder {
            cmd = cmd.option("-c:v", decoder);
        }

        if let Some(ref device) = self.hwaccel_device {
            cmd = cmd.option("-hwaccel_device", device);
        }

        if let Some(ref size) = self.buffer_size {
            cmd = cmd.option("-bufsize", size.as_bytes());
        }

        if let Some(ref _threshold) = self.discard_threshold {
            cmd = cmd.option("-fflags", "+discardcorrupt");
            cmd = cmd.option("-err_detect", "ignore_err");
        }

        // Custom options
        for (key, value) in &self.options {
            cmd = cmd.option(format!("-{}", key), value);
        }

        // Add -i and the input path
        cmd = cmd.option("-i", self.source.as_str());

        cmd.build()
    }
}

/// Builder for device inputs (cameras, screens, etc.)
#[derive(Debug, Clone)]
pub struct DeviceInput {
    /// Device type (e.g., "v4l2", "dshow", "avfoundation")
    device_type: String,
    /// Device name or identifier
    device: String,
    /// Additional options
    options: HashMap<String, String>,
}

impl DeviceInput {
    /// Create a new device input
    pub fn new(device_type: impl Into<String>, device: impl Into<String>) -> Self {
        Self {
            device_type: device_type.into(),
            device: device.into(),
            options: HashMap::new(),
        }
    }

    /// Create a webcam input (platform-specific)
    #[cfg(target_os = "linux")]
    pub fn webcam(device: impl Into<String>) -> Self {
        Self::new("v4l2", device)
    }

    #[cfg(target_os = "windows")]
    pub fn webcam(device: impl Into<String>) -> Self {
        Self::new("dshow", format!("video={}", device.into()))
    }

    #[cfg(target_os = "macos")]
    pub fn webcam(device: impl Into<String>) -> Self {
        Self::new("avfoundation", device)
    }

    /// Create a screen capture input
    #[cfg(target_os = "linux")]
    pub fn screen_capture() -> Self {
        Self::new("x11grab", ":0.0")
    }

    #[cfg(target_os = "windows")]
    pub fn screen_capture() -> Self {
        Self::new("gdigrab", "desktop")
    }

    #[cfg(target_os = "macos")]
    pub fn screen_capture() -> Self {
        Self::new("avfoundation", "1:none")
    }

    /// Add an option
    pub fn option(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.options.insert(key.into(), value.into());
        self
    }

    /// Convert to regular Input
    pub fn into_input(self) -> Input {
        let mut input = Input::new(self.device).format(self.device_type);

        for (key, value) in self.options {
            input = input.option(key, value);
        }

        input
    }
}

/// Builder for network stream inputs
#[derive(Debug, Clone)]
pub struct StreamInput {
    /// Stream URL
    url: String,
    /// Protocol options
    options: HashMap<String, String>,
    /// Reconnect on error
    reconnect: bool,
    /// Reconnect delay
    reconnect_delay: Option<StdDuration>,
    /// Maximum reconnect attempts
    reconnect_attempts: Option<u32>,
}

impl StreamInput {
    /// Create a new stream input
    pub fn new(url: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            options: HashMap::new(),
            reconnect: false,
            reconnect_delay: None,
            reconnect_attempts: None,
        }
    }

    /// Create an RTMP input
    pub fn rtmp(url: impl Into<String>) -> Self {
        Self::new(url)
    }

    /// Create an RTSP input
    pub fn rtsp(url: impl Into<String>) -> Self {
        Self::new(url).option("rtsp_transport", "tcp")
    }

    /// Create an HTTP/HTTPS input
    pub fn http(url: impl Into<String>) -> Self {
        Self::new(url)
    }

    /// Enable reconnection on error
    pub fn reconnect(mut self, enable: bool) -> Self {
        self.reconnect = enable;
        self
    }

    /// Set reconnection delay
    pub fn reconnect_delay(mut self, delay: StdDuration) -> Self {
        self.reconnect_delay = Some(delay);
        self
    }

    /// Set maximum reconnection attempts
    pub fn reconnect_attempts(mut self, attempts: u32) -> Self {
        self.reconnect_attempts = Some(attempts);
        self
    }

    /// Add a protocol option
    pub fn option(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.options.insert(key.into(), value.into());
        self
    }

    /// Set user agent for HTTP
    pub fn user_agent(self, agent: impl Into<String>) -> Self {
        self.option("user_agent", agent)
    }

    /// Set timeout
    pub fn timeout(self, timeout: StdDuration) -> Self {
        self.option("timeout", timeout.as_micros().to_string())
    }

    /// Convert to regular Input
    pub fn into_input(self) -> Input {
        let mut input = Input::new(self.url);

        if self.reconnect {
            input = input.option("reconnect", "1");

            if let Some(delay) = self.reconnect_delay {
                input = input.option("reconnect_delay_max", delay.as_secs().to_string());
            }

            if let Some(attempts) = self.reconnect_attempts {
                input = input.option("reconnect_streamed", attempts.to_string());
            }
        }

        for (key, value) in self.options {
            input = input.option(key, value);
        }

        input
    }
}

/// Builder for concatenating multiple inputs
#[derive(Debug, Clone)]
pub struct ConcatInput {
    /// List of input paths
    inputs: Vec<MediaPath>,
    /// Use concat demuxer instead of filter
    use_demuxer: bool,
}

impl ConcatInput {
    /// Create a new concat input
    pub fn new() -> Self {
        Self {
            inputs: Vec::new(),
            use_demuxer: false,
        }
    }

    /// Add an input file
    pub fn add_input(mut self, path: impl Into<MediaPath>) -> Self {
        self.inputs.push(path.into());
        self
    }

    /// Add multiple input files
    pub fn add_inputs(mut self, paths: impl IntoIterator<Item = impl Into<MediaPath>>) -> Self {
        self.inputs.extend(paths.into_iter().map(Into::into));
        self
    }

    /// Use concat demuxer (requires same codec parameters)
    pub fn use_demuxer(mut self, enable: bool) -> Self {
        self.use_demuxer = enable;
        self
    }

    /// Create inputs for FFmpeg
    pub fn into_inputs(self) -> Result<Vec<Input>> {
        if self.inputs.is_empty() {
            return Err(Error::InvalidArgument(
                "No inputs provided for concatenation".to_string(),
            ));
        }

        if self.use_demuxer {
            // In a real implementation, you would write a temporary file list.
            // For this example, we'll use the `concat:` protocol which works for
            // specific container formats like MPEG-TS.
            let concat_string = self
                .inputs
                .iter()
                .map(|p| p.as_str())
                .collect::<Vec<_>>()
                .join("|");

            Ok(vec![Input::new(format!("concat:{}", concat_string))])
        } else {
            // Return individual inputs for filter-based concatenation
            Ok(self.inputs.into_iter().map(Input::new).collect())
        }
    }
}

impl Default for ConcatInput {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_builder() {
        let input = Input::new("input.mp4")
            .format("mp4")
            .seek(Duration::from_secs(10))
            .duration(Duration::from_secs(30))
            .option("custom", "value");

        let args = input.build_args();

        // The actual args are: ["-f", "mp4", "-ss", "00:00:10", "-t", "00:00:30", "-custom", "value", "-i", "input.mp4"]

        assert!(args.contains(&"-f".to_string()));
        assert!(args.contains(&"mp4".to_string()));
        assert!(args.contains(&"-ss".to_string()));
        // Corrected assertion for seek time
        assert!(args.contains(&"00:00:10".to_string()));
        assert!(args.contains(&"-t".to_string()));
        // Corrected assertion for duration
        assert!(args.contains(&"00:00:30".to_string()));
        assert!(args.contains(&"-custom".to_string()));
        assert!(args.contains(&"value".to_string()));
        assert!(args.contains(&"-i".to_string()));
        assert!(args.contains(&"input.mp4".to_string()));
    }
    #[test]
    fn test_stream_input() {
        let input = StreamInput::rtsp("rtsp://example.com/stream")
            .reconnect(true)
            .timeout(StdDuration::from_secs(10))
            .into_input();

        let args = input.build_args();
        assert!(args.contains(&"-rtsp_transport".to_string()));
        assert!(args.contains(&"tcp".to_string()));
        assert!(args.contains(&"-reconnect".to_string()));
        assert!(args.contains(&"1".to_string()));
        assert!(args.contains(&"-timeout".to_string()));
    }

    #[test]
    fn test_concat_input() {
        let concat = ConcatInput::new()
            .add_input("file1.mp4")
            .add_input("file2.mp4")
            .add_input("file3.mp4");

        let inputs = concat.into_inputs().unwrap();
        assert_eq!(inputs.len(), 3);
    }
}
