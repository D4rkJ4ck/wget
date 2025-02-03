use {clap::Parser, std::path::Path};

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(short = 'B', help = "Background Download")]
    pub background: bool,

    #[arg(short = 'O', help = "Customize Name")]
    pub output: Option<String>,

    #[arg(short = 'P', default_value = "~/Download/", help = "Customize path")]
    pub path: Option<String>,

    #[arg(long = "rate-limit", help = "Set rate limit [k / M]")]
    pub rate_limit: Option<String>,

    #[arg(short = 'i', help = "Download different files")]
    pub input: Option<String>,

    #[arg(long = "mirror", help = "Website mirroring")]
    pub mirror: bool,

    #[arg(long = "reject", help = "Reject file types")]
    pub reject: Vec<String>,

    #[arg(long = "exclude", help = "Exclude directories")]
    pub exclude: Vec<String>,

    #[arg(long = "convert-links", help = "Convert links in output")]
    pub convert_links: bool,

    #[arg()]
    pub url: String,
}

impl Args {
    pub async fn validate(&self) -> Result<(), String> {
        if let Some(path) = &self.path {
            if !path.starts_with("~/") && !path.starts_with('/') {
                return Err("Path must start with '˜/'or '/'".to_string());
            }
        }

        if let Some(rate) = &self.rate_limit {
            if !rate.ends_with('k') && !rate.ends_with('M') {
                return Err("Rate limit must end with 'k'or 'M'".to_string());
            }

            let num = rate[..rate.len() - 1].parse::<f64>();
            if num.is_err() || num.unwrap() <= 0.0 {
                return Err("Rate limit must be a positive number".to_string());
            }
        }

        if let Some(file) = &self.input {
            if !Path::new(file).exists() {
                return Err(format!("Input file '{}'does not exist", file));
            }
        }

        if !self.url.starts_with("http://") && !self.url.starts_with("https://") {
            return Err("URL must start with 'http://' or 'https://'".to_string());
        }

        Ok(())
    }
}
