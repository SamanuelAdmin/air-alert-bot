use std::collections::HashSet;
use hocon::{HoconLoader, Error};
use serde::Deserialize;


#[derive(Deserialize, Debug)]
pub struct Configs {
    pub default_templates_dir: String,
    pub template_name: String,
    pub requests_timeout: u16,
    pub tracked_regions: HashSet<u32>
}

pub static CONFIG_FILEPATH: &str = "./src/config.conf";

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

