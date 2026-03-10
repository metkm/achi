use std::io::{BufRead, BufReader, BufWriter, Write};

use clap::Parser;
use interfaces::{
    steam::Steam,
    worker::{GetAchievement, GetAchievementResponse},
};

use serde::{Deserialize, Serialize};

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    app_id: Option<i32>,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "method", content = "params")] // Optional: gives you cleaner JSON
pub enum Command {
    GetAchievement(GetAchievement),
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

    let steam = Steam::new()?;
    let client = steam.get_steam_client()?;

    let pipe = client.create_steam_pipe()?;
    let user = client.connect_to_global_user(pipe);

    let user_stats = client.get_steam_user_stats(user, pipe);

    let stdout = std::io::stdout();
    let stdin = std::io::stdin();

    let mut writer = BufWriter::new(stdout.lock());
    let mut reader = BufReader::new(stdin.lock());

    // if let Some(app_id) = args.app_id {
    //     writer.flush().unwrap();
    // }

    let mut buf = String::new();

    loop {
        buf.clear();
        if reader.read_line(&mut buf).unwrap() == 0 {
            break;
        }

        let Ok(cmd) = serde_json::from_str::<Command>(&buf) else {
            continue;
        };

        match cmd {
            Command::GetAchievement(GetAchievement { id }) => {
                let mut is_achieved = false;
                let mut unlock_time = 0;

                user_stats.get_achievement_and_unlock_time(
                    format!("{id}\0").as_ptr() as *const i8,
                    &mut is_achieved,
                    &mut unlock_time,
                );

                let response = GetAchievementResponse { is_achieved };
                writeln!(writer, "{}", serde_json::to_string(&response).unwrap()).ok();
            }
        }

        writer.flush().unwrap();
    }

    Ok(())
}
