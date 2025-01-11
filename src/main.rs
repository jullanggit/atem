// Dont want to slap this on every function that uses CONFIG_PATH
// Look at the reasoning there for why this is ok
#![expect(clippy::borrow_interior_mutable_const)]

use serde::Deserialize;
use std::{cell::LazyCell, collections::HashMap, fs, path::PathBuf, process::exit};
use toml::Table;

mod cli;

// Its a LazyCell so from the view of any accessors it doesnt mutate
#[expect(clippy::declare_interior_mutable_const)]
const CONFIG_PATH: LazyCell<String> = LazyCell::new(|| {
    let home = std::env::var("HOME").expect("HOME is not set");
    format!("{home}/.config/meta")
});

#[derive(Debug, Deserialize)]
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
    list: String,
    /// Command for checking which of the current configs items are actually installed
    check: String,
    /// Command for upgrading all items
    upgrade: Option<String>,
    /// The items the manager is supposed to have
    #[serde(default)]
    items: Vec<String>,
}

fn main() {
    let mut managers = get_managers();

    let configs_path = PathBuf::from(format!("{}/configs", *CONFIG_PATH));
    configs_path
        .read_dir()
        .expect("Failed to read configs dir")
        .for_each(|config_file| {
            let config_file = config_file.unwrap();

            let config_string =
                fs::read_to_string(config_file.path()).expect("Failed to read config file");

            let config_table: Table =
                toml::from_str(&config_string).expect("Failed to deserialize config");

            for (manager_name, value) in config_table {
                let manager = managers
                    .get_mut(&manager_name)
                    .expect("Manager should exist");

                if let Some(array) = value.as_array() {
                    for item in array {
                        manager
                            .items
                            .push(item.as_str().expect("Item should be a string").to_string());
                    }
                // We allow single-item entries
                } else if let Some(item) = value.as_str() {
                    manager.items.push(item.to_string());
                } else {
                    eprintln!("Items should be either a single string or an array of string");
                    exit(1);
                }
            }
        });
}

fn get_managers() -> HashMap<String, Manager> {
    let manager_path = PathBuf::from(format!("{}/managers", *CONFIG_PATH));

    manager_path
        .read_dir()
        .expect("Failed to read manager dir")
        .map(|manager_file| {
            let manager_file = manager_file.unwrap();

            let manager_string =
                fs::read_to_string(manager_file.path()).expect("Failed to read manager file");
            let manager: Manager =
                toml::from_str(&manager_string).expect("Failed to deserialize manager");

            let name = manager_file
                .file_name()
                .to_str()
                .expect("Failed to get manager name")
                .into();

            (name, manager)
        })
        .collect()
}
