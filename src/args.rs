use crate::error::Error;
use crate::timestamp::Timestamp;

use clap::Parser;
use std::path;

#[derive(Parser, Debug, Default)]
#[clap(author = "Dylan Morrison", version)]
/// ttrim - Trim video files directly in the terminal.
pub struct Args {
    /// Path to the video file to be trimmed.
    pub target_file: path::PathBuf,
    #[clap(short, long = "start")]
    /// The desired starting point of the trimmed video.
    /// Can be provided as a timestamp in the format `HH:MM:SS`, `MM:SS`, a percentage `DD%` or as a number of seconds.
    pub start_timestamp: Option<String>,
    #[clap(short, long = "end")]
    /// The desired end point of the trimmed video.
    /// Can be provided as a timestamp in the format `HH:MM:SS`, `MM:SS`, a percentage `DD%` or as a number of seconds.
    pub end_timestamp: Option<String>,
    #[clap(short, long)]
    /// The output location for the trimmed video file.
    /// If not provided, the trimmed video will be saved in the same directory as the original video. Can be a specific file or a directory in which case a filename will be generated.
    pub output: Option<path::PathBuf>,
}

impl Args {
    pub fn get_start_timestamp(&self) -> Result<Timestamp, Error> {
        get_timestamp(self.start_timestamp.clone(), true)
    }

    pub fn get_end_timestamp(&self) -> Result<Timestamp, Error> {
        get_timestamp(self.end_timestamp.clone(), false)
    }
}

fn get_timestamp(arg_timestamp: Option<String>, start: bool) -> Result<Timestamp, Error> {
    if let Some(timestamp) = arg_timestamp {
        Timestamp::parse_timestamp(&timestamp)
    } else {
        if start {
            Ok(Timestamp::Start)
        } else {
            Ok(Timestamp::End)
        }
    }
}
