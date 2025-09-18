use thiserror::Error;

#[derive(Error, Debug)]
pub enum ActixError {
    #[error("{0}")]
    Std(#[from] std::io::Error),

    #[error("{0}")]
    Req(#[from] reqwest::Error),

    #[error("{0}")]
    Sqlx(#[from] sqlx::Error)
}