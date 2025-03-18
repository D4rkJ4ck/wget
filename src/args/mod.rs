mod rate;
mod validate;

use {
    crate::utils::parse_rate_limit_for_clap,
    clap::Parser,
};
pub(super) use {
    // mirror::Mirror,
    rate::RateLimit,
};

#[derive(Parser, Debug)]
pub struct Args {
    #[arg()]
    url: String,

    #[arg(short = 'B', help = "Background Download")]
    background: bool,

    #[arg(short = 'O', help = "Customize Name")]
    output: Option<String>,

    #[arg(short = 'P', help = "Customize path")]
    path: Option<String>,

    #[arg(long = "rate-limit", value_parser = parse_rate_limit_for_clap, help = "Set rate limit [k/M]")]
    rate_limit: Option<RateLimit>,

    #[arg(short = 'i', help = "Download different files")]
    input: Option<String>,

    #[arg(long = "mirror", help = "Mirroring Websites")]
    mirror: bool,

    #[arg(
        long = "reject",
        short = 'R',
        help = "Reject file types"
    )]
    reject: Option<Vec<String>>,

    #[arg(
        long = "exclude",
        short = 'X',
        help = "Exclude directories"
    )]
    exclude: Option<Vec<String>>,

    #[arg(long = "convert-links", help = "Convert links")]
    convert_links: bool,
}
