use serde::{Serialize, Deserialize};
use std::{fs, collections::HashMap};

#[derive(Clone, Serialize, Deserialize)]
pub struct Run {
    pub name: String,
    pub job_id: String,
    pub status: Status,
    pub checked: bool,
}

#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub enum Status {
    Running,
    Pending,
    Finished,
}

#[derive(Serialize, Deserialize)]
pub struct RunState {
    pub runs: Vec<Run>,
}

pub fn state_file() -> std::path::PathBuf {
    let mut path = dirs::cache_dir().unwrap();
    path.push("cluster-monitor");
    fs::create_dir_all(&path).ok();
    path.push("runs.json");
    path
}

pub fn load() -> RunState {
    let path = state_file();
    if let Ok(data) = fs::read_to_string(path) {
        serde_json::from_str(&data).unwrap_or(RunState { runs: vec![] })
    } else {
        RunState { runs: vec![] }
    }
}

pub fn save(state: &RunState) {
    let data = serde_json::to_string_pretty(state).unwrap();
    fs::write(state_file(), data).unwrap();
}
