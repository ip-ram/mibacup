pub const HELP: &str = r#"
Usage: mibackup [OPTIONS]

Options:
    -f, --frequency <TIME>
        Set the removal frequency time.
        Default: 1 minute.

    -r, --removal-frequency <TIME>
        Set the removal frequency time.
        Default: 1 minute.

    -u, --frequency-unit <UNIT>
        Set the frequency time unit.
        Default: minute.
        Available units:
            m  minute
            h  hour
            d  day
            w  week
            M  month
            y  year

    -U, --removal-frequency-unit <UNIT>
        Set the removal frequency unit.
        Default: minute.
        Available units:
            m  minute
            h  hour
            d  day
            w  week
            M  month
            y  year

    -s, --source <PATH>
        Set the source directory.
        Default: ./.
        Can be specified multiple times.

    -e, --exclude <PATH>
        Exclude a directory from backup.
        Default: none.
        Can be specified multiple times.

    -b, --backup <PATH>
        Set the backup directory.
        Default: ../backup.
        Can be specified multiple times.

    -R, --recursive
        Enable recursive backup.

    -i, --incremential_method <METHOD>
        Set the incremental backup method.
        Default: gzip.
        Available methods:
            full
            incremental
            differential
            none

    -c, --compression_method <METHOD>
        Set the compression method.
        Default: none.
        Available methods:
            gzip
            zstd
            none

    -l, --list
        List backup files.

    -v, --verify
        Verify backup.

    -h, --help
        Show this help message.
"#;
