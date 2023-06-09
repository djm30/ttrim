use std::path::PathBuf;
use std::process::Command;
use std::str;

pub fn get_video_length(path: PathBuf) -> f64 {
    let video_path = path.into_os_string().into_string().unwrap();
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
    println!("Video Path: {}", video_path);
    println!("Error: {}", str::from_utf8(&output.stderr).unwrap());
    println!("Duration: {}", duration_str);
    println!("Duration: {}", duration);
    duration
}

// Todo: Implement this function
pub fn trim_video(start: f64, end: f64, path: PathBuf, filename: PathBuf) {}

fn generate_filename() -> String {
    String::from("test")
}
