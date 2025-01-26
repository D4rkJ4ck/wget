use clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(
        short = 'B',
        help = "Background Download"
    )]
    pub background: bool,

    #[arg(
        short = 'O',
        help = "Customize Name"
    )]
    pub output: Option<String>,

    #[arg(
        short = 'P',
        default_value = "~/Download/",
        help = "Customize path"
    )]
    pub path: Option<String>,

    #[arg(
        long = "rate-limit",
        help = "Set rate limit [k / M]"
    )]
    pub rate_limit: Option<String>,

    #[arg(
        short = 'i',
        help = "Download different files"
    )]
    pub input: Option<String>,

    #[arg(
        long = "mirror",
        help = "Website mirroring"
    )]
    pub mirror: bool,

    #[arg(
        long = "reject",
        help = "Reject file types"
    )]
    pub reject: Vec<String>,

    #[arg(
        long = "exclude",
        help = "Exclude directories"
    )]
    pub exclude: Vec<String>,

    #[arg(
        long = "convert-links",
        help = "Convert links in output"
    )]
    pub convert_links: bool,

    #[arg()]
    pub url: String,
}

impl Args {}
