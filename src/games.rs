use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Document {
    #[serde(rename = "game")]
    games: Vec<i32>,
}

pub fn get_game_list() -> anyhow::Result<Vec<i32>> {
    let response = reqwest::blocking::get("https://gib.me/sam/games.xml")
        .unwrap()
        .text()
        .unwrap();

    let parsed = serde_xml_rs::from_str::<Document>(&response)?;

    Ok(parsed.games)
}
