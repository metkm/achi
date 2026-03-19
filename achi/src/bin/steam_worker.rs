#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{
    cell::RefCell,
    io::{BufRead, BufReader, BufWriter, Write},
    rc::Rc,
    time::{Duration, Instant},
};

use anyhow::bail;
use clap::Parser;
use interfaces::{
    callbacks::user_stats_received::{Callback, UserStatsReceivedT},
    constants::USER_STATS_RECEIVED_ID,
    steam::Steam,
    worker::{Cmd, GetAchievement, GetAchievementResponse},
};

use log::{debug, error, info};

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    app_id: Option<i32>,
}

fn main() -> anyhow::Result<()> {
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
        .init();

    let args = Args::parse();

    if let Some(app_id) = args.app_id {
        unsafe {
            std::env::set_var("SteamAppId", app_id.to_string());
        }
    }

    let mut steam = Steam::new()?;
    let client = steam.get_steam_client()?;

    let pipe = client.create_steam_pipe()?;
    let user = client.connect_to_global_user(pipe);

    let user_stats = client.get_steam_user_stats(user, pipe);

    let received_user_stats = Rc::new(RefCell::new(false));

    let stats_received_callback: Callback<UserStatsReceivedT> = Callback {
        id: USER_STATS_RECEIVED_ID,
        is_server: false,
        on_run: {
            let received_user_stats = Rc::clone(&received_user_stats);

            Box::new(move |result| {
                debug!("Got results of UserStatsReceivedT {:?}", result);
                *received_user_stats.borrow_mut() = true;
            })
        },
    };

    steam.register_callback(stats_received_callback);

    unsafe { user_stats.request_userstats() };
    info!("Requested user results.");

    let timeout = Duration::from_secs(3);
    let start = Instant::now();

    while !*received_user_stats.borrow() {
        if start.elapsed() > timeout {
            error!("Receive user stats timeout");
            bail!("Receive user stats timeout");
        }

        steam.run_callbacks(pipe);
        std::thread::sleep(std::time::Duration::from_millis(200));
    }

    let stdout = std::io::stdout();
    let stdin = std::io::stdin();

    let mut writer = BufWriter::new(stdout.lock());
    let mut reader = BufReader::new(stdin.lock());

    let mut buf = String::new();

    writeln!(writer, "READY").ok();
    writer.flush().ok();

    loop {
        buf.clear();
        if reader.read_line(&mut buf).unwrap() == 0 {
            break;
        }

        let Ok(cmd) = serde_json::from_str::<Cmd>(&buf) else {
            continue;
        };

        match cmd {
            Cmd::GetAchievement(GetAchievement { id }) => {
                let mut is_achieved = false;
                let mut unlock_time = 0;

                let c_id = std::ffi::CString::new(id).expect("Invalid ID");

                unsafe {
                    user_stats.get_achievement_and_unlock_time(
                        c_id.as_ptr(),
                        &mut is_achieved,
                        &mut unlock_time,
                    );
                };

                let response = GetAchievementResponse { is_achieved };
                writeln!(writer, "{}", serde_json::to_string(&response).unwrap()).ok();
            }
            Cmd::SetAchievement(id) => {
                let c_id = std::ffi::CString::new(id).expect("Invalid ID");

                unsafe {
                    user_stats.set_achievement(c_id.as_ptr());
                };

                writeln!(writer, "true").ok();
            }
            Cmd::ClearAchievement(id) => {
                let c_id = std::ffi::CString::new(id).expect("Invalid ID");

                unsafe {
                    user_stats.clear_achievement(c_id.as_ptr());
                };

                writeln!(writer, "true").ok();
            }
        }

        writer.flush().unwrap();
    }

    Ok(())
}
