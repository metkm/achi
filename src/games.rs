#[allow(dead_code)]
pub fn get_game_list() {
    #[allow(unused_variables)]
    let response = reqwest::blocking::get("https://gib.me/sam/games.xml")
        .unwrap()
        .text()
        .unwrap();
}
