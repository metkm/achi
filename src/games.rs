use std::io::{Read, Write};

use crate::error::AppError;

use log::{error, info};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Document {
    #[serde(rename = "game")]
    games: Vec<i32>,
}

pub fn get_game_list() -> Result<Vec<i32>, AppError> {
    let file_content = match std::fs::File::open("game_list.txt") {
        Ok(mut file) => {
            let mut content = String::new();

            if let Err(error) = file.read_to_string(&mut content) {
                error!(
                    "game_list.txt cache is found but there was an error reading contents: {error}"
                );
                None
            } else {
                Some(content)
            }
        }
        Err(_) => {
            info!("game_list.txt cache is not found");
            None
        }
    };

    let parsed: Document = match file_content {
        Some(file_content) => serde_xml_rs::from_str(&file_content)?,
        None => {
            info!("getting list of games from https://gib.me/sam/games.xml");
            let response = reqwest::blocking::get("https://gib.me/sam/games.xml")?.text()?;

            match std::fs::File::create("game_list.txt") {
                Ok(mut file) => {
                    info!("writing game list cache to game_list.txt");

                    if let Err(error) = file.write(&response.as_bytes()) {
                        error!("error writing cache to file game_list.txt {error}");
                    };
                }
                Err(error) => {
                    error!("error creating cache file game_list.txt {error}");
                }
            };

            serde_xml_rs::from_str(&response)?
        }
    };

    Ok(parsed.games)
}
