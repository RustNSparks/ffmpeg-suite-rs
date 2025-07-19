use ffmpeg_rs::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    // Simple conversion
    FFmpegBuilder::convert("video.mp4", "output.webm")
        .run()
        .await?;

    // Complex transcoding with filters
    FFmpegBuilder::new()?
        .input(Input::new("video.mp4"))
        .output(
            Output::new("output.mp4")
                .video_codec_opts(presets::h264::youtube_1080p())
                .audio_codec(Codec::aac())
                .metadata("title", "My Video")
        )
        .video_filter(VideoFilter::scale(1920, 1080))
        .on_progress(|p| println!("Progress: {p:?}"))
        .run()
        .await?;

    Ok(())
}