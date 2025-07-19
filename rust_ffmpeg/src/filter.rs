use ffmpeg_common::{utils, Result};
use std::fmt;

/// Video filter
#[derive(Debug, Clone)]
pub struct VideoFilter {
    name: String,
    params: Vec<(String, String)>,
}

impl VideoFilter {
    /// Create a new video filter
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            params: Vec::new(),
        }
    }

    /// Add a parameter
    pub fn param(mut self, key: impl Into<String>, value: impl ToString) -> Self {
        self.params.push((key.into(), value.to_string()));
        self
    }

    /// Scale filter
    pub fn scale(width: i32, height: i32) -> Self {
        Self::new("scale")
            .param("w", width)
            .param("h", height)
    }

    /// Scale with aspect ratio preservation
    pub fn scale_aspect(width: i32) -> Self {
        Self::new("scale")
            .param("w", width)
            .param("h", -1)
    }

    /// Crop filter
    pub fn crop(width: u32, height: u32, x: u32, y: u32) -> Self {
        Self::new("crop")
            .param("w", width)
            .param("h", height)
            .param("x", x)
            .param("y", y)
    }

    /// Pad filter
    pub fn pad(width: u32, height: u32) -> Self {
        Self::new("pad")
            .param("width", width)
            .param("height", height)
            .param("x", "(ow-iw)/2")
            .param("y", "(oh-ih)/2")
    }

    /// Rotate filter
    pub fn rotate(angle: f64) -> Self {
        Self::new("rotate").param("angle", angle)
    }

    /// Transpose (90 degree rotations)
    pub fn transpose(direction: TransposeDirection) -> Self {
        Self::new("transpose").param("dir", direction as u8)
    }

    /// Horizontal flip
    pub fn hflip() -> Self {
        Self::new("hflip")
    }

    /// Vertical flip
    pub fn vflip() -> Self {
        Self::new("vflip")
    }

    /// FPS filter
    pub fn fps(framerate: f64) -> Self {
        Self::new("fps").param("fps", framerate)
    }

    /// Deinterlace with yadif
    pub fn deinterlace() -> Self {
        Self::new("yadif")
    }

    /// Denoise with hqdn3d
    pub fn denoise(strength: f64) -> Self {
        Self::new("hqdn3d").param("luma_spatial", strength)
    }

    /// Sharpen with unsharp
    pub fn sharpen() -> Self {
        Self::new("unsharp")
    }

    /// Blur
    pub fn blur(radius: u32) -> Self {
        Self::new("boxblur").param("lr", radius).param("lp", 1)
    }

    /// Overlay filter
    pub fn overlay(x: impl Into<String>, y: impl Into<String>) -> Self {
        Self::new("overlay")
            .param("x", x.into())
            .param("y", y.into())
    }

    /// Draw text
    pub fn drawtext(text: impl Into<String>) -> Self {
        Self::new("drawtext")
            .param("text", utils::escape_filter_string(&text.into()))
    }

    /// Fade in
    pub fn fade_in(duration: f64) -> Self {
        Self::new("fade")
            .param("type", "in")
            .param("duration", duration)
    }

    /// Fade out
    pub fn fade_out(duration: f64, start_time: f64) -> Self {
        Self::new("fade")
            .param("type", "out")
            .param("duration", duration)
            .param("start_time", start_time)
    }

    /// Set PTS (presentation timestamp)
    pub fn setpts(expr: impl Into<String>) -> Self {
        Self::new("setpts").param("expr", expr.into())
    }

    /// Select frames
    pub fn select(expr: impl Into<String>) -> Self {
        Self::new("select").param("expr", expr.into())
    }

    /// EQ (brightness/contrast/saturation)
    pub fn eq() -> Self {
        Self::new("eq")
    }

    /// Add brightness adjustment to EQ filter
    pub fn brightness(mut self, value: f64) -> Self {
        if self.name == "eq" {
            self.params.push(("brightness".to_string(), value.to_string()));
        }
        self
    }

    /// Add contrast adjustment to EQ filter
    pub fn contrast(mut self, value: f64) -> Self {
        if self.name == "eq" {
            self.params.push(("contrast".to_string(), value.to_string()));
        }
        self
    }

    /// Add saturation adjustment to EQ filter
    pub fn saturation(mut self, value: f64) -> Self {
        if self.name == "eq" {
            self.params.push(("saturation".to_string(), value.to_string()));
        }
        self
    }

    /// Format conversion
    pub fn format(pix_fmt: impl Into<String>) -> Self {
        Self::new("format").param("pix_fmts", pix_fmt.into())
    }
}

