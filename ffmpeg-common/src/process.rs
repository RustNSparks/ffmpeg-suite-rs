use std::path::{Path, PathBuf};
use std::process::Stdio;
use std::time::Duration;
use tokio::io::{AsyncBufReadExt, AsyncRead, AsyncReadExt, BufReader};
use tokio::process::{Child, Command};
use tokio::time::timeout;
use tracing::{debug, trace};
use which::which;

use crate::error::{Error, Result};

/// Find the path to an FFmpeg executable
pub fn find_executable(name: &str) -> Result<PathBuf> {
    which(name).map_err(|_| Error::ExecutableNotFound(name.to_string()))
}

/// Process execution configuration
#[derive(Debug, Clone)]
pub struct ProcessConfig {
    /// Executable path
    pub executable: PathBuf,
    /// Working directory
    pub working_dir: Option<PathBuf>,
    /// Environment variables
    pub env: Vec<(String, String)>,
    /// Timeout for the process
    pub timeout: Option<Duration>,
    /// Whether to capture stdout
    pub capture_stdout: bool,
    /// Whether to capture stderr
    pub capture_stderr: bool,
    /// Whether to pipe stdin
    pub pipe_stdin: bool,
}

impl ProcessConfig {
    /// Create a new process configuration
    pub fn new(executable: impl Into<PathBuf>) -> Self {
        Self {
            executable: executable.into(),
            working_dir: None,
            env: Vec::new(),
            timeout: None,
            capture_stdout: true,
            capture_stderr: true,
            pipe_stdin: false,
        }
    }

    /// Set working directory
    pub fn working_dir(mut self, dir: impl Into<PathBuf>) -> Self {
        self.working_dir = Some(dir.into());
        self
    }

    /// Add environment variable
    pub fn env(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.env.push((key.into(), value.into()));
        self
    }

    /// Set timeout
    pub fn timeout(mut self, duration: Duration) -> Self {
        self.timeout = Some(duration);
        self
    }

    /// Set stdout capture
    pub fn capture_stdout(mut self, capture: bool) -> Self {
        self.capture_stdout = capture;
        self
    }

    /// Set stderr capture
    pub fn capture_stderr(mut self, capture: bool) -> Self {
        self.capture_stderr = capture;
        self
    }

    /// Set stdin piping
    pub fn pipe_stdin(mut self, pipe: bool) -> Self {
        self.pipe_stdin = pipe;
        self
    }
}

/// Process handle for running FFmpeg processes
pub struct Process {
    child: Child,
    config: ProcessConfig,
}

impl Process {
    /// Spawn a new process with arguments
    pub async fn spawn(config: ProcessConfig, args: Vec<String>) -> Result<Self> {
        debug!("Spawning process: {} {:?}", config.executable.display(), args);

        let mut cmd = Command::new(&config.executable);

        // Add arguments
        for arg in &args {
            cmd.arg(arg);
        }

        // Set working directory
        if let Some(ref dir) = config.working_dir {
            cmd.current_dir(dir);
        }

        // Set environment variables
        for (key, value) in &config.env {
            cmd.env(key, value);
        }

        // Configure stdio
        cmd.stdin(if config.pipe_stdin {
            Stdio::piped()
        } else {
            Stdio::null()
        });

        cmd.stdout(if config.capture_stdout {
            Stdio::piped()
        } else {
            Stdio::null()
        });

        cmd.stderr(if config.capture_stderr {
            Stdio::piped()
        } else {
            Stdio::null()
        });

        // Kill on drop
        cmd.kill_on_drop(true);

        let child = cmd.spawn().map_err(Error::Io)?;

        Ok(Self { child, config })
    }

    /// Wait for the process to complete
    pub async fn wait(mut self) -> Result<ProcessOutput> {
        // This async block will capture the process output.
        // We explicitly map `std::io::Error` to our custom `Error::Io` variant
        // to resolve the compiler's type inference ambiguity.
        let wait_future = async {
            let status = self.child.wait().await.map_err(Error::Io)?;

            let stdout = if self.config.capture_stdout {
                if let Some(mut stdout) = self.child.stdout.take() {
                    let mut buf = Vec::new();
                    stdout.read_to_end(&mut buf).await.map_err(Error::Io)?;
                    Some(buf)
                } else {
                    None
                }
            } else {
                None
            };

            let stderr = if self.config.capture_stderr {
                if let Some(mut stderr) = self.child.stderr.take() {
                    let mut buf = Vec::new();
                    stderr.read_to_end(&mut buf).await.map_err(Error::Io)?;
                    Some(buf)
                } else {
                    None
                }
            } else {
                None
            };

            Ok(ProcessOutput {
                status,
                stdout,
                stderr,
            })
        };

        if let Some(timeout_duration) = self.config.timeout {
            match timeout(timeout_duration, wait_future).await {
                // The future completed without timing out. `result` is the `Result` from our future.
                Ok(result) => result,
                // The future timed out.
                Err(_) => {
                    let _ = self.child.kill().await;
                    Err(Error::Timeout(timeout_duration))
                }
            }
        } else {
            // No timeout configured, just await the future.
            wait_future.await
        }
    }

