use ffmpeg_common::{Error, Result};

use crate::format::OutputFormat;
pub(crate) use crate::types::ProbeResult;

mod json;

/// Parse FFprobe output based on format
pub fn parse_output(output: &str, format: OutputFormat) -> Result<ProbeResult> {
    match format {
        OutputFormat::Json => json::parse_json(output),
        _ => Err(Error::Unsupported(format!(
            "Parser for {} format not implemented",
            format
        ))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_json() {
        let json_output = r#"{
            "format": {
                "filename": "test.mp4",
                "nb_streams": 2,
                "format_name": "mov,mp4,m4a,3gp,3g2,mj2",
                "format_long_name": "QuickTime / MOV",
                "duration": "10.000000",
                "size": "1048576",
                "bit_rate": "838860"
            },
            "streams": [
                {
                    "index": 0,
                    "codec_name": "h264",
                    "codec_type": "video",
                    "width": 1920,
                    "height": 1080
                },
                {
                    "index": 1,
                    "codec_name": "aac",
                    "codec_type": "audio",
                    "sample_rate": "48000",
                    "channels": 2
                }
            ]
        }"#;

        let result = parse_output(json_output, OutputFormat::Json).unwrap();

        assert!(result.format.is_some());
        let format = result.format.unwrap();
        assert_eq!(format.filename, Some("test.mp4".to_string()));
        assert_eq!(format.nb_streams, Some(2));
        assert_eq!(format.duration, Some("10.000000".to_string()));

        assert_eq!(result.streams.len(), 2);
        assert_eq!(result.streams[0].codec_name, Some("h264".to_string()));
        assert_eq!(result.streams[1].codec_name, Some("aac".to_string()));
    }

    #[test]
    fn test_unsupported_format() {
        let result = parse_output("", OutputFormat::Xml);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), Error::Unsupported(_)));
    }
}