use std::{
    fmt,
    num,
};

pub type AppResult<T> = Result<T, AppErr>;

#[derive(Debug)]
pub enum AppErr {
    Regex(regex::Error),
    ParseFloat(num::ParseFloatError),
    InvalidInput,
    Mirror(&'static str),
    Other(String),
}

impl fmt::Display for AppErr {
    #[rustfmt::skip]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppErr::Regex(e) => write!(f, "Regex Matching: {}", e),
            AppErr::ParseFloat(e) => write!(f, "Float Parsing: {}", e),
            AppErr::InvalidInput => write!(f, "Invalid input"),
            AppErr::Mirror(option) => write!(f, "Cannot use {} without '--mirror'", option),
            AppErr::Other(msg) => write!(f, "{msg}"),
        }
    }
}

impl std::error::Error for AppErr {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            AppErr::Regex(err) => Some(err),
            AppErr::ParseFloat(err) => Some(err),
            _ => None,
        }
    }
}

impl From<regex::Error> for AppErr {
    fn from(err: regex::Error) -> Self { AppErr::Regex(err) }
}

impl From<num::ParseFloatError> for AppErr {
    fn from(err: num::ParseFloatError) -> Self { AppErr::ParseFloat(err) }
}

impl From<String> for AppErr {
    fn from(err: String) -> Self { AppErr::Other(err) }
}

impl From<AppErr> for clap::Error {
    fn from(e: AppErr) -> Self {
        match e {
            AppErr::InvalidInput => clap::Error::raw(
                clap::error::ErrorKind::InvalidValue,
                "Invalid Input: {e}",
            ),
            _ => clap::Error::raw(
                clap::error::ErrorKind::InvalidValue,
                "[ERROR] -> {e}",
            ),
        }
    }
}
