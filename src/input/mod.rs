use chrono::format::Item;

#[derive(Clone, Copy)]
enum TimeUnit {
    Minute(),
    Hour(),
    Day(),
    Week(),
    Month(),
    Year(),
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
    frequency: TimeUnit,            // Единица измерения частоты создания
    removal_frequency: TimeUnit,    // Единица измерения частоты бекапов
    frequency_time: u64, // Частота создания бекапа
    removal_frequency_time: u64, // Частота удаления бекапа
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
        frequency_time: u64,
        removal_frequency_time: u64,
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
            frequency_time,
            removal_frequency_time,
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
    pub fn get_frequency_time(&self) -> &u64 {
        &self.frequency_time
    }
    pub fn get_removal_frequency_time(&self) -> &u64 {
        &self.removal_frequency_time
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
    pub fn get_help(&self) -> bool {
        self.help
    }
    //setter
    pub fn set_frequency(&mut self, frequency: String) {
        match frequency.as_str() {
            "m" => self.frequency = TimeUnit::Minute(),
            "h" => self.frequency = TimeUnit::Hour(),
            "d" => self.frequency = TimeUnit::Day(),
            "w" => self.frequency = TimeUnit::Week(),
            "M" => self.frequency = TimeUnit::Month(),
            "y" => self.frequency = TimeUnit::Year(),
            _ => (),
        }
    }
    pub fn set_removal_frequency(&mut self, frequency: String) {
        match frequency.as_str() {
            "m" => self.removal_frequency = TimeUnit::Minute(),
            "h" => self.removal_frequency = TimeUnit::Hour(),
            "d" => self.removal_frequency = TimeUnit::Day(),
            "w" => self.removal_frequency = TimeUnit::Week(),
            "M" => self.removal_frequency = TimeUnit::Month(),
            "y" => self.removal_frequency = TimeUnit::Year(),
            _ => (),
        }
    }
    pub fn set_frequency_time(&mut self, frequency_time: String) {
        self.frequency_time = frequency_time.parse().unwrap();
    }
    pub fn set_removal_frequency_time(&mut self, frequency_time: String) {
        self.removal_frequency_time = frequency_time.parse().unwrap();
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
        self.recursive = recursive
    }
    pub fn set_incremental_method(&mut self, incremental_method: String) {
        match incremental_method.as_str() {
               "Full" => self.incremental = IncrementalMethod::Full,
               "Incr" => self.incremental = IncrementalMethod::Incremental,
               "Diff" => self.incremental = IncrementalMethod::Differential,
               "None" => self.incremental = IncrementalMethod::None,
            _ => ()
        }
    }
    pub fn set_compression_method(&mut self, compression_method: String) {
        match compression_method.as_str() {
            "Gzip" => self.compression = CompressionMethod::Gzip,
            "Zstd" => self.compression = CompressionMethod::Zstd,
            "None" => self.compression = CompressionMethod::None,
            _ => ()
        }
    }
    pub fn set_list(&mut self, list: bool) {
        self.list = list
    }
    pub fn set_verify(&mut self, verify: bool) {
        self.verify = verify;
    }
    pub fn set_help(&mut self, help: bool) {
        self.help = help;
    }

    fn parser_function(
        &mut self,
        args: &mut dyn Iterator<Item = String>,
        parser: fn(&mut Self, String),
        error: &'static str,
    ) -> Result<(), &'static str> {
        match args.next() {
            Some(arg) => {
                parser(self, arg);
                Ok(())
            }
            None => Err(error),
        }
    }

    fn parse_no_element_function(
        &mut self,
        parser: fn(&mut Self, bool)
    ) {
        parser(self, true)
    }

    pub fn env2conf(
        args: &mut dyn Iterator<Item = String>,
    ) -> Result<Config, &'static str> {
        // make basik config
        let mut config = Config::new(
            TimeUnit::Minute(),
            TimeUnit::Minute(),
            1,
            1,
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

        //skip unneeded paramete
        args.next();

        while let Some(arg) = args.next() {
            match arg.as_str() {
                "--frequency" | "-f" => config.parser_function(
                    args, 
                    Config::set_removal_frequency_time,
                    "[ERROR]:no frequency time set. Default frequency time is 1 minute. You can change frequency time unit with help '-F'"
                )?,
                "--removal-frequency" | "-r" => config.parser_function(
                    args,
                    Config::set_removal_frequency_time,
                    "[ERROR]:no removal frequency time set. Default removal frequency time is 1 minute. You can change frequency time unit with help '-R'."
                )?,
                "--frequency-unit" | "-u" => config.parser_function(
                    args,
                    Config::set_frequency,
                    "[ERROR]:no frequency unit set. Default frequency unit is minute. You can choose:\nminute 'm'\nhour 'h'\nday 'd'\nweek 'w'\nmonth 'M'\nyear 'y'"
                )?,
                "--removal-frequency-unit" | "-U" => config.parser_function(
                    args,
                    Config::set_removal_frequency,
                    "[ERROR]:no removal frequency unit set. Default removal frequency unit is minute. You can choose:\nminute 'm'\nhour 'h'\nday 'd'\nweek 'w'\nmonth 'M'\nyear 'y'"
                )?,
                "--source" | "-s" => config.parser_function(
                    args,
                    Config::set_source_path, 
                    "[ERROR]:no source path set. Default source path is './'. You can add several different directories."
                )?,
                "--exclude" | "-e" => config.parser_function(
                    args,
                    Config::set_exclude_path,
                    "[ERROR]:no exclude path set. Default exclude path is none. You can add several different directories."
                )?,
                "--backup" | "-b" => config.parser_function(
                    args,
                    Config::set_backup_path,
                    "[ERROR]:no backup path set. Default backup path is '../backup'. You can add several different directories."
                )?,
                "--recursive" | "-R" => config.parse_no_element_function(
                    Config::set_recursive
                ),
                "--incremential_method" | "-i" => config.parser_function(
                    args, 
                    Config::set_incremental_method, 
                    "[ERROR]:no incremential method set. Default incremential method is gzip. You can choose:\nfull\nincremental\ndifferential\nnone"
                )?,
                "--compression_method" | "-c" => config.parser_function(
                    args, 
                    Config::set_compression_method, 
                    "[ERROR]:no compression method set. Default compression method is none. You can choose:\ngzip\nzstd\nnone"
                )?,
                "--list" | "-l" => config.parse_no_element_function(
                    Config::set_list
                ),
                "--verify" | "-v" => config.parse_no_element_function(
                    Config::set_verify
                ),
                "--help" | "-h" => config.parse_no_element_function(Config::set_help),
                _ => return Err(""),
            }
        }
        Ok(config)
    }
}
