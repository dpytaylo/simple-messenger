use std::fmt::Result;
use std::{
    error::Error,
    fmt::{Display, Formatter},
};

#[derive(Debug)]
pub struct ExtractionError;

impl Display for ExtractionError {
    fn fmt(&self, _f: &mut Formatter<'_>) -> Result {
        Ok(())
    }
}

impl Error for ExtractionError {}
