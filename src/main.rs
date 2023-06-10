mod args;
mod error;
mod timestamp;
mod video_utils;

use clap::Parser;

use args::Args;
use error::Error;
use timestamp::Timestamp;
use video_utils::PathType;

fn main() -> Result<(), Error> {
    let args = Args::parse();

    let target_file = args.target_file.clone();

    if !target_file.exists() {
        Err(Error::InputFileDoesntExist)?
    }

    if !video_utils::check_valid_file_extension(&target_file) {
        Err(Error::InvalidExtension)?
    }

    let start_timestamp = args.get_start_timestamp()?;
    let mut end_timestamp = args.get_end_timestamp()?;

    let duration = video_utils::get_video_length(&target_file)?;

    if end_timestamp.is_before(&start_timestamp, duration) {
        Err(Error::EndTimestampBeforeStartTimestamp)?
    }

    if end_timestamp.to_seconds(duration) > duration {
        end_timestamp = Timestamp::End;
    }

    let output_path = match args.output {
        Some(mut path) => {
            if path.is_dir() {
                path.push(video_utils::generate_output_filename(
                    &target_file,
                    PathType::FileOnly,
                ))
            }
            if path.exists() {
                return Err(Error::OutputFileExists);
            }
            if !video_utils::check_valid_file_extension(&path) {
                return Err(Error::InvalidExtension);
            }
            path
        }
        None => video_utils::generate_output_filename(&target_file, PathType::Relative),
    };

    video_utils::trim_video(
        start_timestamp.to_seconds(duration),
        end_timestamp.to_seconds(duration),
        &target_file,
        &output_path,
    )?;

    println!(
        "Successfully trimmed video. Output file: {}",
        output_path.display()
    );

    Ok(())
}
