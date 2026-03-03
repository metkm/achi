use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    // #[error(transparent)]
    // WindowsError(#[from] windows::core::Error),
    #[error("unable to load steamclient64.dll. Are you sure this path is correct {0}?")]
    CantLoadSteamClientModule(String),

    #[error("registry path of {0} can't be opened or not found")]
    RegistryNotFound(String),

    #[error("can't create steam interface")]
    ErrorCreatingInterface,

    #[error("can't create stream pipe")]
    ErrorCreatingStreamPipe,

    #[error(transparent)]
    IoError(#[from] std::io::Error),

    #[error(transparent)]
    RequestError(#[from] reqwest::Error),

    #[error(transparent)]
    RequestParseError(#[from] serde_xml_rs::Error),
}
