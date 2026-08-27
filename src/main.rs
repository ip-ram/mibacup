mod backup;
use backup::BackupConfig;

use std::env::{self};

use std::thread;
use std::time::{Duration, Instant};

use fs_extra::dir::{CopyOptions, copy};
use std::fs;
use std::io::Write;

use chrono::Local;

fn main() {
    let args = env::args();

    let _conf = match BackupConfig::env2conf(args) {
        Ok(conf) => match run(&conf) {
            Ok(str) => Ok(str),
            Err(err) => Err(err),
        },
        Err(str) => panic!("{str}"),
    };
}

pub fn run(conf: &BackupConfig) -> Result<&'static str, std::io::Error> {
    let interval = Duration::from_secs(conf.get_frequency());
    let mut next_run = Instant::now();

    loop {
        let now = Instant::now();

        if next_run > now {
            thread::sleep(interval);
        }

        next_run += interval;

        let _result = match backup_dir(conf.get_data_dir(), conf.get_backup_dir()) {
            Ok(status) => match print_log_in_log_dir(conf.get_backup_dir(), status) {
                Ok(str) => Ok(str),
                Err(err) => Err(err),
            },
            Err(status) => match print_log_in_log_dir(conf.get_backup_dir(), &status.to_string()) {
                Ok(str) => Ok(str),
                Err(err) => Err(err),
            },
        };
    }
}

fn backup_dir(from: String, to: String) -> Result<&'static str, fs_extra::error::Error> {
    let mut options = CopyOptions::new();

    options.copy_inside = false;

    match copy(from, to, &options) {
        Result::Ok(_int) => Ok("backup success"),
        Result::Err(err) => Err(err),
    }
}

fn print_log_in_log_dir(to: String, log_text: &str) -> Result<&'static str, std::io::Error> {
    let backup_dir = to;
    let log_path = format!("{}{}", backup_dir, "/log.txt");
    let mut file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_path)
        .unwrap();

    let now_from_str = Local::now();
    let log_output = now_from_str.to_string() + log_text;
    match writeln!(file, "{log_output}") {
        Ok(_) => Ok("success"),
        Err(err) => Err(err),
    }
}
