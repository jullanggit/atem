use clap::Parser;
use cli::Cli;

mod cli;

// Its a LazyCell so from the view of any accessors it doesnt mutate
#[expect(clippy::declare_interior_mutable_const)]
const CONFIG_PATH: LazyCell<String> = LazyCell::new(|| {
    let home = std::env::var("HOME").expect("HOME is not set");
    format!("{home}/.config/meta")
});

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
