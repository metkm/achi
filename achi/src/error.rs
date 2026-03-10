use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error(transparent)]
    Interface(#[from] interfaces::error::Error),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),

    #[error(transparent)]
    ReqwestParse(#[from] serde_xml_rs::Error),
}

pub type Result<T> = std::result::Result<T, AppError>;
