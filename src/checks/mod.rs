pub mod vars;
use dirs::home_dir;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use vars::{CONFIG_PATH, FILES};

fn create_folder_and_necessary_files(config_path: &Path) {
    // Ensure base config folder exists
    if let Err(e) = fs::create_dir_all(config_path) {
        eprintln!("Failed to create config folder: {}", e);
        return;
    }

    for relative in FILES.iter() {
        let full_path = config_path.join(relative.trim_start_matches('/'));

        if let Some(parent) = full_path.parent() {
            if let Err(e) = fs::create_dir_all(parent) {
                eprintln!("Failed to create directory {:?}: {}", parent, e);
                continue;
            }
        }

        if !full_path.exists() {
            if let Err(e) = fs::File::create(&full_path).and_then(|mut f| f.write_all(b"")) {
                eprintln!("Failed to create file {:?}: {}", full_path, e);
            }
        }
    }
}

fn config_files_exist() {
    let config_path = expand_tilde(CONFIG_PATH);

    if !config_path.exists() {
        eprintln!("Failed to find Config Folder creating.....");
        match std::fs::create_dir(&config_path) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Failed to create Config Directory {:?}", e);
            }
        };
    }

    create_folder_and_necessary_files(&config_path);
}

pub fn expand_tilde(path: &str) -> PathBuf {
    if let Some(stripped) = path.strip_prefix("~") {
        if let Some(home) = home_dir() {
            return home.join(stripped.trim_start_matches('/'));
        }
    }
    PathBuf::from(path)
}

pub fn first_start() {
    config_files_exist();
}
