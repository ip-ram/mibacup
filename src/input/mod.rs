#[derive(Clone, Copy)]
enum TimeUnit {
    Minute(u8),
    Hour(u8),
    Day(u8),
    Month(u8),
    Year(u8),
}

enum CompressionMethod {
    Gzip,
    Zstd,
    None,
}

impl CompressionMethod {
    fn get_compression_method(&self) -> &Self {
        self
    }
}

enum IncrementalMethod {
    Full,
    Incremental,
    Differential,
    None,
}

impl IncrementalMethod {
    fn get_incremental_method(&self) -> &Self {
        self
    }
}

pub struct Config {
    frequency: TimeUnit,            // Частота создания бекапа
    removal_frequency: TimeUnit,    // Частота удаления бекапов
    configuration_file: String,     // Файл конфигурации
    source_path: Vec<String>,       // Файлы которые нужно бекапить
    exclude_path: Vec<String>,      // Файлы с конфигур которые не нужно бекапить
    backup_path: Vec<String>,       // Директория для бекапа
    recursive: bool,                // Рекурсивный бекап для папок
    incremental: IncrementalMethod, // Метод инкрементации
    compression: CompressionMethod, // Метод компрессии
    list: bool,                     // Вывод информации по бекапам
    verify: bool,                   // Проверка на поврежденность файла
    help: bool,
}

impl Config {
    pub fn new(
        frequency: TimeUnit,
        removal_frequency: TimeUnit,
        configuration_file: String,
        source_path: Vec<String>,
        exclude_path: Vec<String>,
        backup_path: Vec<String>,
        recursive: bool,
        incremental: IncrementalMethod,
        compression: CompressionMethod,
        list: bool,
        verify: bool,
        help: bool,
    ) -> Config {
        Config {
            frequency,
            removal_frequency,
            configuration_file,
            source_path,
            exclude_path,
            backup_path,
            recursive,
            incremental,
            compression,
            list,
            verify,
            help,
        }
    }
    // getter
    pub fn get_frequency(&self) -> &TimeUnit {
        &self.frequency
    }
    pub fn get_removal_frequency(&self) -> &TimeUnit {
        &self.removal_frequency
    }
    pub fn get_source_path(&self) -> &Vec<String> {
        &self.source_path
    }
    pub fn get_exclude_path(&self) -> &Vec<String> {
        &self.exclude_path
    }
    pub fn get_backup_path(&self) -> &Vec<String> {
        &self.backup_path
    }
    pub fn get_is_recursive(&self) -> bool {
        self.recursive
    }
    pub fn get_incremental_method(&self) -> &IncrementalMethod {
        &self.incremental
    }
    pub fn get_compression_method(&self) -> &CompressionMethod {
        &self.compression
    }
    pub fn get_list(&self) -> bool {
        self.list
    }
    pub fn get_verify(&self) -> bool {
        self.verify
    }
    //setter
    pub fn set_frequency(&mut self, frequency: TimeUnit) {
        self.frequency = frequency;
    }
    pub fn set_removal_frequency(&mut self, frequency: TimeUnit) {
        self.removal_frequency = frequency;
    }
    pub fn set_source_path(&mut self, source_path: String) {
        self.source_path.push(source_path);
    }
    pub fn set_exclude_path(&mut self, exclude_path: String) {
        self.exclude_path.push(exclude_path);
    }
    pub fn set_backup_path(&mut self, backup_path: String) {
        self.backup_path.push(backup_path)
    }
    pub fn set_recursive(&mut self, recursive: bool) {
        self.recursive = recursive;
    }
    pub fn set_incremental_method(&mut self, incremental_method: IncrementalMethod) {
        self.incremental = incremental_method;
    }
    pub fn set_compression_method(&mut self, compression_method: CompressionMethod) {
        self.compression = compression_method;
    }
    pub fn set_list(&mut self, list: bool) {
        self.list = list
    }
    pub fn set_verify(&mut self, verify: bool) {
        self.verify = verify;
    }

    pub fn env2conf(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        // make basik config
        let mut config = Config {
            frequency: TimeUnit::Minute(1),
            removal_frequency: TimeUnit::Minute(1),
            backup_path: Vec::new(),
            recursive: false,
            incremental: IncrementalMethod::None,
            compression: CompressionMethod::Gzip,
            verify: false,
            configuration_file: String::new(),
            source_path: Vec::new(),
            exclude_path: Vec::new(),
            list: false,
            help: false,
        };

        //skip unneeded parameter
        args.next();

        while let Some(arg) = args.next() {
            match arg.as_str() {
                "--frequency" | "-f" => match args.next() {
                    Some(time_unit) => {
                        let frequency: u8 = args.next().unwrap().parse().unwrap();
                        match time_unit.as_str() {
                            "-m" | "--minutes" => config.set_frequency(TimeUnit::Minute(frequency)),
                            "-h" | "--hours" => config.set_frequency(TimeUnit::Hour(frequency)),
                            "-d" | "--day" => config.set_frequency(TimeUnit::Day(frequency)),
                            "-M" | "--month" => config.set_frequency(TimeUnit::Month(frequency)),
                            "-y" | "--year" => config.set_frequency(TimeUnit::Year(frequency)),
                            &_ => config.set_frequency(TimeUnit::Minute(frequency)),
                        }
                    }
                    None => {
                        return Err("[ERROR] No frequency time set. Default time unit is minutes");
                    }
                },
                &_ => return Err(""),
            }
        }
        Ok(config)
    }
    // pub fn env2conf(mut args: impl Iterator<Item = String>) -> Result<BackupConfig, &'static str> {
    //     //skip unneeded parameter
    //     args.next();
    //
    //     let frequency: u64 = match args.next() {
    //         Some(freq) => freq.trim().parse().unwrap(),
    //         None => return Err("no frequency set"),
    //     };
    //
    //     let data_dir: String = match args.next() {
    //         Some(str) => str,
    //         None => return Err("no data dir set"),
    //     };
    //
    //     let backup_path: String = match args.next() {
    //         Some(str) => str,
    //         None => return Err("no backupdir set"),
    //     };
    //
    //     let conf = BackupConfig {
    //         frequency,
    //         data_dir,
    //         backup_path,
    //     };
    //     Ok(conf)
    // }
}
