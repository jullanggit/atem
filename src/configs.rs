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
}
