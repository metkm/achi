use crate::error::Result;

use interfaces::steam;

pub struct SteamState {
    pub client: Result<steam::SteamClient>,
}

impl SteamState {
    pub fn new(id: Option<i32>) -> Self {
        let client = steam::SteamClient::new(id).map_err(|error| crate::error::AppError::from(error));

        Self { client }
    }
}
