use std::convert::AsRef;
use std::io;
use std::path::{Path, PathBuf};
use std::fs::File;

pub const CONFIG_FILE_NAME: &str = ".spooky";

pub struct Config {
    ssh_run: String,
    rsync_up: String,
    rsync_down: String,
}

pub fn read_config_file(file: &mut File) -> Result<Config, String> {

    let mut contents = String::new();
    let bytes_read = file.read_to_string(&mut contents)?;
    assert_eq!(contents.len(), bytes_read);

    Ok(Config::from_str(&contents)?)
}

pub fn acquire_lock()  -> Result<File, String> {
    let current_dir = std::env::current_dir()?.canonicalize()?;
    let path = current_dir.join(CONFIG_FILE_NAME);

    let file = File::open(path)?;

    let lock_result = unsafe {
        libc::flock(file.as_raw_fd(), libc::LOCK_EX)
    };

    match lock_result {
        0 => {
            Ok(file)
        }
        _ => {
            Err("could not acquire file lock".into())
        }
    }
}

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
