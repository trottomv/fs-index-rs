use config::{Config, ConfigError, File};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub debug: bool,
    pub ignore_patterns: Vec<String>,
    pub projects: Vec<String>,
    pub key_words: Vec<String>,
    pub output_filename: String,
}

#[cfg(not(test))]
const CONFIG_FILE_PATH: &str = "./config/config.toml";
#[cfg(test)]
const CONFIG_FILE_PATH: &str = "./config/tests.toml";

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let _settings = Config::builder()
            .add_source(File::with_name(CONFIG_FILE_PATH))
            .build()
            .unwrap();

        _settings.try_deserialize()
    }
}

#[cfg(test)]
mod tests {

    use crate::settings;

    #[test]
    fn test_settings() {
        let settings: settings::Settings = settings::Settings::new().unwrap();
        assert_eq!(settings.projects, ["myproject"]);
        assert_eq!(settings.output_filename, "fs_index.json");
        assert_eq!(settings.key_words, ["backend", "docker"]);
        assert_eq!(settings.ignore_patterns.len(), 0);
    }
}
