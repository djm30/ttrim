use crate::error::Error;

use chrono::prelude::*;
use std::path::PathBuf;
use std::process::Command;
use std::str;

fn check_program_installed(program_name: String) -> bool {
    Command::new("which")
        .arg(program_name)
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

// Need to make this handle errors
pub fn get_video_length(path: &PathBuf) -> Result<f64, Error> {
    if !check_program_installed("ffprobe".to_string()) {
        Err(Error::FfprobeNotInstalled)?
    }

    let video_path = path
        .to_str()
        .ok_or(Error::InvalidPath(path.to_string_lossy().into_owned()))?;

    let output = Command::new("ffprobe")
        .arg("-v")
        .arg("error")
        .arg("-show_entries")
        .arg("format=duration")
        .arg("-of")
        .arg("default=noprint_wrappers=1:nokey=1")
        .arg(&video_path)
        .output();

    let output = match output {
        Ok(output) => match output.status.success() {
            true => output,
            false => return Err(Error::FfpmegError)?,
        },
        Err(_) => return Err(Error::FfpmegError)?,
    };

    let duration_str = str::from_utf8(&output.stdout).map_err(|_| {
        Error::InvalidData("Failed to read ffprobe output to a UTF-8 string".to_string())
    })?;

    let duration: f64 = duration_str.trim().parse().map_err(|_| {
        Error::InvalidData(format!(
            "Failed to parse ffprobe output as a float: {}",
            duration_str
        ))
    })?;

    Ok(duration)
}

// Todo: Implement this function
pub fn trim_video(
    start: f64,
    end: f64,
    input_path: &PathBuf,
    output_path: &PathBuf,
) -> Result<(), Error> {
    if !check_program_installed("ffmpeg".to_string()) {
        Err(Error::FfpmegNotInstalled)?
    }

    let output = Command::new("ffmpeg")
        .arg("-i")
        .arg(input_path)
        .arg("-v")
        .arg("error")
        .arg("-ss")
        .arg(format!("{}", start))
        .arg("-to")
        .arg(format!("{}", end))
        .arg("-c")
        .arg("copy")
        .arg(output_path)
        .output();

    match output {
        Ok(output) => match output.status.success() {
            true => output,
            false => return Err(Error::FfpmegError)?,
        },
        Err(_) => return Err(Error::FfpmegError)?,
    };

    Ok(())
}

// TODO: More file formats are probably supported
pub fn check_valid_file_extension(path: &PathBuf) -> bool {
    let valid_extensions = [
        "mp4", "avi", "mov", "wmv", "flv", "mkv", "webm", "m4v", "mpg", "mpeg", "m2v", "3gp",
        "3g2", "m4v",
    ];

    match path.extension() {
        Some(extension) => valid_extensions.contains(&extension.to_str().unwrap_or("")),
        None => false,
    }
}

pub enum PathType {
    Relative,
    FileOnly,
}

pub fn generate_output_filename(input_path: &PathBuf, path_type: PathType) -> PathBuf {
    let mut output_path = PathBuf::new();

    let filestem = input_path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("output");

    let extension = input_path
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("mp4");

    let now = Utc::now();
    let timestamp = now.format("%Y%m%d%H%M%S");

    if matches!(path_type, PathType::Relative) {
        output_path.push("./");
    }

    output_path.push(format!("{}_{}_trim.{}", filestem, timestamp, extension));
    output_path
}
