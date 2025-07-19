/// Show mode for FFplay
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ShowMode {
    /// Show video (default)
    Video = 0,
    /// Show audio waves
    Waves = 1,
    /// Show audio frequency band using RDFT
    Rdft = 2,
}

impl ShowMode {
    /// Get the numeric value for command line
    pub fn as_u8(self) -> u8 {
        self as u8
    }

    /// Get the string representation
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Video => "video",
            Self::Waves => "waves",
            Self::Rdft => "rdft",
        }
    }
}

impl Default for ShowMode {
    fn default() -> Self {
        Self::Video
    }
}

/// Key bindings for FFplay
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyBinding {
    /// Quit
    Q,
    /// Quit (ESC)
    Esc,
    /// Toggle fullscreen
    F,
    /// Pause/Resume
    P,
    /// Pause/Resume (Space)
    Space,
    /// Toggle mute
    M,
    /// Decrease volume
    Nine,
    /// Increase volume
    Zero,
    /// Decrease volume (/)
    Slash,
    /// Increase volume (*)
    Asterisk,
    /// Cycle audio channel
    A,
    /// Cycle video channel
    V,
    /// Cycle subtitle channel
    T,
    /// Cycle program
    C,
    /// Cycle video filters or show modes
    W,
    /// Step to next frame
    S,
    /// Seek backward 10 seconds
    Left,
    /// Seek forward 10 seconds
    Right,
    /// Seek backward 1 minute
    Down,
    /// Seek forward 1 minute
    Up,
    /// Seek to previous chapter or -10 minutes
    PageDown,
    /// Seek to next chapter or +10 minutes
    PageUp,
}

impl KeyBinding {
    /// Get the key description
    pub fn description(self) -> &'static str {
        match self {
            Self::Q | Self::Esc => "Quit",
            Self::F => "Toggle fullscreen",
            Self::P | Self::Space => "Pause/Resume",
            Self::M => "Toggle mute",
            Self::Nine | Self::Slash => "Decrease volume",
            Self::Zero | Self::Asterisk => "Increase volume",
            Self::A => "Cycle audio channel",
            Self::V => "Cycle video channel",
            Self::T => "Cycle subtitle channel",
            Self::C => "Cycle program",
            Self::W => "Cycle video filters or show modes",
            Self::S => "Step to next frame",
            Self::Left => "Seek backward 10 seconds",
            Self::Right => "Seek forward 10 seconds",
            Self::Down => "Seek backward 1 minute",
            Self::Up => "Seek forward 1 minute",
            Self::PageDown => "Seek to previous chapter or -10 minutes",
            Self::PageUp => "Seek to next chapter or +10 minutes",
        }
    }
}

/// Mouse actions for FFplay
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MouseAction {
    /// Right click to seek
    RightClick,
    /// Left double-click to toggle fullscreen
    LeftDoubleClick,
}

impl MouseAction {
    /// Get the action description
    pub fn description(self) -> &'static str {
        match self {
            Self::RightClick => "Seek to percentage in file",
            Self::LeftDoubleClick => "Toggle fullscreen",
        }
    }
}

/// FFplay window state
#[derive(Debug, Clone, Default)]
pub struct WindowState {
    /// Window width
    pub width: Option<u32>,
    /// Window height
    pub height: Option<u32>,
    /// Window X position
    pub x: Option<i32>,
    /// Window Y position
    pub y: Option<i32>,
    /// Fullscreen state
    pub fullscreen: bool,
    /// Window title
    pub title: Option<String>,
}

/// Playback state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlaybackState {
    /// Playing
    Playing,
    /// Paused
    Paused,
    /// Stopped
    Stopped,
}

/// Audio visualization type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VisualizationType {
    /// No visualization
    None,
    /// Waveform
    Waveform,
    /// Spectrum (RDFT)
    Spectrum,
}

impl From<ShowMode> for VisualizationType {
    fn from(mode: ShowMode) -> Self {
        match mode {
            ShowMode::Video => Self::None,
            ShowMode::Waves => Self::Waveform,
            ShowMode::Rdft => Self::Spectrum,
        }
    }
}

/// Hardware acceleration options
#[derive(Debug, Clone)]
pub struct HwAccelOptions {
    /// Enable hardware acceleration
    pub enabled: bool,
    /// Hardware acceleration method
    pub method: Option<String>,
    /// Device to use
    pub device: Option<String>,
}

impl Default for HwAccelOptions {
    fn default() -> Self {
        Self {
            enabled: false,
            method: None,
            device: None,
        }
    }
}

impl HwAccelOptions {
    /// Create new hardware acceleration options
    pub fn new() -> Self {
        Self::default()
    }

    /// Enable hardware acceleration
    pub fn enable(mut self) -> Self {
        self.enabled = true;
        self
    }

    /// Set acceleration method
    pub fn method(mut self, method: impl Into<String>) -> Self {
        self.method = Some(method.into());
        self.enabled = true;
        self
    }

    /// Set device
    pub fn device(mut self, device: impl Into<String>) -> Self {
        self.device = Some(device.into());
        self
    }

    /// Use VAAPI acceleration
    #[cfg(target_os = "linux")]
    pub fn vaapi() -> Self {
        Self::new().method("vaapi")
    }

