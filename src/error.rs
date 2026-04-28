use strum_macros::{AsRefStr, Display};

#[derive(AsRefStr, Display, Debug)]
pub enum Error{
    ConfigError(String),
    ValidationError(String),
    DeserializationError(String),
}