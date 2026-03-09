use crate::error::Result;

use interfaces::steam;

pub struct SteamState {
    pub client: Result<steam::SteamClient>,
}

impl SteamState {
    pub fn new() -> Self {
        let client =
            steam::SteamClient::new().map_err(|error| crate::error::AppError::from(error));

        Self { client }
    }

    pub fn reload(&mut self, id: Option<i32>) {
        println!("{:?}", id);

        unsafe {
            match id {
                Some(id) => {
                    std::env::set_var("SteamAppId", id.to_string());
                },
                None => {
                    std::env::remove_var("SteamAppId");
                }
            }
        }

        if let Ok(client) = &mut self.client {
            client.reload().ok();
        }
    }
}
