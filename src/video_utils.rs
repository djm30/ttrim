use chrono::prelude::*;
use std::fmt::format;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::str;

// Need to make this handle errors
pub fn get_video_length(path: &PathBuf) -> f64 {
    let video_path = path.to_owned().into_os_string().into_string().unwrap();
    let output = Command::new("ffprobe")
        .arg("-v")
        .arg("error")
        .arg("-show_entries")
        .arg("format=duration")
        .arg("-of")
        .arg("default=noprint_wrappers=1:nokey=1")
        .arg(&video_path)
        .output()
        .unwrap();

    let duration_str = str::from_utf8(&output.stdout).unwrap();
    let duration: f64 = duration_str.trim().parse().unwrap();

    // println!("Video Path: {}", video_path);
    // println!("Error: {}", str::from_utf8(&output.stderr).unwrap());
    // println!("Duration: {}", duration_str);
    // println!("Duration: {}", duration);

    duration
}

// Todo: Implement this function
pub fn trim_video(start: f64, end: f64, input_path: &PathBuf, output_path: &PathBuf) {
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
        .output()
        .unwrap();

    println!("Error: {}", str::from_utf8(&output.stderr).unwrap());
    println!("Output: {}", str::from_utf8(&output.stdout).unwrap());
}

pub fn check_valid_file_extension(path: &PathBuf) -> bool {
    let extension = path.extension().unwrap();
    extension == "mp4"
}

pub fn generate_filename(input_path: &PathBuf) -> PathBuf {
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

    output_path.push("./");
    output_path.push(format!("{}_{}_trim.{}", filestem, timestamp, extension));
    output_path
}
