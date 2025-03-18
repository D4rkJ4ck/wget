use crate::utils::AppResult;

#[derive(Debug, Default, Clone, clap::Args)]
pub(crate) struct Mirror {
    #[arg(
        long = "reject",
        short = 'R',
        help = "Reject file types"
    )]
    reject: Vec<String>,

    #[arg(
        long = "exclude",
        short = 'X',
        help = "Exclude directories"
    )]
    exclude: Vec<String>,

    #[arg(long = "convert-links", help = "Convert links")]
    convert_links: bool,
}

impl Mirror {
    pub fn parse() -> AppResult<Self> { Ok(Self::default()) }
}
