use crate::error::Result;

use gpui::{Context, EventEmitter};
use interfaces::steam;

#[derive(Debug)]
pub enum SteamEvent {
    Initialized,
}

pub struct SteamState {
    pub client: Result<steam::SteamClient>,
}

impl SteamState {
    pub fn new() -> Self {
        let client = steam::SteamClient::new().map_err(crate::error::AppError::from);

        Self { client }
    }

    pub fn reload(cx: &mut Context<Self>) {
        cx.spawn(async move |this, cx| {
            this.update(cx, |this, cx| {
                this.client = steam::SteamClient::new().map_err(crate::error::AppError::from);

                if this.client.is_ok() {
                    cx.emit(SteamEvent::Initialized);
                }

                cx.notify();
            })
            .ok();
        })
        .detach();
    }
}

impl EventEmitter<SteamEvent> for SteamState {}
