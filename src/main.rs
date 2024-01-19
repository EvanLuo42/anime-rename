use std::fs;

use clap::Parser;

fn main() {
    let args = Args::parse();
    match args.torrent_name {
        torrent_name if torrent_name.contains("桜都字幕组") => ying_du_pattern(torrent_name, args.file_path, args.root_path),
        _ => {}
    }
}

fn ying_du_pattern(torrent_name: String, file_path: String, root_path: String) {
    let episode = torrent_name
        .split_ascii_whitespace()
        .last()
        .unwrap()
        .replace('[', "")
        .replace(']', " ")
        .split_ascii_whitespace()
        .collect::<Vec<&str>>()
        .first()
        .unwrap()
        .to_string();
    let mut directories = file_path.clone().split('/').map(|x| x.to_string()).collect::<Vec<String>>();
    let file_name = directories.pop().unwrap();
    let season = directories.pop().unwrap().split_ascii_whitespace().last().unwrap().to_string();
    let anime_name = directories.pop().unwrap();
    let extension = file_name.split('.').last().unwrap().to_string();
    fs::rename(file_path, format!("{}/{} - s{}e{}.{}", root_path, anime_name, season, episode, extension)).unwrap();
}

#[derive(Parser, Debug)]
#[command(
    author = env!("CARGO_PKG_AUTHORS"),
    version = env!("CARGO_PKG_VERSION"),
    about = env!("CARGO_PKG_DESCRIPTION"),
    long_about = None,
)]
pub struct Args {
    /// Path of the file waiting for being renamed
    #[arg(short, long)]
    file_path: String,

    /// Name of the file waiting for being renamed
    #[arg(short, long)]
    torrent_name: String,

    #[arg(short, long)]
    root_path: String
}