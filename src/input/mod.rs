use chrono::format::Item;

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

    fn parse_frequency(
        &mut self,
        args: &mut dyn Iterator<Item = String>,
    ) -> Result<(), &'static str> {
        match args.next() {
            Some(time_unit) => {
                let frequency: u8 = args.next().unwrap().parse().unwrap();
                match time_unit.as_str() {
                    "-m" | "--minutes" => Ok(self.set_frequency(TimeUnit::Minute(frequency))),
                    "-h" | "--hours" => Ok(self.set_frequency(TimeUnit::Hour(frequency))),
                    "-d" | "--day" => Ok(self.set_frequency(TimeUnit::Day(frequency))),
                    "-M" | "--month" => Ok(self.set_frequency(TimeUnit::Month(frequency))),
                    "-y" | "--year" => Ok(self.set_frequency(TimeUnit::Year(frequency))),
                    &_ => Ok(self.set_frequency(TimeUnit::Minute(frequency))),
                }
            }
            None => {
                return Err("[ERROR] No frequency time set. Default time unit is minutes");
            }
        }
    }

    fn parse_remocal_frequency(
        &mut self,
        args: &mut dyn Iterator<Item = String>,
    ) -> Result<(), &'static str> {
        match args.next() {
            Some(time_unit) => {
                let removal_frequency: u8 = args.next().unwrap().parse().unwrap();
                match time_unit.as_str() {
                    "-m" | "--minutes" => {
                        Ok(self.set_removal_frequency(TimeUnit::Minute(removal_frequency)))
                    }
                    "-h" | "--hours" => {
                        Ok(self.set_removal_frequency(TimeUnit::Hour(removal_frequency)))
                    }
                    "-d" | "--day" => {
                        Ok(self.set_removal_frequency(TimeUnit::Day(removal_frequency)))
                    }
                    "-M" | "--month" => {
                        Ok(self.set_removal_frequency(TimeUnit::Month(removal_frequency)))
                    }
                    "-y" | "--year" => {
                        Ok(self.set_removal_frequency(TimeUnit::Year(removal_frequency)))
                    }
                    &_ => Ok(self.set_removal_frequency(TimeUnit::Minute(removal_frequency))),
                }
            }
            None => {
                return Err("[ERROR]:no removal frequency time set. Default time unit is minutes");
            }
        }
    }

    fn parse_source_file(
        &mut self,
        args: &mut dyn Iterator<Item = String>,
    ) -> Result<(), &'static str> {
        match args.next() {
            Some(source_path) => Ok(self.set_source_path(source_path)),
            None => Err(
                "[ERROR]:no source path set. Default source path is './'. You can add several different directories",
            ),
        }
    }

    pub fn env2conf(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        // make basik config
        let mut config = Config::new(
            TimeUnit::Minute(1),
            TimeUnit::Minute(1),
            String::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            false,
            IncrementalMethod::None,
            CompressionMethod::None,
            false,
            false,
            false,
        );

        //skip unneeded parameter
        args.next();

        while let Some(arg) = args.next() {
            match arg.as_str() {
                "--frequency" | "-f" => match config.parse_frequency(&mut args) {
                    Ok(_) => (),
                    Err(str) => return Err(&str),
                },
                "--frequency-removal" | "-F" => match config.parse_remocal_frequency(&mut args) {
                    Ok(_) => (),
                    Err(str) => return Err(&str),
                },
                "--source" | "-s" => match config.parse_source_file(&mut args) {
                    Ok(_) => (),
                    Err(str) => return Err(&str),
                },
                &_ => return Err(""),
            }
        }
        Ok(config)
    }
}
