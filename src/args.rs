use crate::error::Error;
use crate::timestamp::Timestamp;

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
