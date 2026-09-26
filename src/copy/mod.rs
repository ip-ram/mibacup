use crate::prelude::*;

// Function for copy directory without compression
fn copy_dir(
    backup_directory_list: Vec<String>,
    source_directory_list: Vec<String>,
    excludes_directory_list: Vec<String>,
    date: &str,
) -> Result<Vec<String>> {
    let mut status: Vec<String> = Vec::new();
    let mut options = fs_extra::dir::CopyOptions::new();
    options.copy_inside = true;
    for backup_directory in &backup_directory_list {
        for source_directory in &source_directory_list {
            let backup_directory = format!("{}/{}", backup_directory, date);
            match fs_extra::dir::copy(source_directory, &backup_directory, &options) {
                Ok(ok) => {
                    let ok = format!("[OK]:create: {}, status: {}", backup_directory, ok);
                    status.push(ok);
                }
                Err(err) => {
                    let err = format!(
                        "[ERROR]:can`t create file {}, status: {}",
                        backup_directory, err
                    );
                    status.push(err.to_string());
                }
            }
        }
    }
    Ok(status)
}

// Function for copy files without compression
fn copy_file(
    backup_directory_list: Vec<String>,
    source_directory_list: Vec<String>,
    excludes_directory_list: Vec<String>,
    date: &str,
) -> Result<Vec<String>> {
    let mut status: Vec<String> = Vec::new();
    for backup_directory in &backup_directory_list {
        for source_directory in &source_directory_list {
            let backup_directory = format!("{}/{}", backup_directory, date);
            match std::fs::copy(source_directory, &backup_directory) {
                Ok(ok) => {
                    let ok = format!("[OK]:create: {}, status: {}", backup_directory, ok);
                    status.push(ok);
                }
                Err(err) => {
                    let err = format!(
                        "[ERROR]:can`t create file {}, status: {}",
                        backup_directory, err
                    );
                    status.push(err.to_string());
                }
            }
        }
    }
    Ok(status)
}
