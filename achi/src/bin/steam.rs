use std::io::{BufRead, BufReader, BufWriter, Write};

use clap::Parser;
use interfaces::steam::Steam;

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
    let _user = client.connect_to_global_user(pipe);

    let stdout = std::io::stdout();
    let stdin = std::io::stdin();

    let mut writer = BufWriter::new(stdout.lock());
    let mut reader = BufReader::new(stdin.lock());

    if let Some(app_id) = args.app_id {
        println!("Initialized with id {app_id}");
        writer.flush().unwrap();
    }

    let mut buf = String::new();

    loop {
        buf.clear();

        if reader.read_line(&mut buf).unwrap() == 0 {
            break;
        }

        writeln!(writer, "pong from worker {}", buf.trim()).unwrap();
        writer.flush().unwrap();
    }
    Ok(())
}
