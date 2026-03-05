use log::info;
use serde::{Deserialize, Serialize};

use crate::error::AppError;

#[derive(Serialize, Deserialize)]
struct Document {
    #[serde(rename = "game")]
    games: Vec<i32>,
}

pub fn get_game_list() -> Result<Vec<i32>, AppError> {
    info!("getting list of games from https://gib.me/sam/games.xml");

    info!("1");
    let response = reqwest::blocking::get("https://gib.me/sam/games.xml")?.text()?;

    info!("2");
    let parsed = serde_xml_rs::from_str::<Document>(&response)?;
    info!("3");

    Ok(parsed.games)
}
