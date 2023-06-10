use clap::Parser;
use std::path;

#[derive(Parser, Debug, Default)]
#[clap(author = "Dylan Morrison", version)]
/// ttrim - Trim video files directly in the terminal
pub struct Args {
    /// The video file to be trimmed
    pub target_file: path::PathBuf,
    #[clap(short, long = "start")]
    /// The desired starting point
    pub start_timestamp: Option<String>,
    #[clap(short, long = "end")]
    /// The desired end point
    pub end_timestamp: Option<String>,
    #[clap(short, long)]
    /// The output location
    pub output: Option<path::PathBuf>,
}

// TODO Check ffmpeg and ffprobe are installed
