// use std::env;

// #[tokio::main]
// async fn main() -> Result<(), &'static str> {
//     let args: Vec<String> = env::args().collect();

//     if args.len() < 2 {
//         return Err("The normal usage of wget example: wget https://some_url.ogr/file.zip");
//     }

//     println!("Downloading file from: {}", args[1..].join("\n"));

//     Ok(())
// }

use clap::{Arg, Command};
use reqwest;
use std::path::Path;
use std::sync::Arc;
use tokio::{
    fs::File,
    io::AsyncWriteExt,
    task,
    time::{sleep, Duration},
};
use url::Url;

#[derive(Clone)]
struct DownloadOptions {
    background_download: bool,
    output_filename: Option<String>,
    save_path: String,
    rate_limit: Option<u64>,
    input_file: Option<String>,
    mirror_mode: bool,
    reject_file_types: Vec<String>,
    exclude_paths: Vec<String>,
    convert_links: bool,
}

impl DownloadOptions {
    fn parse_args() -> Self {
        let matches = Command::new("wget-clone")
            .arg(
                Arg::new("background")
                    .short('B')
                    .long("background")
                    .help("Download in background"),
            )
            .arg(
                Arg::new("output")
                    .short('O')
                    .long("output")
                    .takes_value(true)
                    .help("Output filename"),
            )
            .arg(
                Arg::new("path")
                    .short('P')
                    .long("path")
                    .takes_value(true)
                    .default_value(".")
                    .help("Path to save downloaded file"),
            )
            .arg(
                Arg::new("rate-limit")
                    .long("rate-limit")
                    .takes_value(true)
                    .help("Download speed limit (k/M)"),
            )
            .arg(
                Arg::new("input")
                    .short('i')
                    .long("input")
                    .takes_value(true)
                    .help("File with download links"),
            )
            .arg(
                Arg::new("mirror")
                    .long("mirror")
                    .help("Mirror entire website"),
            )
            .arg(
                Arg::new("reject")
                    .short('R')
                    .long("reject")
                    .takes_value(true)
                    .help("Reject file types"),
            )
            .arg(
                Arg::new("exclude")
                    .short('X')
                    .long("exclude")
                    .takes_value(true)
                    .help("Exclude paths"),
            )
            .arg(
                Arg::new("convert-links")
                    .long("convert-links")
                    .help("Convert links for offline viewing"),
            )
            .get_matches();

        Self {
            background_download: matches.is_present("background"),
            output_filename: matches
                .value_of("output")
                .map(String::from),
            save_path: matches
                .value_of("path")
                .unwrap_or(".")
                .to_string(),
            rate_limit: matches
                .value_of("rate-limit")
                .and_then(|limit| {
                    let (num, multiplier) = limit.split_at(limit.len() - 1);
                    let base: u64 = num.parse().ok()?;
                    match multiplier {
                        "k" => Some(base * 1024),
                        "M" => Some(base * 1024 * 1024),
                        _ => None,
                    }
                }),
            input_file: matches
                .value_of("input")
                .map(String::from),
            mirror_mode: matches.is_present("mirror"),
            reject_file_types: matches
                .value_of("reject")
                .map(|s| {
                    s.split(',')
                        .map(String::from)
                        .collect()
                })
                .unwrap_or_default(),
            exclude_paths: matches
                .value_of("exclude")
                .map(|s| {
                    s.split(',')
                        .map(String::from)
                        .collect()
                })
                .unwrap_or_default(),
            convert_links: matches.is_present("convert-links"),
        }
    }
}

async fn download_file(
    url: &str,
    options: Arc<DownloadOptions>,
) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let response = client.get(url).send().await?;
    let total_size = response
        .content_length()
        .unwrap_or(0);

    let filename = options
        .output_filename
        .clone()
        .unwrap_or_else(|| {
            Path::new(url)
                .file_name()
                .map(|f| {
                    f.to_string_lossy()
                        .into_owned()
                })
                .unwrap_or_else(|| "downloaded_file".to_string())
        });

    let save_path = Path::new(&options.save_path).join(&filename);
    let mut file = File::create(save_path).await?;

    let mut downloaded = 0u64;
    let mut stream = response.bytes_stream();

    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;

        // Rate limiting logic
        if let Some(rate_limit) = options.rate_limit {
            let sleep_duration =
                Duration::from_millis((chunk.len() as f64 / rate_limit as f64 * 1000.0) as u64);
            sleep(sleep_duration).await;
        }

        file.write_all(&chunk).await?;
        downloaded += chunk.len() as u64;

        // Progress tracking could be added here
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let options = Arc::new(DownloadOptions::parse_args());

    // Handle input file or direct URL
    if let Some(input_file) = &options.input_file {
        let urls = tokio::fs::read_to_string(input_file).await?;
        let download_tasks: Vec<_> = urls
            .lines()
            .map(|url| {
                let opts = options.clone();
                task::spawn(async move { download_file(url, opts).await })
            })
            .collect();

        for task in download_tasks {
            task.await??;
        }
    } else {
        // Single URL download
        let url = std::env::args()
            .nth(1)
            .expect("Please provide a URL to download");
        download_file(&url, options).await?;
    }

    Ok(())
}
