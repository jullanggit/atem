use serde::Deserialize;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Config<'a> {
    #[serde(borrow)]
    managers: Vec<Manager<'a>>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Manager<'a> {
    /// Command for adding one/multiple item
    add: &'a str,
    /// Whether multiple packages can be added at once
    single_add: bool,
    /// Command for adding an item
    remove: &'a str,
    /// Command for getting a list of all installed items
    installed: &'a str,
    /// Command for upgrading all items
    upgrade: Option<&'a str>,
    /// The items the manager is supposed to have
    items: Vec<&'a str>,
}
