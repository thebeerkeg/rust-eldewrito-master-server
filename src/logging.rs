use std::str::FromStr;
use env_logger::Builder;
use log::LevelFilter;

pub fn setup(log_level: &Option<String>) {
    let level = config_level_or_default(log_level);

    Builder::new().filter_level(level).init();
}

fn config_level_or_default(log_level: &Option<String>) -> LevelFilter {
    match log_level {
        None => LevelFilter::Info,
        Some(level) => LevelFilter::from_str(level).unwrap(),
    }
}
