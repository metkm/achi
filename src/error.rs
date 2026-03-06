use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("failed to load steamclient64.dll")]
    SteamClientLoad(String),

    #[error("failed to read registry path {0}")]
    RegistryRead(String),

    #[error("failed to create steam interface")]
    SteamInterfaceCreation,

    #[error("failed to create steam pipe. Is steam running?")]
    SteamPipeCreation,

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),

    #[error(transparent)]
    ReqwestParse(#[from] serde_xml_rs::Error),
}
