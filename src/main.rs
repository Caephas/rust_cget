use clap::{App, Arg};
use tokio::fs::{OpenOptions, self};
use tokio::io::AsyncWriteExt;
use url::Url;
use indicatif::{ProgressBar, ProgressStyle};
use std::path::PathBuf;
use reqwest::header::CONTENT_LENGTH;



#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("Cget")
        .version("0.1.0")
        .author("Caephas <arinzeobidiegwu@gmail.com>")
        .about("A wget clone written in Rust")
        .arg(Arg::with_name("URL")
            .required(true)
            .takes_value(true)
            .index(1)
            .help("The URL to download "))
        .get_matches();

    let url = matches.value_of("URL").unwrap();
    let url = Url::parse(&url)?;
    download(url).await?;
    Ok(())
}

async fn download(url: Url) -> Result<(), Box<dyn std::error::Error>> {
    // Define file path
    let path = url
        .path_segments()
        .and_then(std::iter::Iterator::last)
        .and_then(|name| if name.is_empty() { None } else { Some(name) })
        .unwrap_or("tmp.zip");

    let mut file_path = dirs::download_dir().unwrap_or_else(|| PathBuf::from("~/Downloads"));
    file_path.push(path);
    
    // Check if file already exists and find its size
    let initial_position = if file_path.exists() {
        let metadata = fs::metadata(&file_path).await?;
        metadata.len()
    } else {
        0
    };
    
    // Make a range request to download only the remainder of the file
    let client = reqwest::Client::new();
    let mut response = client.get(url)
        .header("Range", format!("bytes={}-", initial_position))
        .send()
        .await?;

    let total_size = {
        response.headers()
            .get(CONTENT_LENGTH)
            .and_then(|ct_len| ct_len.to_str().ok())
            .and_then(|ct_len| ct_len.parse::<u64>().ok())
            .unwrap_or(0)
    } + initial_position;

    let pb = ProgressBar::new(total_size);
    pb.set_position(initial_position);
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})")
        .progress_chars("#>-"));
    
    let mut dest = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&file_path)
        .await?;

    while let Some(chunk) = response.chunk().await? {
        pb.inc(chunk.len() as u64);
        dest.write_all(&chunk).await?;
    }

    pb.finish_with_message("download finished");

    println!("Downloaded to {:?}", file_path);

    Ok(())
}