impl fmt::Display for VideoFilter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)?;
        if !self.params.is_empty() {
            write!(f, "=")?;
            let params: Vec<String> = self.params
                .iter()
                .map(|(k, v)| format!("{}={}", k, v))
                .collect();
            write!(f, "{}", params.join(":"))?;
        }
        Ok(())
    }
}

/// Audio filter
#[derive(Debug, Clone)]
pub struct AudioFilter {
    name: String,
    params: Vec<(String, String)>,
}

impl AudioFilter {
    /// Create a new audio filter
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            params: Vec::new(),
        }
    }

    /// Add a parameter
    pub fn param(mut self, key: impl Into<String>, value: impl ToString) -> Self {
        self.params.push((key.into(), value.to_string()));
        self
    }

    /// Volume adjustment
    pub fn volume(level: f64) -> Self {
        Self::new("volume").param("volume", level)
    }

    /// Audio normalization
    pub fn loudnorm() -> Self {
        Self::new("loudnorm")
            .param("I", -16)
            .param("TP", -1.5)
            .param("LRA", 11)
    }

    /// Dynamic audio normalizer
    pub fn dynaudnorm() -> Self {
        Self::new("dynaudnorm")
    }

    /// High pass filter
    pub fn highpass(frequency: u32) -> Self {
        Self::new("highpass").param("f", frequency)
    }

    /// Low pass filter
    pub fn lowpass(frequency: u32) -> Self {
        Self::new("lowpass").param("f", frequency)
    }

    /// Audio fade in
    pub fn afade_in(duration: f64) -> Self {
        Self::new("afade")
            .param("type", "in")
            .param("duration", duration)
    }

    /// Audio fade out
    pub fn afade_out(duration: f64, start_time: f64) -> Self {
        Self::new("afade")
            .param("type", "out")
            .param("duration", duration)
            .param("start_time", start_time)
    }

    /// Resample audio
    pub fn aresample(sample_rate: u32) -> Self {
        Self::new("aresample").param("sample_rate", sample_rate)
    }

    /// Change tempo without changing pitch
    pub fn atempo(tempo: f64) -> Self {
        Self::new("atempo").param("tempo", tempo)
    }

    /// Audio delay
    pub fn adelay(delays: impl Into<String>) -> Self {
        Self::new("adelay").param("delays", delays.into())
    }

    /// Audio echo
    pub fn aecho(delays: impl Into<String>, decays: impl Into<String>) -> Self {
        Self::new("aecho")
            .param("delays", delays.into())
            .param("decays", decays.into())
    }

    /// Compressor
    pub fn acompressor() -> Self {
        Self::new("acompressor")
    }

    /// Limiter
    pub fn alimiter() -> Self {
        Self::new("alimiter")
    }

    /// Gate
    pub fn agate() -> Self {
        Self::new("agate")
    }

    /// EQ band
    pub fn anequalizer(frequency: u32, width: f64, gain: f64) -> Self {
        Self::new("anequalizer")
            .param("frequency", frequency)
            .param("width", width)
            .param("gain", gain)
    }

    /// Channel manipulation
    pub fn channelmap(map: impl Into<String>) -> Self {
        Self::new("channelmap").param("map", map.into())
    }

    /// Mix channels
    pub fn amerge() -> Self {
        Self::new("amerge")
    }

    /// Split channels
    pub fn channelsplit() -> Self {
        Self::new("channelsplit")
    }
}

impl fmt::Display for AudioFilter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)?;
        if !self.params.is_empty() {
            write!(f, "=")?;
            let params: Vec<String> = self.params
                .iter()
                .map(|(k, v)| format!("{}={}", k, v))
                .collect();
            write!(f, "{}", params.join(":"))?;
        }
        Ok(())
    }
}

/// Transpose direction
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum TransposeDirection {
    CounterClockwiseFlip = 0,
    Clockwise = 1,
    CounterClockwise = 2,
    ClockwiseFlip = 3,
}

/// Complex filter graph builder
#[derive(Debug, Clone, Default)]
pub struct FilterGraph {
    nodes: Vec<FilterNode>,
    edges: Vec<FilterEdge>,
}

#[derive(Debug, Clone)]
struct FilterNode {
    id: String,
    filter: String,
    inputs: Vec<String>,
    outputs: Vec<String>,
}

#[derive(Debug, Clone)]
struct FilterEdge {
    from: String,
    to: String,
}

impl FilterGraph {
    /// Create a new filter graph
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a filter node
    pub fn add_filter(
        mut self,
        filter: impl Into<String>,
        inputs: Vec<String>,
        outputs: Vec<String>,
    ) -> Self {
        let id = format!("f{}", self.nodes.len());
        self.nodes.push(FilterNode {
            id: id.clone(),
            filter: filter.into(),
            inputs,
            outputs,
        });
        self
    }

