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