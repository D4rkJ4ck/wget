use {
    crate::utils::{
        AppErr,
        AppResult,
    },
    regex::Regex,
};

#[derive(Debug, Clone)]
enum RateUnit {
    Kilobytes,
    Megabytes,
}

#[derive(Debug, Clone)]
pub(crate) struct RateLimit {
    value: f64,
    unit:  RateUnit,
}

impl RateLimit {
    pub(crate) fn parse(input: &str) -> AppResult<Self> {
        match Regex::new(r"(\d+)([k|M])$")?.captures(input) {
            Some(captures) => {
                let value = match captures
                    .get(1)
                    .map(|m| m.as_str())
                {
                    Some(num) => num.parse::<f64>()?,
                    None => return Err(AppErr::InvalidInput),
                };

                let unit = match captures
                    .get(2)
                    .map(|m| m.as_str())
                {
                    Some("k") => RateUnit::Kilobytes,
                    Some("M") => RateUnit::Megabytes,
                    _ => return Err(AppErr::InvalidInput),
                };

                Ok(Self {
                    value,
                    unit,
                })
            }
            None => Err(AppErr::InvalidInput),
        }
    }
}
