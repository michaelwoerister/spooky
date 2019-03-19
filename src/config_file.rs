use std::convert::AsRef;
use std::io;
use std::path::{Path, PathBuf};

pub const CONFIG_FILE_NAME: &str = ".spooky";

pub fn find_config_dir(current_dir: impl AsRef<Path>) -> io::Result<PathBuf> {
    let current_dir = current_dir.as_ref();
    let config_file_path = current_dir.join(CONFIG_FILE_NAME);

    if config_file_path.exists() {
        Ok(current_dir.to_path_buf())
    } else {
        if let Some(parent) = current_dir.parent() {
            find_config_dir(parent)
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "not found",
            ))
        }
    }
}

pub fn switch_to_config_file_dir() -> std::io::Result<()> {
    let current_dir = std::env::current_dir()?.canonicalize()?;
    let config_dir = find_config_dir(&current_dir)?;
    std::env::set_current_dir(config_dir)
}
