// #![allow(dead_code, unused)]
// #![cfg_attr(not(debug_assertions), warn(dead_code, unused))]

mod api;
mod app;
mod components;
mod error;
mod models;
mod program;
mod states;

use std::{borrow::Cow, rc::Rc, sync::Arc};

use gpui::{App, AppContext, Application, WindowOptions, px};

use gpui_component::{Root, Theme, ThemeSet};
use gpui_component_assets::Assets;

use crate::program::Program;

pub fn init(cx: &mut App) {
    const CATPPUCCIN_MOCHA: &[u8] = include_bytes!("assets/themes/catppuccin.json");

    let theme_str = std::str::from_utf8(CATPPUCCIN_MOCHA).unwrap();
    let theme_set: ThemeSet = serde_json::from_str(theme_str).unwrap();

    if let Some(theme_config) = theme_set
        .themes
        .iter()
        .find(|them| them.name == "Catppuccin Mocha")
    {
        Theme::global_mut(cx).apply_config(&Rc::new(theme_config.clone()));
    }
}

fn main() {
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
        .init();

    let client = app::http_client::ReqwestHttpClient::new().unwrap();

    let application = Application::new()
        .with_http_client(Arc::new(client))
        .with_assets(Assets);

    application.run(|cx| {
        cx.text_system()
            .add_fonts(vec![Cow::Borrowed(include_bytes!(
                "assets/Inter-Medium.ttf"
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
                let view = cx.new(|context| Program::new(window, context));

                cx.new(|cx| Root::new(view, window, cx))
            })?;

            Ok::<_, anyhow::Error>(())
        })
        .detach();
    });
}
