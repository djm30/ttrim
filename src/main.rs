mod args;
mod timestamp;
mod video_utils;

use clap::Parser;

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

    if !args.target_file.exists() {
        panic!("Specified video file doesnt exist");
    }

    if !video_utils::check_valid_file_extension(&args.target_file) {
        panic!("Video file must be an mp4");
    }

    let start_timestamp = get_timestamp(args.start_timestamp, true);
    let mut end_timestamp = get_timestamp(args.end_timestamp, false);

    let duration = video_utils::get_video_length(&args.target_file);

    if end_timestamp.is_before(&start_timestamp, duration) {
        panic!("Please ensure start timestamp is before end timestamp");
    }

    if end_timestamp.to_seconds(duration) > duration {
        end_timestamp = Timestamp::End;
    }

    // Need to validate output path
    // Either by getting the option passed in, validatiing it or generating one
    let output_path = if let Some(mut path) = args.output {
        if path.exists() {
            panic!("Output path already exists");
        }
        if !video_utils::check_valid_file_extension(&path) {
            panic!("Output path must be an mp4");
        }
        path
    } else {
        video_utils::generate_filename(&args.target_file)
    };

    println!("Output Path: {:?}", output_path);

    video_utils::trim_video(
        start_timestamp.to_seconds(duration),
        end_timestamp.to_seconds(duration),
        &args.target_file,
        &output_path,
    );

    println!("Done")
}
