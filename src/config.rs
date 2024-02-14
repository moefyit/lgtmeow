use super::storage::{ensure_dir, CONFIG_DIR};
use serde::{Deserialize, Serialize};
use std::{io::Write, path::PathBuf};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub image_size: u32,
    pub emoji_codepoint_pairs: Vec<(String, String)>,
}

impl Config {
    pub fn new(image_size: u32, emoji_codepoint_pairs: Vec<(String, String)>) -> Config {
        Config {
            image_size,
            emoji_codepoint_pairs,
        }
    }

    #[inline]
    fn path() -> PathBuf {
        CONFIG_DIR.join("config.toml")
    }

    pub fn save(&self) -> Result<(), std::io::Error> {
        ensure_dir(&CONFIG_DIR)?;
        let mut file = std::fs::File::create(Config::path().as_path())?;
        file.write_all(
            toml::to_string_pretty(self)
                .expect("Could not serialize config")
                .as_bytes(),
        )?;
        Ok(())
    }

    pub fn load() -> Result<Config, std::io::Error> {
        let config: Config = toml::from_str(&std::fs::read_to_string(Config::path().as_path())?)
            .expect("Could not deserialize config");
        Ok(config)
    }

    pub fn exists() -> bool {
        std::path::Path::new(&Config::path()).exists()
    }
}
