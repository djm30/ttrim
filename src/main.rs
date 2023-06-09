mod args;
mod timestamp;

use clap::Parser;
use mp4;

use args::Args;
use timestamp::{Timestamp, TimestampError};

fn get_timestamp(arg_timestamp: Option<String>, start: bool) -> Timestamp {
    if let Some(timestamp) = arg_timestamp {
        match Timestamp::parse_timestamp(&timestamp) {
            Ok(timestamp) => timestamp,
            Err(error) => match error {
                TimestampError::NoMatch => panic!("Please enter a valid timestamp format"),
                TimestampError::InvalidTime(msg) => panic!("{}", msg),
                TimestampError::PercentageOutOfRange(msg) => panic!("{}", msg),
            },
        }
    } else {
        if start {
            Timestamp::Start
        } else {
            Timestamp::End
        }
    }
}

fn main() {
    let args = Args::parse();
    println!("{:?}", args);

    // Steps before video trimming can proceed
    // Verify video file exists
    // Verify both start and end timestamps
    // Verify video input is an mp4
    // Verify output path doesnt already exist

    let start_timestamp = get_timestamp(args.start_timestamp, true);
    let end_timestamp = get_timestamp(args.end_timestamp, false);
}
