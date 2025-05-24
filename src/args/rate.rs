use {
    crate::{
        debug,
        utils::{
            AppErr,
            AppResult,
        },
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
        let re = Regex::new(r"(\d+)([k|M])$")?
            // .map_err(|e| debug!(e).into())?
            .captures(input);

        match re {
            Some(captures) => {
                let value = captures
                    .get(1)
                    .map(|m| m.as_str());

                let value = match value {
                    Some(num) => num.parse::<f64>()?,
                    None => return Err(debug!(AppErr::InvalidInput)),
                };

                let unit = captures
                    .get(2)
                    .map(|m| m.as_str());

                let unit = match unit {
                    Some("k") => RateUnit::Kilobytes,
                    Some("M") => RateUnit::Megabytes,
                    _ => return Err(debug!(AppErr::InvalidInput)),
                };

                Ok(Self {
                    value,
                    unit,
                })
            }
            None => Err(debug!(AppErr::InvalidInput)),
        }
    }
}
