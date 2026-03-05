#![allow(dead_code, unused)]
#![cfg_attr(not(debug_assertions), warn(dead_code, unused))]

use std::{fs::File, io::Write};

use log::info;

use crate::games::get_game_list;

mod error;
mod games;
mod interfaces;
mod keyvalue;
mod models;
mod steam;

fn main() -> anyhow::Result<()> {
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
        .init();

    // unsafe {
    //     std::env::set_var("SteamAppId", "3164500");
    // }

    let steam = steam::Steam::new()?;
    let client = steam.get_steam_client()?;

    let pipe = client.create_stream_pipe()?;
    let user = client.connect_to_global_user(pipe);

    // let steam_user = client.get_steam_user(user, pipe);

    // let steam_id = steam_user.get_steam_id();
    // let is_logged_on = steam_user.get_is_logged_on();

    // info!("user steam id: {steam_id}. Logged on: {is_logged_on}");

    // let steam_apps001 = client.get_steam_apps001(user, pipe);

    // let data = steam_apps001.get_appdata(480, "name");
    // println!("App name {:?}", data.unwrap());

    // // let steam_apps008 = client.get_steam_apps008(user, pipe);

    // // let owned_games = get_game_list()?
    // //     .into_iter()
    // //     .filter(|id| steam_apps008.is_subscribed_app(*id))
    // //     .collect::<Vec<i32>>();

    // // info!("found {} games", owned_games.len());

    // // for game_id in owned_games {
    // //     println!("{:?} - {:?}", game_id, _steam_apps001.get_appdata(game_id, "name"));
    // // }

    // // info!("start");

    // let kvt =
    //     keyvalue::KeyValue::from_install_path(&steam::Steam::get_install_path().unwrap(), 3164500)
    //         .unwrap();

    // let stats = kvt
    //     .get_kv_by_name("3164500")
    //     .and_then(|kv| kv.get_kv_by_name("stats"));

    // // let mut f = File::create_new("asd.txt").unwrap();
    // // f.write(format!("{:#?}", stats).as_bytes());

    // let user_stats = client.get_steam_user_stats(user, pipe);
    // user_stats.set_achievement(
    //     String::from("INDIAN_DEALER\0")
    //         .as_ptr() as *const i8
    // );

    // // if let Some(_stats) = stats {
    // //     for stat in &_stats.children {
    // //         for bits in stat.children.iter().filter(|b| b.name == "bits") {
    // //             for bit in &bits.children {
    // //                 let Some(achievement) = models::achievement::Achievement::from_bit_kv(bit) else {
    // //                     continue
    // //                 };

    // //                 println!("{:?}", achievement);
    // //             }
    // //         }
    // //     }
    // // }

    Ok(())
}
