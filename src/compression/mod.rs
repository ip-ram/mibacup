use crate::prelude::*;

pub fn compress_dir_gzip(
    backup_directory_list: &Vec<impl AsRef<std::path::Path>>,
    source_directory_list: &Vec<impl AsRef<std::path::Path>>,
    excludes_directory_list: &Vec<impl AsRef<std::path::Path>>,
    date: &str,
) -> Result<Vec<String>> {
    let mut status: Vec<String> = Vec::new();

    for source_directory in source_directory_list {
        let source_directory = source_directory.as_ref().to_str().unwrap();
        let filename = format!("{}.tar.gz", date);

        match std::fs::File::create(filename) {
            Ok(file) => {
                let encoder = flate2::write::GzEncoder::new(file, flate2::Compression::default());
                let mut tar = tar::Builder::new(encoder);
                for backup_directory in backup_directory_list {
                    let backup_directory = backup_directory.as_ref().to_str().unwrap();
                    match tar.append_dir_all(backup_directory, source_directory) {
                        Ok(ok) => status.push(format!(
                            "sucsessfuly added {} to {}",
                            source_directory, backup_directory
                        )),
                        Err(err) => status.push(err.to_string()),
                    }
                }
            }
            Err(err) => status.push(err.to_string()),
        }

        for backup_directory in backup_directory_list {}
    }

    Ok(status)
}
