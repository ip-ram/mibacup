use crate::prelude::*;
//For writeln
use std::io::Write;
/*
Function to write log in file
Converts the vector Vec<[String, bool, date]> to a colorful log: [date]: STATUS log
*/

pub fn write_log(
    log_directory: impl AsRef<std::path::Path>,
    status: &[String],
    date: &str,
) -> Result<()> {
    let log_directory = log_directory.as_ref();

    std::fs::create_dir_all(log_directory)?;

    let log_path = log_directory.join("mibackup_log.txt");

    let file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log_path)?;

    let mut writer = std::io::BufWriter::new(file);

    for status_item in status {
        writeln!(writer, "[{}] {}", date, status_item)?;
    }

    writer.flush()?;

    Ok(())
}

// Function to print log to ui
pub fn print_log(status: &[String], date: &str) {
    for status_item in status {
        println!("[{}] {}", date, status_item);
    }
}
