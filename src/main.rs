mod timestamp;
use clap::Parser;
use mp4;
use regex::Regex;
use std::env;

use timestamp::{Timestamp, TimestampError};

#[derive(Parser, Debug, Default)]
#[clap(author = "Dylan Morrison", version)]
/// ttrim - Trim video files directly in the terminal
struct Args {
    /// The video file to be trimmed
    target_file: String,
    #[clap(short, long)]
    /// The desired starting point
    start_timestamp: Option<String>,
    #[clap(short, long)]
    /// The desired end point
    end_timestamp: Option<String>,
    #[clap(short, long)]
    /// The output location
    output: Option<String>,
}

fn main() {
    let args = Args::parse();
    println!("{:?}", args);
    let x = Timestamp::parse_timestamp("12:35").unwrap();
    let y = Timestamp::parse_timestamp("10%").unwrap();
    let z = Timestamp::parse_timestamp("43").unwrap();
    dbg!(x);
    dbg!(y);
    dbg!(z);
}

// I want to focus on getting the CLI interface sorted first
