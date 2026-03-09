use crate::{error::Result, models};

use interfaces::{
    Interface,
    native::{steam_apps001::ISteamApps001, steam_apps008::ISteamApps008},
};

use log::{error, info, warn};
use serde::{Deserialize, Serialize};
use std::{fs, sync::Arc};

pub enum RequestStatus {
    Idle,
    Pending,
    Success,
    Error(String),
}

#[derive(Serialize, Deserialize)]
struct Document {
    #[serde(rename = "game")]
    games: Vec<i32>,
}

pub fn get_game_ids() -> Result<Vec<i32>> {
    let cache_path = "game_list.txt";
    let url = "https://gib.me/sam/games.xml";

    if let Ok(content) = fs::read_to_string(cache_path) {
        info!("Loading game list from cache");
        if let Ok(document) = serde_xml_rs::from_str::<Document>(&content) {
            return Ok(document.games);
        } else {
            error!("Cache is found but unable to parse")
        }
    } else {
        warn!("Cache not found");
    }

    info!("Getting list of games from https://gib.me/sam/games.xml");

    let response = reqwest::blocking::get(url)?.text()?;

    if let Err(e) = fs::write(cache_path, &response) {
        error!("Failed to write cache: {e}");
    } else {
        info!("Cache updated");
    }

    let document = serde_xml_rs::from_str::<Document>(&response)?;
    Ok(document.games)
}

pub fn get_library(
    apps001: Arc<Interface<ISteamApps001>>,
    apps008: Arc<Interface<ISteamApps008>>,
) -> Result<Vec<models::game::Game>> {
    let ids = get_game_ids()?;

    Ok(
        ids
            .into_iter()
            .filter_map(|id| {
                if id == 480 || !apps008.is_subscribed_app(id) {
                    return None;
                }

                let Some(name) = apps001.get_appdata(id, "name") else {
                    warn!("Couldn't get name for app id: {id}");
                    return None;
                };

                let Some(image) = apps001.get_appdata(id, "small_capsule/english") else {
                    warn!("Couldn't get image for app id: {id}");
                    return None;
                };

                Some(
                    models::game::Game {
                        id,
                        name,
                        image_url: format!("https://shared.cloudflare.steamstatic.com/store_item_assets/steam/apps/{id}/{image}")
                    }
                )
            })
            .collect()
    )
}
