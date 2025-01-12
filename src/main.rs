// Dont want to slap this on every function that uses CONFIG_PATH
// Look at the reasoning there for why this is ok
#![expect(clippy::borrow_interior_mutable_const)]

use colored::{Color, Colorize};
use serde::Deserialize;
use std::{
    cell::LazyCell,
    collections::{HashMap, HashSet},
    fs,
    path::PathBuf,
    process::{Command, exit},
};
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
    /// Whether add/remove can should be passed only one item per invocation
    #[serde(default)]
    single_arg: bool,
    /// Command for adding an item
    remove: String,
    /// Command for getting a whitespace-separated list of all installed items
    list: String,
    /// Command for upgrading all items
    upgrade: Option<String>,

    /// The items the manager is supposed to have
    #[serde(default)]
    items: HashSet<String>,

    /// The items to add to the system
    #[serde(default)]
    items_to_add: Vec<String>,
    /// The items to remove from the system
    #[serde(default)]
    items_to_remove: Vec<String>,
}

fn main() {
    let mut managers = load_managers();

    load_configs(&mut managers);

    compute_and_print_add_remove(&mut managers);
}

/// Computes and prints the items to add and remove for each manager
fn compute_and_print_add_remove(managers: &mut HashMap<String, Manager>) {
    for (manager_name, manager) in managers {
        // Get system items
        let output = Command::new("fish").arg("-c").arg(&manager.list).output(); // TODO: Add setting for which shell to use

        let system_items = match output {
            Ok(output) => {
                if output.status.success() {
                    String::from_utf8(output.stdout).expect("Command output should be UTF-8")
                } else {
                    eprintln!(
                        "Command 'list' for manager {manager_name} failed with error: \n{}",
                        String::from_utf8_lossy(&output.stderr)
                    );
                    exit(1);
                }
            }
            Err(e) => {
                eprintln!("Failed to execute command 'list': {e}");
                exit(1);
            }
        };

        let system_items = system_items
            .split_whitespace()
            .map(str::to_string)
            .collect();

        manager.items_to_add = manager
            .items
            .difference(&system_items)
            .map(Clone::clone)
            .collect();
        manager.items_to_remove = system_items
            .difference(&manager.items)
            .map(Clone::clone)
            .collect();

        println!("{}:", manager_name.bold());
        for item_to_add in &manager.items_to_add {
            let colored_string = item_to_add.green();

            println!("{}", colored_string);
        }
        for item_to_remove in &manager.items_to_remove {
            let colored_string = item_to_remove.red();

            println!("{}", colored_string);
        }
    }
}

fn load_managers() -> HashMap<String, Manager> {
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
                .strip_suffix(".toml")
                .expect("File should be a toml")
                .into();

            (name, manager)
        })
        .collect()
}

/// Loads the config items for each manager
fn load_configs(managers: &mut HashMap<String, Manager>) {
    // Start at the current machine's config file
    let hostname = hostname::get()
        .expect("Failed to get hostname")
        .into_string()
        .expect("Hostname should be valid UTF-8");

    // The list of configs that should be parsed, gets continually extended when a new config file is imported
    // Paths are evaluated relative to CONFIG_PATH/configs/ and are appended with .toml
    let mut configs_to_parse: Vec<String> = vec![format!("../machines/{hostname}")]; // A bit hacky, but should resolve to CONFIG_PATH/machines/{hostname}.toml

    // Cant find a better way that allows pushing while iterating
    let mut i = 0;
    while let Some(config_file) = configs_to_parse.get(i) {
        let config_file = format!("{}/configs/{config_file}.toml", *CONFIG_PATH);

        // Load the config file
        let config_string = fs::read_to_string(config_file).expect("Config file should exist");

        // Deserialize it
        let config_table: Table =
            toml::from_str(&config_string).expect("Failed to deserialize config");

        for (manager_name, value) in config_table {
            // Create an iterator over the items of the entry
            value
                // Both arrays...
                .as_array()
                .into_iter()
                .flat_map(|vec| {
                    vec.iter()
                        .map(|value| value.as_str().expect("Item should be a string"))
                })
                // ...and single-value items are allowed
                .chain(value.as_str().into_iter())
                .for_each(|item| {
                    // Didnt find a way to push this up without code duplication
                    if manager_name == "imports" {
                        let item = item.into();
                        // Avoid infinite loop when two configs import each other
                        if !configs_to_parse.contains(&item) {
                            configs_to_parse.push(item);
                        }
                    } else {
                        // Add the items to the manager
                        managers
                            .get_mut(&manager_name)
                            .expect("Manager should exist")
                            .items
                            .insert(item.into());
                    }
                });
        }

        i += 1;
    }
}
