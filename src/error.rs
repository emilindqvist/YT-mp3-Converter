use std::io;

#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("yt-dlp not found on PATH — install via `pip install yt-dlp` or see https://github.com/yt-dlp/yt-dlp")]
    YtDlpMissing,

    #[error("ffmpeg not found on PATH — required for audio extraction")]
    FfmpegMissing,

    #[error("download cancelled")]
    Cancelled,

    #[error("yt-dlp failed (exit {code}): {stderr}")]
    YtDlp { code: i32, stderr: String },

    #[error("failed to spawn yt-dlp: {0}")]
    Spawn(#[source] io::Error),

    #[error("io error: {0}")]
    Io(#[from] io::Error),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl AppError {
    pub fn exit_code(&self) -> i32 {
        match self {
            Self::YtDlpMissing => 2,
            Self::FfmpegMissing => 3,
            Self::Cancelled => 130,
            _ => 1,
        }
    }
}
