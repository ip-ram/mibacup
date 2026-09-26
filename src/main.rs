//self modules
mod archiving;
mod compression;
mod config;
mod copy;
mod log;
use config::Config;

//Thread modles
use std::thread;

//Time modules
use chrono::Local;
use std::time::{Duration, Instant};

//ENV modules
use std::env::{self};

//Prelde
pub mod prelude;

//Help tip
mod help;

fn main() {
    let mut args = env::args();
    match Config::env2conf(&mut args) {
        Ok(conf) => run(conf),
        Err(str) => panic!("{}", str),
    };
}

// TODO: Make the function functional
// 1. Reading the config
// 2. Making a decision depending on the config
fn run(config: Config) {
    if config.get_help() {
        println!("{}", help::HELP);
        return;
    }
    let interval = Duration::from_secs(*config.get_frequency_time());
    let mut next_run = Instant::now();

    loop {
        let now = Instant::now();

        if next_run > now {
            thread::sleep(interval);
        }

        next_run += interval;

        let date = Local::now().to_string();
    }
}
