use regex::Regex;
use std::collections::HashMap;
use std::path::Path;
use once_cell::sync::Lazy;

use crate::error::{Error, Result};

/// Regular expressions for parsing
static TIME_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^(\d{2}):(\d{2}):(\d{2})(?:\.(\d+))?$").unwrap()
});

static BITRATE_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^(\d+(?:\.\d+)?)\s*([kmgKMG])?(?:bit|bps|b)?(?:/s)?$").unwrap()
});

static RESOLUTION_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^(\d+)[xX](\d+)$").unwrap()
});

/// Parse a bitrate string (e.g., "128k", "5M", "1000")
/// Parse a bitrate string (e.g., "128k", "5M", "1000")
pub fn parse_bitrate(s: &str) -> Result<u64> {
    let s = s.trim();

    if let Some(captures) = BITRATE_REGEX.captures(s) {
        let number: f64 = captures[1].parse()
            .map_err(|_| Error::ParseError(format!("Invalid bitrate number: {}", &captures[1])))?;

        // Store the lowercase string in a variable to extend its lifetime.
        let suffix = captures.get(2).map(|m| m.as_str().to_lowercase());

        // Match on a slice (`&str`) of the `suffix` string.
        let multiplier = match suffix.as_deref() {
            Some("k") => 1_000.0,
            Some("m") => 1_000_000.0,
            Some("g") => 1_000_000_000.0,
            None => 1.0,
            _ => return Err(Error::ParseError(format!("Invalid bitrate suffix in: {}", s))),
        };

        Ok((number * multiplier) as u64)
    } else {
        // Try parsing as plain number
        s.parse::<u64>()
            .map_err(|_| Error::ParseError(format!("Invalid bitrate: {}", s)))
    }
}

/// Parse a resolution string (e.g., "1920x1080")
pub fn parse_resolution(s: &str) -> Result<(u32, u32)> {
    if let Some(captures) = RESOLUTION_REGEX.captures(s.trim()) {
        let width: u32 = captures[1].parse()
            .map_err(|_| Error::ParseError(format!("Invalid width: {}", &captures[1])))?;
        let height: u32 = captures[2].parse()
            .map_err(|_| Error::ParseError(format!("Invalid height: {}", &captures[2])))?;
        Ok((width, height))
    } else {
        Err(Error::ParseError(format!("Invalid resolution format: {}", s)))
    }
}

/// Parse key=value pairs from FFmpeg output
pub fn parse_key_value_pairs(text: &str) -> HashMap<String, String> {
    let mut map = HashMap::new();

    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        if let Some((key, value)) = line.split_once('=') {
            map.insert(key.trim().to_string(), value.trim().to_string());
        }
    }

    map
}

/// Escape a string for use in filter graphs
pub fn escape_filter_string(s: &str) -> String {
    s.chars()
        .flat_map(|c| match c {
            '\\' => vec!['\\', '\\'],
            ':' => vec!['\\', ':'],
            '\'' => vec!['\\', '\''],
            '[' => vec!['\\', '['],
            ']' => vec!['\\', ']'],
            ',' => vec!['\\', ','],
            ';' => vec!['\\', ';'],
            '=' => vec!['\\', '='],
            c => vec![c],
        })
        .collect()
}

/// Quote a path for command line if needed
pub fn quote_path(path: &Path) -> String {
    let s = path.to_string_lossy();

    // Check if quoting is needed
    if s.contains(' ') || s.contains('\'') || s.contains('"') || s.contains('\\') {
        // Use single quotes and escape any single quotes
        format!("'{}'", s.replace('\'', "'\\''"))
    } else {
        s.into_owned()
    }
}

/// Format a duration for display (human-readable)
pub fn format_duration_human(duration: &std::time::Duration) -> String {
    let total_secs = duration.as_secs();
    let hours = total_secs / 3600;
    let minutes = (total_secs % 3600) / 60;
    let seconds = total_secs % 60;
    let millis = duration.subsec_millis();

    if hours > 0 {
        format!("{}h {}m {}s", hours, minutes, seconds)
    } else if minutes > 0 {
        format!("{}m {}.{:03}s", minutes, seconds, millis)
    } else {
        format!("{}.{:03}s", seconds, millis)
    }
}

/// Parse a frame rate string (e.g., "25", "29.97", "30000/1001")
pub fn parse_framerate(s: &str) -> Result<f64> {
    let s = s.trim();

    // Handle fraction format (e.g., "30000/1001")
    if let Some((num, den)) = s.split_once('/') {
        let numerator: f64 = num.parse()
            .map_err(|_| Error::ParseError(format!("Invalid framerate numerator: {}", num)))?;
        let denominator: f64 = den.parse()
            .map_err(|_| Error::ParseError(format!("Invalid framerate denominator: {}", den)))?;

        if denominator == 0.0 {
            return Err(Error::ParseError("Framerate denominator cannot be zero".to_string()));
        }

        Ok(numerator / denominator)
    } else {
        // Handle decimal format
        s.parse::<f64>()
            .map_err(|_| Error::ParseError(format!("Invalid framerate: {}", s)))
    }
}

/// Get file extension from a path
pub fn get_extension(path: &Path) -> Option<String> {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|s| s.to_lowercase())
}

