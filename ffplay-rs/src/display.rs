use ffmpeg_common::CommandBuilder;

use crate::types::ShowMode;

/// Display options for FFplay
#[derive(Debug, Clone, Default)]
pub struct DisplayOptions {
    /// Window width
    width: Option<u32>,
    /// Window height
    height: Option<u32>,
    /// Fullscreen mode
    fullscreen: bool,
    /// Disable display
    no_display: bool,
    /// Borderless window
    borderless: bool,
    /// Always on top
    always_on_top: bool,
    /// Window title
    window_title: Option<String>,
    /// Window X position
    left: Option<i32>,
    /// Window Y position
    top: Option<i32>,
    /// Show mode
    show_mode: Option<ShowMode>,
}

impl DisplayOptions {
    /// Create new display options
    pub fn new() -> Self {
        Self::default()
    }

    /// Set window width
    pub fn width(mut self, width: u32) -> Self {
        self.width = Some(width);
        self
    }

    /// Set window height
    pub fn height(mut self, height: u32) -> Self {
        self.height = Some(height);
        self
    }

    /// Set window size
    pub fn size(mut self, width: u32, height: u32) -> Self {
        self.width = Some(width);
        self.height = Some(height);
        self
    }

    /// Enable fullscreen mode
    pub fn fullscreen(mut self, enable: bool) -> Self {
        self.fullscreen = enable;
        self
    }

    /// Disable display
    pub fn no_display(mut self, enable: bool) -> Self {
        self.no_display = enable;
        self
    }

    /// Enable borderless window
    pub fn borderless(mut self, enable: bool) -> Self {
        self.borderless = enable;
        self
    }

    /// Enable always on top
    pub fn always_on_top(mut self, enable: bool) -> Self {
        self.always_on_top = enable;
        self
    }

    /// Set window title
    pub fn window_title(mut self, title: impl Into<String>) -> Self {
        self.window_title = Some(title.into());
        self
    }

    /// Set window position
    pub fn position(mut self, x: i32, y: i32) -> Self {
        self.left = Some(x);
        self.top = Some(y);
        self
    }

    /// Set left position
    pub fn left(mut self, x: i32) -> Self {
        self.left = Some(x);
        self
    }

    /// Set top position
    pub fn top(mut self, y: i32) -> Self {
        self.top = Some(y);
        self
    }

    /// Set show mode
    pub fn show_mode(mut self, mode: ShowMode) -> Self {
        self.show_mode = Some(mode);
        self
    }

    /// Build command line arguments
    pub fn build_args(&self) -> Vec<String> {
        let mut cmd = CommandBuilder::new();

        if let Some(width) = self.width {
            cmd = cmd.option("-x", width);
        }

        if let Some(height) = self.height {
            cmd = cmd.option("-y", height);
        }

        if self.fullscreen {
            cmd = cmd.flag("-fs");
        }

        if self.no_display {
            cmd = cmd.flag("-nodisp");
        }

        if self.borderless {
            cmd = cmd.flag("-noborder");
        }

        if self.always_on_top {
            cmd = cmd.flag("-alwaysontop");
        }

        if let Some(ref title) = self.window_title {
            cmd = cmd.option("-window_title", title);
        }

        if let Some(x) = self.left {
            cmd = cmd.option("-left", x);
        }

        if let Some(y) = self.top {
            cmd = cmd.option("-top", y);
        }

        if let Some(mode) = self.show_mode {
            cmd = cmd.option("-showmode", mode as u8);
        }

        cmd.build()
    }
}

/// Preset display configurations
pub mod presets {
    use super::*;

    /// Standard window (720p)
    pub fn standard() -> DisplayOptions {
        DisplayOptions::new().size(1280, 720)
    }

    /// Fullscreen mode
    pub fn fullscreen() -> DisplayOptions {
        DisplayOptions::new().fullscreen(true)
    }

