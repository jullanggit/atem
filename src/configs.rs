use serde::Deserialize;

#[derive(Deserialize)]
struct Config {
    managers: Vec<Manager>,
}

#[derive(Deserialize)]
struct Manager {
    /// Command for adding one/multiple item
    add: String,
    /// Whether multiple packages can be added at once
    mulit_add: bool,
    /// Command for adding an item
    remove: String,
    /// Command for upgrading all items
    upgrade: Option<String>,
    /// Command for getting a list of all installed items
    installed: Option<String>,
}
