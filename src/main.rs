// #![allow(dead_code, unused)]
// #![cfg_attr(not(debug_assertions), warn(dead_code, unused))]

mod components;
mod error;
mod games;
mod http_client;
mod interfaces;
mod keyvalue;
mod models;
mod program;
mod steam;

use std::{borrow::Cow, rc::Rc, sync::Arc};

use gpui::{App, AppContext, Application, WindowOptions, px};

use gpui_component::{Root, Theme, ThemeSet};
use gpui_component_assets::Assets;

use crate::program::Program;

pub fn init(cx: &mut App) {
    const CATPPUCCIN_MOCHA: &[u8] = include_bytes!("../assets/themes/catppuccin.json");

    let theme_str = std::str::from_utf8(CATPPUCCIN_MOCHA).unwrap();
    let theme_set: ThemeSet = serde_json::from_str(theme_str).unwrap();

    if let Some(theme_config) = theme_set
        .themes
        .iter()
        .find(|them| them.name == "Catppuccin Mocha")
    {
        Theme::global_mut(cx).apply_config(&Rc::new(theme_config.clone()));
    }

    // ThemeRegistry::watch_dir(PathBuf::from("./assets/themes"), cx, |cx| {
    //     let theme = ThemeRegistry::global(cx)
    //         .themes()
    //         .get("Catppuccin Mocha")
    //         .cloned()
    //         .unwrap();

    //     Theme::global_mut(cx).apply_config(&theme);
    // });
}

fn main() {
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
        .init();

    let client = http_client::ReqwestHttpClient::new().unwrap();

    let application = Application::new()
        .with_http_client(Arc::new(client))
        .with_assets(Assets);

    application.run(|cx| {
        cx.text_system()
            .add_fonts(vec![Cow::Borrowed(include_bytes!(
                "../assets/Inter-Medium.ttf"
            ))])
            .unwrap();

        gpui_component::init(cx);
        init(cx);

        let window_options = WindowOptions {
            titlebar: None,
            window_min_size: Some(gpui::Size::new(px(1050.0), px(610.0))),
            window_background: gpui::WindowBackgroundAppearance::Opaque,
            ..Default::default()
        };

        cx.spawn(async move |cx| {
            cx.open_window(window_options, |window, cx| {
                let view = cx.new(|context| Program::new(context));

                cx.new(|cx| Root::new(view, window, cx))
            })?;

            Ok::<_, anyhow::Error>(())
        })
        .detach();
    });

    // unsafe {
    //     std::env::set_var("SteamAppId", "3164500");
    // }

    // let steam = steam::Steam::new()?;
    // let client = steam.get_steam_client()?;

    // let pipe = client.create_stream_pipe()?;
    // let user = client.connect_to_global_user(pipe);

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
}