    /// Use VDPAU acceleration
    #[cfg(target_os = "linux")]
    pub fn vdpau() -> Self {
        Self::new().method("vdpau")
    }

    /// Use VideoToolbox acceleration
    #[cfg(target_os = "macos")]
    pub fn videotoolbox() -> Self {
        Self::new().method("videotoolbox")
    }

    /// Use DXVA2 acceleration
    #[cfg(target_os = "windows")]
    pub fn dxva2() -> Self {
        Self::new().method("dxva2")
    }

    /// Use D3D11VA acceleration
    #[cfg(target_os = "windows")]
    pub fn d3d11va() -> Self {
        Self::new().method("d3d11va")
    }

    /// Use CUDA acceleration
    pub fn cuda() -> Self {
        Self::new().method("cuda")
    }

    /// Use QSV acceleration
    pub fn qsv() -> Self {
        Self::new().method("qsv")
    }
}

/// Vulkan renderer options
#[derive(Debug, Clone, Default)]
pub struct VulkanOptions {
    /// Enable Vulkan renderer
    pub enabled: bool,
    /// Vulkan parameters
    pub params: Vec<(String, String)>,
}

impl VulkanOptions {
    /// Create new Vulkan options
    pub fn new() -> Self {
        Self::default()
    }

    /// Enable Vulkan renderer
    pub fn enable(mut self) -> Self {
        self.enabled = true;
        self
    }

    /// Add a Vulkan parameter
    pub fn param(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.params.push((key.into(), value.into()));
        self.enabled = true;
        self
    }

    /// Build parameter string
    pub fn build_params(&self) -> Option<String> {
        if self.params.is_empty() {
            None
        } else {
            Some(
                self.params
                    .iter()
                    .map(|(k, v)| format!("{}={}", k, v))
                    .collect::<Vec<_>>()
                    .join(":")
            )
        }
    }
}

/// Input statistics
#[derive(Debug, Clone, Default)]
pub struct InputStats {
    /// Stream duration
    pub duration: Option<f64>,
    /// Codec parameters
    pub codec_params: Option<String>,
    /// Current position
    pub position: Option<f64>,
    /// Audio/video sync drift
    pub av_sync_drift: Option<f64>,
}

/// Key binding help text
pub fn get_key_bindings() -> Vec<(String, String)> {
    vec![
        ("q, ESC".to_string(), "Quit".to_string()),
        ("f".to_string(), "Toggle full screen".to_string()),
        ("p, SPC".to_string(), "Pause".to_string()),
        ("m".to_string(), "Toggle mute".to_string()),
        ("9, 0".to_string(), "Decrease/increase volume".to_string()),
        ("/, *".to_string(), "Decrease/increase volume".to_string()),
        ("a".to_string(), "Cycle audio channel".to_string()),
        ("v".to_string(), "Cycle video channel".to_string()),
        ("t".to_string(), "Cycle subtitle channel".to_string()),
        ("c".to_string(), "Cycle program".to_string()),
        ("w".to_string(), "Cycle video filters or show modes".to_string()),
        ("s".to_string(), "Step to next frame".to_string()),
        ("left/right".to_string(), "Seek backward/forward 10 seconds".to_string()),
        ("down/up".to_string(), "Seek backward/forward 1 minute".to_string()),
        ("page down/up".to_string(), "Seek to previous/next chapter or ±10 min".to_string()),
        ("mouse right click".to_string(), "Seek to percentage in file".to_string()),
        ("mouse left double-click".to_string(), "Toggle full screen".to_string()),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_show_mode() {
        assert_eq!(ShowMode::Video.as_u8(), 0);
        assert_eq!(ShowMode::Waves.as_u8(), 1);
        assert_eq!(ShowMode::Rdft.as_u8(), 2);

        assert_eq!(ShowMode::Video.as_str(), "video");
        assert_eq!(ShowMode::default(), ShowMode::Video);
    }

    #[test]
    fn test_key_binding_description() {
        assert_eq!(KeyBinding::Q.description(), "Quit");
        assert_eq!(KeyBinding::F.description(), "Toggle fullscreen");
        assert_eq!(KeyBinding::Left.description(), "Seek backward 10 seconds");
    }

    #[test]
    fn test_visualization_type() {
        assert_eq!(VisualizationType::from(ShowMode::Video), VisualizationType::None);
        assert_eq!(VisualizationType::from(ShowMode::Waves), VisualizationType::Waveform);
        assert_eq!(VisualizationType::from(ShowMode::Rdft), VisualizationType::Spectrum);
    }

    #[test]
    fn test_hwaccel_options() {
        let cuda = HwAccelOptions::cuda();
        assert!(cuda.enabled);
        assert_eq!(cuda.method, Some("cuda".to_string()));

        let custom = HwAccelOptions::new()
            .method("vaapi")
            .device("/dev/dri/renderD128");
        assert!(custom.enabled);
        assert_eq!(custom.method, Some("vaapi".to_string()));
        assert_eq!(custom.device, Some("/dev/dri/renderD128".to_string()));
    }

    #[test]
    fn test_vulkan_options() {
        let vulkan = VulkanOptions::new()
            .enable()
            .param("device_index", "0")
            .param("queue_count", "4");

        assert!(vulkan.enabled);
        assert_eq!(vulkan.build_params(), Some("device_index=0:queue_count=4".to_string()));
    }
}