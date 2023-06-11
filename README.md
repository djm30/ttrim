# ttrim

ttrim is a blazingly fast (probably not) command line utilty to quickly and easily trim video files, using ffpmeg under the hood.

## Installation
Installation can be achieved via cloning this repository, building using `cargo build --release` and adding the resulting binary found in `target/release/ttrim` to your path.

Alternatively you can use the following commands to install:

### Linux / MacOS

```bash
curl -sSL https://raw.githubusercontent.com/djm30/ttrim/master/install.sh | sudo bash
```

*requires sudo permissions to move binary into /usr/local/bin folder*

### Windows
*Powershell*
```powershell
Invoke-Expression (curl https://raw.githubusercontent.com/djm30/ttrim/master/install.ps1 -UseBasicParsing).Content
```
*Must be run as an admin*

## Usage

Please ensure ffpmeg is installed on your system before using, installation instructions for this can be found <a href="https://ffmpeg.org/download.html">here</a>


### Mandatory Argument

- `target_file`: This is the path to the video file you want to trim. 

### Optional Arguments

- `-s`, `--start`: The desired starting point of the trimmed video. 
- `-e`, `--end`: The desired end point of the trimmed video. This can be provided as a timestamp in the format `HH:MM:SS`, `MM:SS`, a percentage `DD%` or as a number of seconds.

- `-e`, `--end`: The desired end point of the trimmed video. This can be provided as a timestamp in the format `HH:MM:SS`, `MM:SS`, a percentage `DD%` or as a number of seconds.

- `-o`, `--output`: The output location for the trimmed video file. If not provided, the trimmed video will be saved in the same directory as the original video. Can be a specific file or a directory in which case a filename will be generated

## Examples

```bash
# Trim video.mp4 starting from 00:10:00 to the end
ttrim video.mp4 --start 10:00

# Trim video.mp4 from the start to 00:45:00
ttrim video.mp4 --e 45

# Trim video.mp4 from 00:10:00 to 00:45:00
ttrim video.mp4 --start 00:10:00 --end 00:45:00

# Trim video.mp4 from 00:10:00 to 00:45:00 and output to /path/to/output.mp4
ttrim video.mp4 --start 10:00 --end 45:00 --output /path/to/output.mp4

# Trim video.mp4 to halfway through and save output to the desktop directory
ttrim video.mp4 -e 50% -o /Users/dylan/Desktop

# Units can be mixed
ttrim video.mp4 -s 10 -e 50%
```
