use thiserror::Error;

#[derive(Error, Debug)]
pub enum DbError {
    #[error("db read error")]
    Read(#[from] fjall::Error),
    #[error("parse error")]
    ValueParse(#[from] serde_json5::Error),
    #[error("value not found in option")]
    ValueNotFound,
}

pub type DbResult<T> = Result<T, DbError>;
