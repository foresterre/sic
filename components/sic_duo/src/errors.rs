use thiserror::Error;

#[derive(Debug, Error)]
pub enum SicDuoError {
    #[error(
        "This crate is an in development crate and doesn't support this operation yet... ({0})"
    )]
    UnimplementedError(String),
}
