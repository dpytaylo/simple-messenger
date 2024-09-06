use garde::Report;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error(transparent)]
    Report(#[from] Report),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
