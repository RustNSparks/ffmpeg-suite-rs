# FFmpeg Suite Rust Wrappers

Safe, idiomatic, and performant Rust wrappers for FFmpeg, FFprobe, and FFplay.

## Overview

This workspace provides three separate crates that wrap the FFmpeg suite of tools:

- **`ffmpeg-rs`** - Video/audio transcoding, filtering, and manipulation
- **`ffprobe-rs`** - Media file inspection and metadata extraction
- **`ffplay-rs`** - Media playback with various display options
- **`ffmpeg-common`** - Shared types and utilities

## Features

- **Type-safe** - Strongly typed APIs prevent common mistakes
- **Async** - Built on Tokio for non-blocking execution
- **Zero-copy** - Efficient handling of large files
- **Cross-platform** - Works on Linux, macOS, and Windows
- **Progress tracking** - Real-time progress updates for long operations
- **Hardware acceleration** - Support for VAAPI, NVENC, QSV, etc.
- **Comprehensive** - Covers most FFmpeg functionality

## Installation

Add the crates you need to your `Cargo.toml`:

```toml
[dependencies]
ffmpeg-rs = "0.1"
ffprobe-rs = "0.1"
ffplay-rs = "0.1"
tokio = { version = "1", features = ["full"] }
```

## Quick Start

### FFmpeg - Transcoding

```rust
use ffmpeg_rs::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    // Simple conversion
    FFmpegBuilder::convert("input.mp4", "output.webm")
        .run()
        .await?;

    // Complex transcoding with filters
    FFmpegBuilder::new()?
        .input(Input::new("input.mp4").seek(Duration::from_secs(10)))
        .output(
            Output::new("output.mp4")
                .video_codec_opts(presets::h264::youtube_1080p())
                .audio_codec(Codec::aac())
                .metadata("title", "My Video")
        )
        .video_filter(VideoFilter::scale(1920, 1080))
        .on_progress(|p| println!("Progress: {:?}", p))
        .run()
        .await?;

    Ok(())
}
```

### FFprobe - Media Inspection

```rust
use ffprobe_rs::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    // Probe media file
    let info = FFprobeBuilder::probe("video.mp4").run().await?;
    
    // Access format info
    if let Some(format) = &info.format {
        println!("Duration: {} seconds", info.duration().unwrap_or(0.0));
        println!("Bitrate: {} bps", format.bit_rate.as_deref().unwrap_or("unknown"));
    }
    
    // Access stream info
    for stream in &info.streams {
        match stream.codec_type.as_deref() {
            Some("video") => {
                println!("Video: {}x{} @ {} fps",
                    stream.width.unwrap_or(0),
                    stream.height.unwrap_or(0),
                    stream.frame_rate().unwrap_or(0.0)
                );
            }
            Some("audio") => {
                println!("Audio: {} Hz, {} channels",
                    stream.sample_rate.as_deref().unwrap_or("?"),
                    stream.channels.unwrap_or(0)
                );
            }
            _ => {}
        }
    }
    
    Ok(())
}
```

### FFplay - Media Playback

```rust
use ffplay_rs::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    // Simple playback
    let mut player = FFplayBuilder::play("video.mp4").spawn().await?;
    player.wait().await?;

    // Advanced playback with options
    let mut player = FFplayBuilder::new()?
        .input("video.mp4")
        .size(1280, 720)
        .window_title("My Player")
        .seek(Duration::from_secs(30))
        .volume(75)
        .spawn()
        .await?;

    // Stop after 10 seconds
    tokio::time::sleep(std::time::Duration::from_secs(10)).await;
    player.kill().await?;

    Ok(())
}
```

## Advanced Examples

### Batch Processing

```rust
use ffmpeg_rs::prelude::*;
use futures::future::try_join_all;

async fn batch_convert(files: Vec<&str>) -> Result<()> {
    let tasks = files.into_iter().map(|file| {
        let output = file.replace(".mp4", ".webm");
        
        FFmpegBuilder::convert(file, output)
            .video_codec_opts(presets::vp9::youtube())
            .run()
    });
    
    try_join_all(tasks).await?;
    Ok(())
}
```

