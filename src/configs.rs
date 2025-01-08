use std::{fs, path::PathBuf};

use serde::Deserialize;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Config {
    managers: Vec<Manager>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Manager {
    /// Command for adding one/multiple item
    add: String,
    /// Whether multiple packages can be added at once
    #[serde(default)]
    single_add: bool,
    /// Command for adding an item
    remove: String,
    /// Command for getting a list of all installed items
    installed: String,
    /// Command for upgrading all items
    upgrade: Option<String>,
    /// The items the manager is supposed to have
    #[serde(default)]
    items: Vec<String>,
    /// The name of the manager
    #[serde(default)]
    name: String,
}

fn load_managers() -> Config {
    let home = std::env::var("HOME").expect("HOME is not set");
    let manager_path = PathBuf::from(format!("{home}/.config/meta/managers"));

    Config {
        managers: manager_path
            .read_dir()
            .expect("Failed to read manager dir")
            .map(|manager_file| manager_file.unwrap())
            .map(|manager_file| {
                let manager_string =
                    fs::read_to_string(manager_file.path()).expect("Failed to read manager file");
                let mut manager: Manager =
                    toml::from_str(&manager_string).expect("Failed to deserialize manager");
                manager.name = manager_file
                    .file_name()
                    .to_str()
                    .expect("Failed to get manager name")
                    .into();
                manager
            })
            .collect(),
    }
}
