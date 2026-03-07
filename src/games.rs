use std::fs;

use crate::error::AppError;

use log::{error, info, warn};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Document {
    #[serde(rename = "game")]
    games: Vec<i32>,
}

pub fn get_game_list() -> Result<Vec<i32>, AppError> {
    let cache_path = "game_list.txt";
    let url = "https://gib.me/sam/games.xml";

    if let Ok(content) = fs::read_to_string(cache_path) {
        info!("Loading game list from cache");
        if let Ok(document) = serde_xml_rs::from_str::<Document>(&content) {
            return Ok(document.games)
        } else {
            error!("Cache is found but unable to parse")
        }
    } else {
        warn!("Cache not found");
    }

    info!("getting list of games from https://gib.me/sam/games.xml");

    let response = reqwest::blocking::get(url)?.text()?;

    if let Err(e) = fs::write(cache_path, &response) {
        error!("Failed to write cache: {e}");
    } else {
        info!("Cache updated");
    }
    
    let document = serde_xml_rs::from_str::<Document>(&response)?;
    Ok(document.games)
}
