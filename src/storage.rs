use lazy_static::lazy_static;
use std::path::{Path, PathBuf};

static APP_NAME: &str = "lgtmeow";

lazy_static! {
    pub static ref CONFIG_DIR: PathBuf = dirs::home_dir()
        .unwrap()
        .join(format!(".config/{}", APP_NAME));
}

pub fn ensure_dir(dir: &Path) -> Result<(), std::io::Error> {
    if !dir.exists() {
        std::fs::create_dir_all(dir)?;
    }
    Ok(())
}

pub fn clean_dir(dir: &Path) -> Result<(), std::io::Error> {
    if !dir.exists() {
        return Ok(());
    }
    std::fs::remove_dir_all(dir)?;
    Ok(())
}
