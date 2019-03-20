
use spooky::{config_file, ssh, rsync};

fn main() {

    config_file::switch_to_config_file_dir();

    // This file must be kept around in order for the lock to be held
    let mut locked_file = config_file::acquire_lock();

    let config = config_file::read_config_file(&mut locked_file).unwrap();

    rsync::run(&config.rsync_up).unwrap();
    ssh::run(&config.ssh_run).unwrap();
    rsync::run(&config.rsync_down).unwrap();
}
