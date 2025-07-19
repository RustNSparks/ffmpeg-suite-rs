use ffmpeg_common::{Error, Result};
use serde_json;

use crate::types::ProbeResult;

/// Parse JSON output from FFprobe
pub fn parse_json(output: &str) -> Result<ProbeResult> {
    serde_json::from_str(output)
        .map_err(|e| Error::ParseError(format!("Failed to parse JSON: {}", e)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_complete_json() {
        let json = r#"{
            "format": {
                "filename": "/path/to/video.mp4",
                "nb_streams": 2,
                "nb_programs": 0,
                "format_name": "mov,mp4,m4a,3gp,3g2,mj2",
                "format_long_name": "QuickTime / MOV",
                "start_time": "0.000000",
                "duration": "596.501333",
                "size": "968413434",
                "bit_rate": "12999521",
                "probe_score": 100,
                "tags": {
                    "major_brand": "mp42",
                    "minor_version": "0",
                    "compatible_brands": "isommp42",
                    "creation_time": "2024-01-15T10:30:00.000000Z",
                    "title": "Sample Video",
                    "encoder": "FFmpeg"
                }
            },
            "streams": [
                {
                    "index": 0,
                    "codec_name": "h264",
                    "codec_long_name": "H.264 / AVC / MPEG-4 AVC / MPEG-4 part 10",
                    "profile": "High",
                    "codec_type": "video",
                    "codec_tag_string": "avc1",
                    "codec_tag": "0x31637661",
                    "width": 1920,
                    "height": 1080,
                    "coded_width": 1920,
                    "coded_height": 1088,
                    "has_b_frames": 2,
                    "sample_aspect_ratio": "1:1",
                    "display_aspect_ratio": "16:9",
                    "pix_fmt": "yuv420p",
                    "level": 40,
                    "color_range": "tv",
                    "color_space": "bt709",
                    "color_transfer": "bt709",
                    "color_primaries": "bt709",
                    "chroma_location": "left",
                    "field_order": "progressive",
                    "refs": 1,
                    "is_avc": "true",
                    "nal_length_size": "4",
                    "r_frame_rate": "30/1",
                    "avg_frame_rate": "30/1",
                    "time_base": "1/15360",
                    "start_pts": 0,
                    "start_time": "0.000000",
                    "duration_ts": 9162752,
                    "duration": "596.501333",
                    "bit_rate": "12737849",
                    "bits_per_raw_sample": "8",
                    "nb_frames": "17895",
                    "disposition": {
                        "default": 1,
                        "dub": 0,
                        "original": 0,
                        "comment": 0,
                        "lyrics": 0,
                        "karaoke": 0,
                        "forced": 0,
                        "hearing_impaired": 0,
                        "visual_impaired": 0,
                        "clean_effects": 0,
                        "attached_pic": 0,
                        "timed_thumbnails": 0
                    },
                    "tags": {
                        "language": "und",
                        "handler_name": "VideoHandler",
                        "vendor_id": "[0][0][0][0]"
                    }
                },
                {
                    "index": 1,
                    "codec_name": "aac",
                    "codec_long_name": "AAC (Advanced Audio Coding)",
                    "profile": "LC",
                    "codec_type": "audio",
                    "codec_tag_string": "mp4a",
                    "codec_tag": "0x6134706d",
                    "sample_fmt": "fltp",
                    "sample_rate": "48000",
                    "channels": 2,
                    "channel_layout": "stereo",
                    "bits_per_sample": 0,
                    "r_frame_rate": "0/0",
                    "avg_frame_rate": "0/0",
                    "time_base": "1/48000",
                    "start_pts": 0,
                    "start_time": "0.000000",
                    "duration_ts": 28632064,
                    "duration": "596.501333",
                    "bit_rate": "253916",
                    "nb_frames": "27961",
                    "disposition": {
                        "default": 1,
                        "dub": 0,
                        "original": 0,
                        "comment": 0,
                        "lyrics": 0,
                        "karaoke": 0,
                        "forced": 0,
                        "hearing_impaired": 0,
                        "visual_impaired": 0,
                        "clean_effects": 0,
                        "attached_pic": 0,
                        "timed_thumbnails": 0
                    },
                    "tags": {
                        "language": "eng",
                        "handler_name": "SoundHandler",
                        "vendor_id": "[0][0][0][0]"
                    }
                }
            ],
            "chapters": [
                {
                    "id": 0,
                    "time_base": "1/1000",
                    "start": 0,
                    "start_time": "0.000000",
                    "end": 120000,
                    "end_time": "120.000000",
                    "tags": {
                        "title": "Chapter 1"
                    }
                },
                {
                    "id": 1,
                    "time_base": "1/1000",
                    "start": 120000,
                    "start_time": "120.000000",
                    "end": 596501,
                    "end_time": "596.501000",
                    "tags": {
                        "title": "Chapter 2"
                    }
                }
            ]
        }"#;

        let result = parse_json(json).unwrap();

        // Test format
        assert!(result.format.is_some());
        let format = result.format.as_ref().unwrap();
        assert_eq!(format.filename, Some("/path/to/video.mp4".to_string()));
        assert_eq!(format.nb_streams, Some(2));
        assert_eq!(format.duration, Some("596.501333".to_string()));
        assert_eq!(format.bit_rate, Some("12999521".to_string()));
        assert_eq!(format.tags.get("title"), Some(&"Sample Video".to_string()));

        // Test streams
        assert_eq!(result.streams.len(), 2);

        let video_stream = &result.streams[0];
        assert_eq!(video_stream.codec_name, Some("h264".to_string()));
        assert_eq!(video_stream.codec_type, Some("video".to_string()));
        assert_eq!(video_stream.width, Some(1920));
        assert_eq!(video_stream.height, Some(1080));
        assert_eq!(video_stream.pix_fmt, Some("yuv420p".to_string()));
        assert_eq!(video_stream.profile, Some("High".to_string()));
        assert_eq!(video_stream.level, Some(40));

        let audio_stream = &result.streams[1];
        assert_eq!(audio_stream.codec_name, Some("aac".to_string()));
        assert_eq!(audio_stream.codec_type, Some("audio".to_string()));
        assert_eq!(audio_stream.sample_rate, Some("48000".to_string()));
        assert_eq!(audio_stream.channels, Some(2));
        assert_eq!(audio_stream.channel_layout, Some("stereo".to_string()));
        assert_eq!(audio_stream.tags.get("language"), Some(&"eng".to_string()));

        // Test chapters
        assert_eq!(result.chapters.len(), 2);
        assert_eq!(result.chapters[0].tags.get("title"), Some(&"Chapter 1".to_string()));
        assert_eq!(result.chapters[1].tags.get("title"), Some(&"Chapter 2".to_string()));
    }

    #[test]
    fn test_parse_minimal_json() {
        let json = r#"{
            "streams": [
                {
                    "index": 0,
                    "codec_type": "video"
                }
            ]
        }"#;

        let result = parse_json(json).unwrap();
        assert!(result.format.is_none());
        assert_eq!(result.streams.len(), 1);
        assert_eq!(result.streams[0].index, 0);
        assert_eq!(result.streams[0].codec_type, Some("video".to_string()));
    }

    #[test]
    fn test_parse_error_json() {
        let json = r#"{
            "error": {
                "code": -2,
                "string": "No such file or directory"
            }
        }"#;

        let result = parse_json(json).unwrap();
        assert!(result.error.is_some());
        let error = result.error.unwrap();
        assert_eq!(error.code, Some(-2));
        assert_eq!(error.string, Some("No such file or directory".to_string()));
    }

    #[test]
    fn test_parse_invalid_json() {
        let invalid_json = "{ invalid json }";
        let result = parse_json(invalid_json);
        assert!(result.is_err());
        match result.unwrap_err() {
            Error::ParseError(msg) => assert!(msg.contains("Failed to parse JSON")),
            _ => panic!("Expected ParseError"),
        }
    }

    #[test]
    fn test_parse_packets_frames() {
        let json = r#"{
            "packets": [
                {
                    "codec_type": "video",
                    "stream_index": 0,
                    "pts": 0,
                    "pts_time": "0.000000",
                    "dts": 0,
                    "dts_time": "0.000000",
                    "duration": 512,
                    "duration_time": "0.033333",
                    "size": "24215",
                    "pos": "48",
                    "flags": "K_"
                }
            ],
            "frames": [
                {
                    "media_type": "video",
                    "stream_index": 0,
                    "key_frame": 1,
                    "pts": 0,
                    "pts_time": "0.000000",
                    "pkt_dts": 0,
                    "pkt_dts_time": "0.000000",
                    "best_effort_timestamp": 0,
                    "best_effort_timestamp_time": "0.000000",
                    "pkt_duration": 512,
                    "pkt_duration_time": "0.033333",
                    "pkt_pos": "48",
                    "pkt_size": "24215",
                    "width": 1920,
                    "height": 1080,
                    "pix_fmt": "yuv420p",
                    "pict_type": "I"
                }
            ]
        }"#;

        let result = parse_json(json).unwrap();

        assert_eq!(result.packets.len(), 1);
        let packet = &result.packets[0];
        assert_eq!(packet.stream_index, 0);
        assert_eq!(packet.pts, Some(0));
        assert_eq!(packet.size, Some("24215".to_string()));

        assert_eq!(result.frames.len(), 1);
        let frame = &result.frames[0];
        assert_eq!(frame.stream_index, 0);
        assert_eq!(frame.key_frame, Some(1));
        assert_eq!(frame.width, Some(1920));
        assert_eq!(frame.height, Some(1080));
        assert_eq!(frame.pict_type, Some("I".to_string()));
    }

    #[test]
    fn test_stream_helper_methods() {
        let json = r#"{
            "streams": [
                {
                    "index": 0,
                    "codec_type": "video",
                    "codec_name": "h264",
                    "width": 1920,
                    "height": 1080,
                    "r_frame_rate": "30000/1001",
                    "bit_rate": "5000000",
                    "duration": "120.5",
                    "tags": {
                        "language": "eng",
                        "title": "Main Video"
                    }
                },
                {
                    "index": 1,
                    "codec_type": "audio",
                    "codec_name": "aac",
                    "sample_rate": "48000",
                    "channels": 2
                },
                {
                    "index": 2,
                    "codec_type": "subtitle",
                    "codec_name": "subrip"
                }
            ]
        }"#;

        let result = parse_json(json).unwrap();

        // Test stream type filters
        let video_streams = result.video_streams();
        assert_eq!(video_streams.len(), 1);
        assert_eq!(video_streams[0].index, 0);

        let audio_streams = result.audio_streams();
        assert_eq!(audio_streams.len(), 1);
        assert_eq!(audio_streams[0].index, 1);

        let subtitle_streams = result.subtitle_streams();
        assert_eq!(subtitle_streams.len(), 1);
        assert_eq!(subtitle_streams[0].index, 2);

        // Test primary stream getters
        let primary_video = result.primary_video_stream().unwrap();
        assert_eq!(primary_video.codec_name, Some("h264".to_string()));

        let primary_audio = result.primary_audio_stream().unwrap();
        assert_eq!(primary_audio.codec_name, Some("aac".to_string()));

        // Test stream info helper methods
        let video = &result.streams[0];
        assert!(video.is_video());
        assert!(!video.is_audio());
        assert_eq!(video.language(), Some("eng"));
        assert_eq!(video.title(), Some("Main Video"));
        assert_eq!(video.resolution(), Some((1920, 1080)));
        assert_eq!(video.frame_rate(), Some(29.970_029_970_029_97));
        assert_eq!(video.bit_rate_bps(), Some(5_000_000));
        assert_eq!(video.duration_seconds(), Some(120.5));

        let audio = &result.streams[1];
        assert!(audio.is_audio());
        assert_eq!(audio.sample_rate_hz(), Some(48000));
    }
}