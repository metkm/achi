use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error(transparent)]
    WindowsError(#[from] windows::core::Error),
    #[error("can't create interface")]
    CantCreateInterface,
    #[error("can't create stream pipe")]
    CantCreateStreamPipe,
}
