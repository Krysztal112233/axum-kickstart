use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("{0}")]
    Db(#[from] sea_orm::error::DbErr),

    #[error("{0}")]
    Conf(#[from] config::ConfigError),

    #[error("{0}")]
    Io(#[from] std::io::Error),

    #[error("{0}")]
    Unknown(String),

    #[error("status: {0}, msg: {1}")]
    UnknownWithCode(u16, String),
}
