use clap::Parser;
use mp4;
use regex::Regex;
use std::env;

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

enum Timestamp {
    Start,
    End,
    Seconds(u32),
    Percentage(u8),
}

impl Timestamp {
    fn parse_timestamp(timestamp: &str) -> Option<Timestamp> {
        // Check if it is just a number, then it is seconds,
        // Check if it is a colon separated timestamp, hh:mm:ss,
        // Otherwise check if it a percentage,
        // If its anything else, its not valid, so maybe return an option??
        if let Ok(seconds) = timestamp.parse::<u32>() {
            return Some(Timestamp::Seconds(seconds));
        }

        None
    }
}

fn main() {
    let args = Args::parse();
    println!("{:?}", args);
}

// I want to focus on getting the CLI interface sorted first