### Live Streaming

```rust
use ffmpeg_rs::prelude::*;

async fn stream_to_rtmp() -> Result<()> {
    FFmpegBuilder::new()?
        .input(DeviceInput::screen_capture().into_input())
        .input(DeviceInput::webcam("/dev/video0").into_input())
        .output(
            Output::new("rtmp://live.example.com/stream/key")
                .for_streaming()
                .video_codec_opts(presets::h264::streaming())
                .audio_codec_opts(presets::audio::aac_standard())
        )
        .filter_complex("[0:v][1:v]overlay=W-w-10:10")
        .run()
        .await?;
    
    Ok(())
}
```

### Extract Frames

```rust
use ffmpeg_rs::prelude::*;

async fn extract_thumbnails() -> Result<()> {
    FFmpegBuilder::new()?
        .input("video.mp4")
        .output(
            ImageSequenceOutput::new("thumb_%04d.jpg")
                .quality(2)
                .framerate(1.0)
                .into_output()
        )
        .video_filter(VideoFilter::scale(320, -1))
        .run()
        .await?;
    
    Ok(())
}
```

### Hardware Acceleration

```rust
use ffmpeg_rs::prelude::*;
use ffmpeg_rs::codec::hardware;

async fn transcode_with_gpu() -> Result<()> {
    FFmpegBuilder::new()?
        .input(Input::new("input.mp4").hwaccel_device("0"))
        .output(
            Output::new("output.mp4")
                .video_codec_opts(hardware::nvenc_h264())
        )
        .hwaccel("cuda")
        .run()
        .await?;
    
    Ok(())
}
```

## Architecture

The crates follow a builder pattern for constructing commands:

```
FFmpegBuilder/FFprobeBuilder/FFplayBuilder
    ├── Input specifications
    ├── Output specifications  
    ├── Filters and processing
    ├── Global options
    └── Execution
```

Key design principles:

- **Type safety** - Invalid combinations are compile-time errors
- **Ergonomics** - Common tasks are simple, complex tasks are possible
- **Performance** - Zero-copy where possible, efficient command building
- **Flexibility** - Raw arguments can be added for unsupported features

## Error Handling

All operations return `Result<T, Error>` with detailed error information:

```rust
use ffmpeg_common::{Error, ResultExt};

match FFmpegBuilder::convert("input.mp4", "output.mp4").run().await {
    Ok(_) => println!("Success!"),
    Err(Error::ExecutableNotFound(name)) => {
        eprintln!("{} not found in PATH", name);
    }
    Err(Error::ProcessFailed { message, stderr, .. }) => {
        eprintln!("Process failed: {}", message);
        if let Some(stderr) = stderr {
            eprintln!("stderr: {}", stderr);
        }
    }
    Err(e) => eprintln!("Error: {}", e),
}
```

## Testing

Run the test suite:

```bash
cargo test --workspace
```

Run with example files:

```bash
cargo run --example transcode -- input.mp4 output.webm
cargo run --example probe -- video.mp4
cargo run --example play -- video.mp4
```

## Safety

These wrappers are designed with safety in mind:

- Path validation prevents command injection
- Proper escaping of filter strings
- Resource cleanup on drop
- Timeout support for all operations
- Graceful process termination

## Platform Support

- **Linux**: Full support including hardware acceleration (VAAPI, VDPAU)
- **macOS**: Full support including VideoToolbox acceleration
- **Windows**: Full support including DXVA2/D3D11VA acceleration

## Contributing

Contributions are welcome! Please ensure:

- Code follows Rust idioms and passes `cargo clippy`
- New features include tests and documentation
- Public APIs have doc comments with examples
- Changes maintain backward compatibility

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.

## Acknowledgments

Built on top of the excellent FFmpeg project. These wrappers would not be possible without the hard work of the FFmpeg community.