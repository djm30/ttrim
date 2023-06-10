use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum Error {
    #[error("{0}")]
    PercentageOutOfRange(String),
    #[error("{0}")]
    InvalidTime(String),
    #[error("Provided timestamp did not match the format (HH:MM:SS) or (NN) for number of seconds or (NN%) for percentage")]
    NoTimestampMatch,
    #[error("Provided file has an invalid or unknown video file extension")]
    InvalidExtension,
    #[error("Provided video file does not exist")]
    InputFileDoesntExist,
    #[error("Provided end timestamp is before start timestamp")]
    EndTimestampBeforeStartTimestamp,
    #[error("Output file already exists")]
    OutputFileExists,
    #[error("An error occurred while running ffmpeg")]
    FfpmegError,
    #[error("Ffmpeg is not installed, please install it and try again")]
    FfpmegNotInstalled,
    #[error("Ffprobe is not installed, please install it and try again")]
    FfprobeNotInstalled,
    #[error("Invalid path: {0}")]
    InvalidPath(String),
    #[error("Invalid data: {0}")]
    InvalidData(String),
}