    /// Get a handle to stdin
    pub fn stdin(&mut self) -> Option<tokio::process::ChildStdin> {
        self.child.stdin.take()
    }

    /// Get a handle to stdout
    pub fn stdout(&mut self) -> Option<tokio::process::ChildStdout> {
        self.child.stdout.take()
    }

    /// Get a handle to stderr
    pub fn stderr(&mut self) -> Option<tokio::process::ChildStderr> {
        self.child.stderr.take()
    }

    /// Kill the process
    pub async fn kill(&mut self) -> Result<()> {
        self.child.kill().await.map_err(Error::Io)
    }

    /// Get the process ID
    pub fn id(&self) -> Option<u32> {
        self.child.id()
    }

    /// Try to wait for the process without blocking
    pub fn try_wait(&mut self) -> Result<Option<std::process::ExitStatus>> {
        // Explicitly map the error to avoid ambiguity with the `?` operator.
        self.child.try_wait().map_err(Error::Io)
    }
}

/// Output from a completed process
#[derive(Debug)]
pub struct ProcessOutput {
    /// Exit status
    pub status: std::process::ExitStatus,
    /// Stdout data if captured
    pub stdout: Option<Vec<u8>>,
    /// Stderr data if captured
    pub stderr: Option<Vec<u8>>,
}

impl ProcessOutput {
    /// Check if the process succeeded
    pub fn success(&self) -> bool {
        self.status.success()
    }

    /// Get stdout as string
    pub fn stdout_str(&self) -> Option<String> {
        self.stdout.as_ref().map(|b| String::from_utf8_lossy(b).into_owned())
    }

    /// Get stderr as string
    pub fn stderr_str(&self) -> Option<String> {
        self.stderr.as_ref().map(|b| String::from_utf8_lossy(b).into_owned())
    }

    /// Convert to a Result, treating non-zero exit as error
    pub fn into_result(self) -> Result<Self> {
        if self.success() {
            Ok(self)
        } else {
            Err(Error::process_failed(
                format!("Process exited with status: {}", self.status),
                Some(self.status),
                self.stderr_str(),
            ))
        }
    }
}

/// Progress information from FFmpeg
#[derive(Debug, Clone)]
pub struct Progress {
    /// Current frame number
    pub frame: Option<u64>,
    /// Frames per second
    pub fps: Option<f64>,
    /// Quality factor
    pub q: Option<f64>,
    /// Current size in bytes
    pub size: Option<u64>,
    /// Current time position
    pub time: Option<Duration>,
    /// Bitrate in bits/s
    pub bitrate: Option<f64>,
    /// Processing speed
    pub speed: Option<f64>,
}

impl Progress {
    /// Parse progress from FFmpeg stderr line
    pub fn parse_line(line: &str) -> Option<Self> {
        if !line.contains("frame=") {
            return None;
        }

        let mut progress = Progress {
            frame: None,
            fps: None,
            q: None,
            size: None,
            time: None,
            bitrate: None,
            speed: None,
        };

        // Parse key=value pairs
        for part in line.split_whitespace() {
            if let Some((key, value)) = part.split_once('=') {
                match key {
                    "frame" => progress.frame = value.parse().ok(),
                    "fps" => progress.fps = value.parse().ok(),
                    "q" => progress.q = value.parse().ok(),
                    "size" => {
                        // Remove "kB" suffix and convert to bytes
                        if let Some(kb_str) = value.strip_suffix("kB") {
                            progress.size = kb_str.parse::<u64>().ok().map(|kb| kb * 1024);
                        }
                    }
                    "time" => {
                        // Parse time in HH:MM:SS.MS format
                        if let Ok(duration) = crate::types::Duration::from_ffmpeg_format(value) {
                            progress.time = Some(duration.into());
                        }
                    }
                    "bitrate" => {
                        // Remove "kbits/s" suffix
                        if let Some(kbits_str) = value.strip_suffix("kbits/s") {
                            progress.bitrate = kbits_str.parse::<f64>().ok().map(|kb| kb * 1000.0);
                        }
                    }
                    "speed" => {
                        // Remove "x" suffix
                        if let Some(speed_str) = value.strip_suffix('x') {
                            progress.speed = speed_str.parse().ok();
                        }
                    }
                    _ => {}
                }
            }
        }

        Some(progress)
    }
}

