use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Unable to create steam pipe")]
    UnableToCreateSteamPipe,
    #[error("Unable to load steamclient.dll: {0}")]
    UnableToLoadSteam(String),
    #[error("Unable to read registry")]
    UnableToReadRegistry(String),
    #[error("Unable to create steam interface")]
    UnableToCreateInterface,

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Serde(#[from] serde_json::Error),

    #[error("error reading stdout/stdin from worker")]
    WorkerStdio
}

pub type Result<T> = std::result::Result<T, Error>;
