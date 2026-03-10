use std::io::{BufRead, BufReader, BufWriter, Write};

use clap::Parser;
use interfaces::{
    steam::Steam,
    worker::{Cmd, GetAchievement, GetAchievementResponse},
};

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    app_id: Option<i32>,
}

fn main() -> anyhow::Result<()> {
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

    let mut buf = String::new();

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