    /// Picture-in-picture mode
    pub fn pip() -> DisplayOptions {
        DisplayOptions::new()
            .size(480, 270)
            .borderless(true)
            .always_on_top(true)
    }

    /// Minimal player
    pub fn minimal() -> DisplayOptions {
        DisplayOptions::new()
            .borderless(true)
    }

    /// Audio visualizer
    pub fn audio_viz() -> DisplayOptions {
        DisplayOptions::new()
            .size(800, 200)
            .show_mode(ShowMode::Waves)
    }

    /// Spectrum analyzer
    pub fn spectrum() -> DisplayOptions {
        DisplayOptions::new()
            .size(800, 400)
            .show_mode(ShowMode::Rdft)
    }

    /// Hidden player (audio only)
    pub fn hidden() -> DisplayOptions {
        DisplayOptions::new().no_display(true)
    }

    /// Centered window
    pub fn centered(width: u32, height: u32) -> DisplayOptions {
        // Note: FFplay doesn't support true centering, this is approximate
        DisplayOptions::new().size(width, height)
    }

    /// Top-left corner
    pub fn top_left(width: u32, height: u32) -> DisplayOptions {
        DisplayOptions::new()
            .size(width, height)
            .position(0, 0)
    }

    /// Top-right corner (approximate)
    pub fn top_right(width: u32, height: u32, screen_width: u32) -> DisplayOptions {
        DisplayOptions::new()
            .size(width, height)
            .position((screen_width - width) as i32, 0)
    }

    /// Bottom-left corner (approximate)
    pub fn bottom_left(width: u32, height: u32, screen_height: u32) -> DisplayOptions {
        DisplayOptions::new()
            .size(width, height)
            .position(0, (screen_height - height) as i32)
    }

    /// Bottom-right corner (approximate)
    pub fn bottom_right(
        width: u32,
        height: u32,
        screen_width: u32,
        screen_height: u32,
    ) -> DisplayOptions {
        DisplayOptions::new()
            .size(width, height)
            .position(
                (screen_width - width) as i32,
                (screen_height - height) as i32,
            )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_options() {
        let opts = DisplayOptions::new()
            .size(1920, 1080)
            .fullscreen(true)
            .window_title("Test Video");

        let args = opts.build_args();
        assert!(args.contains(&"-x".to_string()));
        assert!(args.contains(&"1920".to_string()));
        assert!(args.contains(&"-y".to_string()));
        assert!(args.contains(&"1080".to_string()));
        assert!(args.contains(&"-fs".to_string()));
        assert!(args.contains(&"-window_title".to_string()));
        assert!(args.contains(&"Test Video".to_string()));
    }

    #[test]
    fn test_position() {
        let opts = DisplayOptions::new()
            .position(100, 200);

        let args = opts.build_args();
        assert!(args.contains(&"-left".to_string()));
        assert!(args.contains(&"100".to_string()));
        assert!(args.contains(&"-top".to_string()));
        assert!(args.contains(&"200".to_string()));
    }

    #[test]
    fn test_show_mode() {
        let opts = DisplayOptions::new()
            .show_mode(ShowMode::Waves);

        let args = opts.build_args();
        assert!(args.contains(&"-showmode".to_string()));
        assert!(args.contains(&"1".to_string()));
    }

    #[test]
    fn test_presets() {
        let standard = presets::standard();
        let args = standard.build_args();
        assert!(args.contains(&"1280".to_string()));
        assert!(args.contains(&"720".to_string()));

        let pip = presets::pip();
        let args = pip.build_args();
        assert!(args.contains(&"480".to_string()));
        assert!(args.contains(&"270".to_string()));
        assert!(args.contains(&"-noborder".to_string()));
        assert!(args.contains(&"-alwaysontop".to_string()));

        let hidden = presets::hidden();
        let args = hidden.build_args();
        assert!(args.contains(&"-nodisp".to_string()));
    }
}