    /// Connect two filter nodes
    pub fn connect(mut self, from: impl Into<String>, to: impl Into<String>) -> Self {
        self.edges.push(FilterEdge {
            from: from.into(),
            to: to.into(),
        });
        self
    }

    /// Build the filter graph string
    pub fn build(&self) -> String {
        let mut parts = Vec::new();

        for node in &self.nodes {
            let mut part = String::new();

            // Inputs
            if !node.inputs.is_empty() {
                part.push_str(&node.inputs.join(""));
            }

            // Filter
            part.push_str(&node.filter);

            // Outputs
            if !node.outputs.is_empty() {
                part.push_str(&node.outputs.join(""));
            }

            parts.push(part);
        }

        parts.join(";")
    }
}

/// Common filter chains
pub mod chains {
    use super::*;

    /// Create a thumbnail extraction filter chain
    pub fn thumbnail() -> Vec<VideoFilter> {
        vec![
            VideoFilter::select("eq(pict_type\\,I)"),
            VideoFilter::scale(320, -1),
        ]
    }

    /// Create a GIF optimization filter chain
    pub fn gif_optimize(width: u32, fps: f64) -> Vec<VideoFilter> {
        vec![
            VideoFilter::fps(fps),
            VideoFilter::scale(width as i32, -1),
            VideoFilter::new("split").param("outputs", 2),
            VideoFilter::new("palettegen"),
            VideoFilter::new("paletteuse"),
        ]
    }

    /// Stabilization filter chain
    pub fn stabilize() -> Vec<VideoFilter> {
        vec![
            VideoFilter::new("vidstabdetect").param("shakiness", 5),
            VideoFilter::new("vidstabtransform"),
        ]
    }

    /// Cinematic look
    pub fn cinematic() -> Vec<VideoFilter> {
        vec![
            VideoFilter::eq()
                .contrast(1.2)
                .brightness(-0.05)
                .saturation(0.8),
            VideoFilter::new("curves")
                .param("preset", "vintage"),
            VideoFilter::new("vignette"),
        ]
    }

    /// Upscale with enhancement
    pub fn upscale_enhance(scale: u32) -> Vec<VideoFilter> {
        vec![
            VideoFilter::scale(scale as i32, -1),
            VideoFilter::sharpen(),
            VideoFilter::new("hqdn3d").param("luma_spatial", 4),
        ]
    }

    /// Audio mastering chain
    pub fn audio_master() -> Vec<AudioFilter> {
        vec![
            AudioFilter::highpass(80),
            AudioFilter::acompressor()
                .param("threshold", -20)
                .param("ratio", 4)
                .param("attack", 5)
                .param("release", 50),
            AudioFilter::anequalizer(100, 100.0, 2.0),
            AudioFilter::anequalizer(1000, 500.0, -1.0),
            AudioFilter::anequalizer(10000, 2000.0, 1.0),
            AudioFilter::alimiter()
                .param("limit", -0.5),
            AudioFilter::loudnorm(),
        ]
    }

    /// Podcast audio processing
    pub fn podcast_audio() -> Vec<AudioFilter> {
        vec![
            AudioFilter::highpass(100),
            AudioFilter::lowpass(15000),
            AudioFilter::agate()
                .param("threshold", -35)
                .param("range", -40),
            AudioFilter::acompressor()
                .param("threshold", -15)
                .param("ratio", 3),
            AudioFilter::loudnorm(),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_video_filters() {
        let scale = VideoFilter::scale(1920, 1080);
        assert_eq!(scale.to_string(), "scale=w=1920:h=1080");

        let crop = VideoFilter::crop(640, 480, 100, 50);
        assert_eq!(crop.to_string(), "crop=w=640:h=480:x=100:y=50");

        let text = VideoFilter::drawtext("Hello, World!");
        assert!(text.to_string().contains("drawtext=text="));
    }

    #[test]
    fn test_audio_filters() {
        let volume = AudioFilter::volume(0.5);
        assert_eq!(volume.to_string(), "volume=volume=0.5");

        let tempo = AudioFilter::atempo(1.5);
        assert_eq!(tempo.to_string(), "atempo=tempo=1.5");
    }

    #[test]
    fn test_filter_graph() {
        let graph = FilterGraph::new()
            .add_filter("scale=640:480", vec!["[0:v]".to_string()], vec!["[scaled]".to_string()])
            .add_filter("overlay", vec!["[scaled]".to_string(), "[1:v]".to_string()], vec!["[out]".to_string()]);

        let result = graph.build();
        assert!(result.contains("[0:v]scale=640:480[scaled]"));
        assert!(result.contains("[scaled][1:v]overlay[out]"));
    }
}