/// Progress callback type
pub type ProgressCallback = Box<dyn Fn(Progress) + Send + Sync>;

/// Stream progress updates from FFmpeg stderr
pub async fn stream_progress<R: AsyncRead + Unpin + Send + 'static>(
    stderr: R,
    mut callback: impl FnMut(Progress) + Send + 'static,
) {
    let reader = BufReader::new(stderr);
    let mut lines = reader.lines();

    while let Ok(Some(line)) = lines.next_line().await {
        trace!("FFmpeg stderr: {}", line);
        if let Some(progress) = Progress::parse_line(&line) {
            callback(progress);
        }
    }
}

/// Command builder with safe argument construction
#[derive(Debug, Clone)]
pub struct CommandBuilder {
    args: Vec<String>,
}

impl CommandBuilder {
    /// Create a new command builder
    pub fn new() -> Self {
        Self { args: Vec::new() }
    }

    /// Add a flag (no value)
    pub fn flag(mut self, flag: impl AsRef<str>) -> Self {
        self.args.push(flag.as_ref().to_string());
        self
    }

    /// Add an option with a value
    pub fn option(mut self, key: impl AsRef<str>, value: impl ToString) -> Self {
        self.args.push(key.as_ref().to_string());
        self.args.push(value.to_string());
        self
    }

    /// Add an option only if the value is Some
    pub fn option_if_some<T: ToString>(self, key: impl AsRef<str>, value: Option<T>) -> Self {
        if let Some(val) = value {
            self.option(key, val)
        } else {
            self
        }
    }

    /// Add a flag only if the condition is true
    pub fn flag_if(self, flag: impl AsRef<str>, condition: bool) -> Self {
        if condition {
            self.flag(flag)
        } else {
            self
        }
    }

    /// Add raw arguments
    pub fn args(mut self, args: impl IntoIterator<Item = impl AsRef<str>>) -> Self {
        for arg in args {
            self.args.push(arg.as_ref().to_string());
        }
        self
    }

    /// Add raw argument
    pub fn arg(mut self, arg: impl AsRef<str>) -> Self {
        self.args.push(arg.as_ref().to_string());
        self
    }

    /// Build into a vector of arguments
    pub fn build(self) -> Vec<String> {
        self.args
    }
}

impl Default for CommandBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Helper to validate paths exist
pub fn validate_input_path(path: &Path) -> Result<()> {
    if !path.exists() {
        return Err(Error::Io(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("Input file not found: {}", path.display()),
        )));
    }
    Ok(())
}

/// Helper to validate output path can be written
pub fn validate_output_path(path: &Path) -> Result<()> {
    if let Some(parent) = path.parent() {
        if !parent.exists() {
            return Err(Error::Io(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("Output directory does not exist: {}", parent.display()),
            )));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_builder() {
        let args = CommandBuilder::new()
            .flag("-y")
            .option("-i", "input.mp4")
            .option_if_some("-ss", Some("00:00:10"))
            .option_if_some("-t", None::<&str>)
            .flag_if("-n", false)
            .arg("output.mp4")
            .build();

        assert_eq!(args, vec!["-y", "-i", "input.mp4", "-ss", "00:00:10", "output.mp4"]);
    }

    #[test]
    fn test_progress_parsing() {
        let line = "frame=  100 fps=25.0 q=28.0 size=    1024kB time=00:00:04.00 bitrate=2097.2kbits/s speed=1.00x";
        let progress = Progress::parse_line(line).unwrap();

        assert_eq!(progress.frame, Some(100));
        assert_eq!(progress.fps, Some(25.0));
        assert_eq!(progress.q, Some(28.0));
        assert_eq!(progress.size, Some(1024 * 1024));
        assert_eq!(progress.time, Some(Duration::from_secs(4)));
        assert_eq!(progress.bitrate, Some(2097200.0));
        assert_eq!(progress.speed, Some(1.0));
    }
}
