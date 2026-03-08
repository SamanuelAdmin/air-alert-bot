use std::collections::HashSet;
use hocon::{HoconLoader};
use serde::Deserialize;


#[derive(Deserialize, Debug)]
pub struct Configs {
    pub show_updates: bool, // config to shows any updates, happend in views
    pub default_templates_dir: String,
    pub template_name: String,
    pub requests_timeout: u16,
    pub tracked_regions: HashSet<u32>,
    pub tracked_chats: HashSet<i64>,
    pub mut_first_start: bool // do not show anything with first parse cycle
}

pub static CONFIG_FILEPATH: &str = "config.conf";

pub fn get_configs() -> Configs {
    let configs: Configs = HoconLoader::new()
        .load_file(CONFIG_FILEPATH)
        .expect(
            &format!("Cannot load config file with filepath: {CONFIG_FILEPATH}")
        )
        .resolve()
        .expect(
            &format!(
                "Cannot parse configs from {CONFIG_FILEPATH}"
            )
        );
    configs
}

