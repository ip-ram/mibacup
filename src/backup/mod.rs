pub struct BackupConfig {
    frequency: u64,
    data_dir: String,
    backup_dir: String,
}

impl BackupConfig {
    pub fn new(frequency: u64, data_dir: String, backup_dir: String) -> BackupConfig {
        BackupConfig {
            frequency,
            data_dir,
            backup_dir,
        }
    }
    pub fn get_frequency(&self) -> u64 {
        self.frequency
    }
    pub fn get_data_dir(&self) -> String {
        self.data_dir.clone()
    }
    pub fn get_backup_dir(&self) -> String {
        self.backup_dir.clone()
    }
    pub fn env2conf(mut args: impl Iterator<Item = String>) -> Result<BackupConfig, &'static str> {
        //skip unneeded parameter
        args.next();

        let frequency: u64 = match args.next() {
            Some(freq) => freq.trim().parse().unwrap(),
            None => return Err("no frequency set"),
        };

        let data_dir: String = match args.next() {
            Some(str) => str,
            None => return Err("no data dir set"),
        };

        let backup_dir: String = match args.next() {
            Some(str) => str,
            None => return Err("no backupdir set"),
        };

        let conf = BackupConfig {
            frequency,
            data_dir,
            backup_dir,
        };
        Ok(conf)
    }
}
