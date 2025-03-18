use {
    super::AppErr,
    crate::args::RateLimit,
    clap::error::ErrorKind,
};

pub(crate) fn parse_rate_limit_for_clap(
    input: &str,
) -> Result<RateLimit, clap::Error> {
    RateLimit::parse(input).map_err(|e| match e {
        AppErr::InvalidInput => clap::Error::raw(
            ErrorKind::InvalidValue,
            "Invalid rate limit input.\nMust be like '300k' of '2M'",
        ),
        _ => clap::Error::raw(ErrorKind::InvalidValue, "[ERROR] -> {e}."),
    })
}
