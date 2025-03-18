use {
    super::Args,
    crate::{
        debug,
        utils::{
            AppErr,
            AppResult,
        },
    },
};

impl Args {
    pub fn validate(self) -> AppResult<Self> {
        let has_options = self
            .exclude
            .is_some()
            || self
                .reject
                .is_some()
            || self.convert_links;

        if !self.mirror && has_options {
            return Err(debug!(AppErr::InvalidInput));
        }
        
        

        Ok(self)
    }
}
