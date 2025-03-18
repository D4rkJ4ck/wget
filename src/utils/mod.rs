mod error;
mod functions;

pub(crate) use {
    error::{
        AppErr,
        AppResult,
    },
    functions::parse_rate_limit_for_clap,
};
