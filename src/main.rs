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
}

fn main() {
    let args = Args::parse();

    println!("Getting video stream");
    let url = Url::parse(&args.url).unwrap();
    let video = Video::from_url(&url).unwrap();
    let stream = video.best_audio().unwrap();
    println!("Downloading");

    if let Some(path) = args.out {
        stream.blocking_download_to(&path).unwrap();
        println!("Song downloaded to '{}'", path);
    } else {
        let path = stream.blocking_download().unwrap();
        println!("Song downloaded to '{}'", path.display());
    }
}