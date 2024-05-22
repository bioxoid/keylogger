use chrono::prelude::*;
use device_query::{DeviceQuery, DeviceState};
use std::fs::OpenOptions;
use std::io::Write;
use serde::Serialize;
use winput::{Action, Button, WheelDirection};
use winput::message_loop::{self};

#[derive(Serialize)]
#[derive(PartialEq)]
struct MouseLog {
    rel_x: i32,
    rel_y: i32,
    // abs_x: f32,
    // abs_y: f32,
    action: String,
    // wheel_button: String,
    // wheel_delta: f32,
    // wheel_direction: String,
}

impl MouseLog {
    fn init() -> MouseLog {
        MouseLog {
            rel_x: 0,
            rel_y: 0,
            // abs_x: 0.0,
            // abs_y: 0.0,
            action: String::from(""),
            // wheel_button: String::from(""),
            // wheel_delta: 0.0,
            // wheel_direction: String::from(""),
        }
    }
}

#[derive(Serialize)]
#[derive(PartialEq)]
struct KeyLog {
    keyboard: Vec<String>,
    different_key_pressed: bool,
}

impl KeyLog {
    fn init() -> KeyLog {
        KeyLog {keyboard: [].to_vec(), different_key_pressed: false,}
    }
}

#[derive(Serialize)]
struct Log {
    unix: i64,
    delta: i64,
    keyboard: KeyLog,
    mouse: MouseLog
}

impl Log {
    fn init() -> Log {
        Log {
            unix: 0,
            delta: 0,
            keyboard: KeyLog::init(),
            mouse: MouseLog::init()
        }
    }
}

fn mouse_event (receiver: &message_loop::EventReceiver) -> MouseLog {
    let mut mouselog = MouseLog::init();
    match receiver.next_event() {
        // message_loop::Event::MouseMoveAbsolute {x,y, ..} => {
        //     mouselog.abs_x = x;
        //     mouselog.abs_y = y;
        // },
        message_loop::Event::MouseMoveRelative {x,y,} => {
            mouselog.rel_x = x;
            mouselog.rel_y = y;
        },
        message_loop::Event::MouseButton  {action, button} => {
            mouselog.action = if action == Action::Press {String::from("Press")} else {String::from("Release")};
            mouselog.action = match button {
                Button::Left => String::from("Left"),
                Button::Right => String::from("Right"),
                Button::Middle => String::from("Middle"),
                Button::X1 => String::from("X1"),
                Button::X2 => String::from("X2"),
            }
        },
        // message_loop::Event::MouseWheel  {delta, direction} => {
        //     mouselog.wheel_delta = delta;
        //     mouselog.wheel_direction = if direction == WheelDirection::Vertical {String::from("Vertical")} else if direction == WheelDirection::Horizontal {String::from("Horizontal")} else {String::from("")};
        // },
        _ => (),
    };
    // println!("{:?}", receiver.next_event());
    return mouselog
}

/// Keylogger launch function
pub fn run(path: String) {
    let device_state = DeviceState::new();
    let receiver: message_loop::EventReceiver = message_loop::start().unwrap();
    let mut prev_keys = vec![];
    let mut prev_date: DateTime<Local> = Local::now();
    let mut prev_log = Log::init(); 
    let path = format!("/Users/ogata/Documents/logger/data/{}.jsonl", path);
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
        let mouse_log = mouse_event(&receiver);
        let log = Log {
            unix,
            delta,
            keyboard: KeyLog {
                keyboard: keys.iter().map(|key| key.to_string()).collect(),
                different_key_pressed: keys != prev_keys && !keys.is_empty(),
            },
            mouse: mouse_log
        };
        let json_log = serde_json::to_string(&log).unwrap();
        if prev_log.keyboard != log.keyboard || prev_log.mouse != log.mouse {
            writeln!(file, "{}", json_log).expect("Failed to write to file");
            prev_date = local;
            prev_keys = keys;
        }
        prev_log = log;
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
}

fn main() {
    let game_title = std::env::args().nth(1).expect("no path given");
    run(game_title);
}