use std::fs;

use clap::Parser;
use regex::Regex;

fn main() {
    let args = Args::parse();
    match args.file_path {
        file_path if file_path.contains("Sakurato") => ying_du_pattern(file_path),
        _ => {}
    }
}

fn ying_du_pattern(file_path: String) {
    let mut directories = file_path.split('/').map(|x| x.to_string()).collect::<Vec<String>>();
    let file_name = directories.pop().unwrap();
    let extension = file_name.split('.').last().unwrap().to_string();
    let season = directories.pop().unwrap().split_ascii_whitespace().last().unwrap().to_string();
    let anime_name = directories.pop().unwrap();
    let regex = Regex::new(r"\[(.*?)\]").unwrap();
    let mut matches = regex.captures_iter(&file_name);
    let _fansub = matches.next().unwrap().get(1).unwrap().as_str();
    let episode = matches.next().unwrap().get(1).unwrap().as_str();
    let new_file_name = format!("{} - s{}e{}.{}", anime_name, season, episode, extension);
    fs::rename(file_path, format!("{}/{}", directories.clone().remove(directories.len() - 1), new_file_name)).unwrap();
}

#[cfg(test)]
mod tests {
    use regex::Regex;

    #[test]
    fn test_regex() {
        let regex = Regex::new(r"\[(.*?)\]").unwrap();
        let mut matches = regex.captures_iter("[Sakurato] Sousou no Frieren [01][HEVC-10bit 1080p AAC][CHS&CHT].mkv");
        let _fansub = matches.next().unwrap().get(1).unwrap().as_str();
        let episode = matches.next().unwrap().get(1).unwrap().as_str();
        assert_eq!(episode, "01");
    }
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
}