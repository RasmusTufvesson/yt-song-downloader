use std::{fs, process::Command};

use clap::Parser;
use rustube::{blocking::Video, url::Url};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Url to download song from
    url: String,

    /// File name
    #[arg(short, long)]
    out: Option<String>,

    /// Convert to specified format (ffmpeg)
    #[arg(short, long)]
    convert: bool,
}

fn main() {
    let args = Args::parse();

    println!("Getting video stream");
    let url = Url::parse(&args.url).unwrap();
    let video = Video::from_url(&url).unwrap();
    let stream = video.best_audio().unwrap();
    println!("Downloading");

    if !args.convert {
        if let Some(path) = args.out {
            stream.blocking_download_to(&path).unwrap();
            println!("Song downloaded to '{}'", path);
        } else {
            let path = stream.blocking_download().unwrap();
            println!("Song downloaded to '{}'", path.display());
        }
    } else {
        let path = stream.blocking_download().unwrap();
        println!("Song downloaded to '{}', converting", path.display());
        if args.out == None && path.extension().is_some_and(|x| x == "mp3") {
            println!("Already mp3");
        } else {
            let out = match args.out {
                Some(val) => val,
                None => match path.file_stem() {
                    None => path.to_str().unwrap().to_owned() + ".mp3",
                    Some(val) => val.to_str().unwrap().to_owned() + ".mp3",
                },
            };
            let status = Command::new("ffmpeg")
                .arg("-i")
                .arg(path.to_str().unwrap())
                .arg(&out)
                .status()
                .expect("Failed to run ffmpeg");
            if status.success() {
                println!("Converted file, now  '{}'", out);
                println!("Deleting intermediate file");
                fs::remove_file(path).unwrap();
            } else {
                println!("FFmpeg error");
            }
        }
    }
    println!("Done");
}