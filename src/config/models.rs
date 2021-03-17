use std::{error::Error, fmt::{Display, Result, Formatter}};

#[derive(Debug)]
pub struct ConfigError(pub(crate) String);

impl Display for ConfigError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "Config Error: {}", self.0)
    }
}
impl Error for ConfigError {}