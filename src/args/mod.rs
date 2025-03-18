mod rate;
mod validate;

use {
    crate::utils::AppResult,
    clap::Parser,
    rate::RateLimit,
    url::Url,
};

#[rustfmt::skip]
#[derive(Parser, Debug)]
pub struct Args {
    #[arg(value_parser = Url::parse)]
    url: Url,

    #[arg(short = 'B', help = "Background Download")]
    background: bool,

    #[arg(short = 'O', help = "Customize Name")]
    output: Option<String>,

    #[arg(short = 'P', help = "Customize path")]
    path: Option<String>,

    #[arg(long = "rate-limit", value_parser = RateLimit::parse, help = "Set rate limit [k/M]")]
    rate_limit: Option<RateLimit>,

    #[arg(short = 'i', help = "Download different files")]
    input: Option<String>,

    #[arg(long = "mirror", help = "Mirroring Websites")]
    mirror: bool,

    #[arg(long = "reject", short = 'R', help = "Reject file types")]
    reject: Option<Vec<String>>,

    #[arg(long = "exclude", short = 'X', help = "Exclude directories")]
    exclude: Option<Vec<String>>,

    #[arg(long = "convert-links", help = "Convert links")]
    convert_links: bool,
}

impl Args {
    fn get_file_name(&self) -> String {
        let path = self
            .url
            .path();

        let segments: Vec<&str> = path
            .split('/')
            .collect();

        if let Some(last) = segments.last() {
            if !last.is_empty() {
                return last.to_string();
            }
        };

        if path.is_empty() || path == "" {
            return "index.html".to_string();
        };

        path.trim_end_matches('/')
            .split('/')
            .last()
            .unwrap_or("download")
            .to_string()
    }
}
