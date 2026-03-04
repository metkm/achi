use std::{fs::File, io::Write};

use crate::games::get_game_list;

use log::info;

mod error;
mod games;
mod interfaces;
mod keyvalue;
mod steam;

fn main() -> anyhow::Result<()> {
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
        .init();

    let steam = steam::Steam::new()?;
    let client = steam.create_interface::<interfaces::client::SteamClient>()?;

    let pipe = client.create_stream_pipe()?;
    let user = client.connect_to_global_user(pipe);

    let steam_user = client.get_steam_user(user, pipe);

    let steam_id = steam_user.get_steam_id();
    let is_logged_on = steam_user.get_is_logged_on();

    info!("user steam id: {steam_id}. Logged on: {is_logged_on}");

    let _steam_apps001 = client.get_steam_apps001(user, pipe);

    // let data = steam_apps001.get_appdata(480, "name");
    // println!("App name {:?}", data.unwrap());

    let _steam_apps008 = client.get_steam_apps008(user, pipe);

    // let owned_games = get_game_list()?
    //     .into_iter()
    //     .filter(|id| steam_apps008.is_subscribed_app(*id))
    //     .collect::<Vec<i32>>();

    // info!("found {} games", owned_games.len());

    // for game_id in owned_games {
    //     println!("{:?} - {:?}", game_id, _steam_apps001.get_appdata(game_id, "name"));
    // }

    // info!("start");
    // keyvalue::Vdf::from_install_path(&steam::Steam::get_install_path().unwrap(), 3450310).unwrap();

    let kvt = keyvalue::KeyValue::from_install_path(&steam::Steam::get_install_path().unwrap(), 3450310)
            .unwrap();

    let stats = kvt
        .get_kv_by_name("3450310")
        .and_then(|kv| kv.get_kv_by_name("stats"));

    let mut file = File::create("asd.txt").unwrap();
    file.write_all(format!("{:#?}", stats).as_bytes());



    // if let Some(_stats) = stats {
    //     for stat in &_stats.children {
    //         println!("{:?} - {:?}", stat.name, stat.value);
    //     }
    // }

    // println!("{:#?}", kvt);
    // info!("end");

    // for id in get_game_list().unwrap() {
    //     if !steam_apps008.is_subscribed_app(id) {
    //         continue;
    //     }

    //     let name = steam_apps001.get_appdata(id, "name").unwrap();
    //     // println!("id: {:?} -  name: {:?}", id, name);
    //     info!("id: {}")
    // }

    Ok(())
}
