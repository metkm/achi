use std::fmt::Debug;
use std::io::{BufRead, BufReader, Write};
use std::process::{Child, ChildStdin, ChildStdout, Command, Stdio};

use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

use crate::error::Result;

#[derive(Serialize, Deserialize, Debug)]
pub struct GetAchievement {
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetAchievementResponse {
    pub is_achieved: bool,
}

#[derive(Serialize, Deserialize)]
pub enum Cmd {
    GetAchievement(GetAchievement),
    SetAchievement(String),
    ClearAchievement(String),
}

pub struct SteamWorker {
    _child: Child,
    pub stdin: ChildStdin,
    pub reader: BufReader<ChildStdout>,
}

impl SteamWorker {
    pub fn new(id: i32) -> Result<Self> {
        let mut child = Command::new("steam_worker")
            .arg("--app-id")
            .arg(id.to_string())
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            // .stderr(Stdio::piped())
            .spawn()?;

        let stdin = child.stdin.take().unwrap();
        let stdout = child.stdout.take().unwrap();

        Ok(Self {
            _child: child,
            stdin,
            reader: BufReader::new(stdout),
        })
    }

    pub fn send<T: DeserializeOwned>(&mut self, request: Cmd) -> Result<T> {
        writeln!(self.stdin, "{}", serde_json::to_string(&request).unwrap())?;
        self.stdin.flush()?;

        let mut line = String::new();
        self.reader.read_line(&mut line)?;

        Ok(serde_json::from_str::<T>(&line)?)
    }
}