/// Guess format from file extension
pub fn guess_format_from_extension(path: &Path) -> Option<&'static str> {
    match get_extension(path)?.as_str() {
        // Video formats
        "mp4" => Some("mp4"),
        "m4v" => Some("mp4"),
        "mkv" => Some("matroska"),
        "webm" => Some("webm"),
        "avi" => Some("avi"),
        "mov" => Some("mov"),
        "qt" => Some("mov"),
        "flv" => Some("flv"),
        "wmv" => Some("asf"),
        "mpg" | "mpeg" => Some("mpeg"),
        "ts" | "m2ts" => Some("mpegts"),
        "vob" => Some("mpeg"),
        "3gp" => Some("3gp"),
        "ogv" => Some("ogg"),

        // Audio formats
        "mp3" => Some("mp3"),
        "m4a" => Some("mp4"),
        "aac" => Some("aac"),
        "ogg" | "oga" => Some("ogg"),
        "flac" => Some("flac"),
        "wav" => Some("wav"),
        "opus" => Some("opus"),
        "wma" => Some("asf"),
        "ac3" => Some("ac3"),
        "dts" => Some("dts"),

        // Image formats
        "jpg" | "jpeg" => Some("image2"),
        "png" => Some("image2"),
        "bmp" => Some("image2"),
        "gif" => Some("gif"),
        "webp" => Some("webp"),

        // Subtitle formats
        "srt" => Some("srt"),
        "ass" | "ssa" => Some("ass"),
        "vtt" => Some("webvtt"),
        "sub" => Some("subviewer"),

        _ => None,
    }
}

/// Sanitize a filename for safe file system usage
pub fn sanitize_filename(name: &str) -> String {
    name.chars()
        .map(|c| match c {
            '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|' => '_',
            c if c.is_control() => '_',
            c => c,
        })
        .collect()
}

/// Check if a string looks like a URL
pub fn is_url(s: &str) -> bool {
    s.starts_with("http://") ||
        s.starts_with("https://") ||
        s.starts_with("rtmp://") ||
        s.starts_with("rtmps://") ||
        s.starts_with("rtsp://") ||
        s.starts_with("rtsps://") ||
        s.starts_with("file://") ||
        s.starts_with("udp://") ||
        s.starts_with("tcp://") ||
        s.starts_with("pipe:") ||
        s.contains("://")
}

/// Merge two sets of arguments, with later args overriding earlier ones
pub fn merge_args(base: Vec<String>, overrides: Vec<String>) -> Vec<String> {
    let mut result = base;
    let mut seen_flags = std::collections::HashSet::new();

    // Track which flags take values
    let value_flags: std::collections::HashSet<&str> = [
        "-i", "-f", "-c", "-codec", "-vf", "-af", "-s", "-r", "-b", "-aspect",
        "-t", "-ss", "-to", "-fs", "-preset", "-crf", "-qp", "-profile", "-level",
        "-pix_fmt", "-ar", "-ac", "-ab", "-map", "-metadata", "-filter_complex",
    ].iter().cloned().collect();

    // Process overrides
    let mut i = 0;
    while i < overrides.len() {
        let flag = &overrides[i];

        if value_flags.contains(flag.as_str()) && i + 1 < overrides.len() {
            // Flag with value
            if !seen_flags.contains(flag) {
                result.push(flag.clone());
                result.push(overrides[i + 1].clone());
                seen_flags.insert(flag.clone());
            }
            i += 2;
        } else {
            // Standalone flag
            if !seen_flags.contains(flag) {
                result.push(flag.clone());
                seen_flags.insert(flag.clone());
            }
            i += 1;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_bitrate() {
        assert_eq!(parse_bitrate("128k").unwrap(), 128_000);
        assert_eq!(parse_bitrate("5M").unwrap(), 5_000_000);
        assert_eq!(parse_bitrate("1.5m").unwrap(), 1_500_000);
        assert_eq!(parse_bitrate("1000").unwrap(), 1000);
        assert_eq!(parse_bitrate("2.5G").unwrap(), 2_500_000_000);
    }

    #[test]
    fn test_parse_resolution() {
        assert_eq!(parse_resolution("1920x1080").unwrap(), (1920, 1080));
        assert_eq!(parse_resolution("1280X720").unwrap(), (1280, 720));
        assert_eq!(parse_resolution(" 640x480 ").unwrap(), (640, 480));
    }

    #[test]
    fn test_parse_framerate() {
        assert_eq!(parse_framerate("25").unwrap(), 25.0);
        assert_eq!(parse_framerate("29.97").unwrap(), 29.97);
        assert_eq!(parse_framerate("30000/1001").unwrap(), 29.97002997002997);
        assert_eq!(parse_framerate("24").unwrap(), 24.0);
    }

    #[test]
    fn test_escape_filter_string() {
        assert_eq!(escape_filter_string("text"), "text");
        assert_eq!(escape_filter_string("text:with:colons"), "text\\:with\\:colons");
        assert_eq!(escape_filter_string("text[with]brackets"), "text\\[with\\]brackets");
        assert_eq!(escape_filter_string("text='value'"), "text\\=\\'value\\'");
    }

    #[test]
    fn test_sanitize_filename() {
        assert_eq!(sanitize_filename("normal_file.mp4"), "normal_file.mp4");
        assert_eq!(sanitize_filename("file:with*invalid?chars.mp4"), "file_with_invalid_chars.mp4");
        assert_eq!(sanitize_filename("path/to/file.mp4"), "path_to_file.mp4");
    }

    #[test]
    fn test_is_url() {
        assert!(is_url("https://example.com/video.mp4"));
        assert!(is_url("rtmp://server/live/stream"));
        assert!(is_url("file:///path/to/file.mp4"));
        assert!(!is_url("/path/to/file.mp4"));
        assert!(!is_url("C:\\path\\to\\file.mp4"));
    }

    #[test]
    fn test_guess_format() {
        assert_eq!(guess_format_from_extension(Path::new("video.mp4")), Some("mp4"));
        assert_eq!(guess_format_from_extension(Path::new("audio.mp3")), Some("mp3"));
        assert_eq!(guess_format_from_extension(Path::new("video.mkv")), Some("matroska"));
        assert_eq!(guess_format_from_extension(Path::new("image.jpg")), Some("image2"));
    }
}