## About
A meta package manager inspired by [rebos](https://gitlab.com/Oglo12/rebos)

Mostly made for the same reasons as [rebos](https://gitlab.com/Oglo12/rebos), but with a different approach to determining the system's state:
Atem has no generation tracking. Instead system state is determined using each managers 'list' command.
This solves one of the main problems I've had when using rebos: the 'built' generation and the actual system getting out of sync.
While this does increase the burden the individual managers have to bear, [all of the ones I personally use](https://github.com/jullanggit/atem-configs) already have a built-in way to get their items

See [my atem configs](https://github.com/jullanggit/atem-configs) for a reference config

## Naming
- Meta backwards is atem
- Also atem is breath in german, and i like the idea that it breathes life into your system

## Ideas, contributing, bugs etc
- Atem is still very much under active development, so if you have any ideas / feature requests or encounter any bugs, please open an issue or a PR

## Features
  - Managers
    - User-defined
    - Ordering
    - System state dynamically determined by managers
    - Maybe user-defined additional options
  - Configs
    - Modular
    - Different machines
  - No generation tracking, should be handled by git

## Managers
- Each manager is a file in the managers/ subdirectory
- The ordering of the managers is defined in `manager_order`
### Commands
- add: command for adding one or multiple items
  - <item> will be replaced by a single item, <items> by all of them, separated by spaces
- remove: command for removing one or multiple items (same formatting as above)
- list: command for listing all installed items, separated by newlines
  - can also use the same formatting as above
    - command will be passed all items in the configuration
  - used for determining the system state
- upgrade: command for upgrading all items (does not receive any items from atem)
### Options
- remove_then_add: first remove then add items
- items_separator: The separator to use when filling in the <items> in format commands. Defaults to space
### Implemented Managers
Can be found in [my atem configs](https://github.com/jullanggit/atem-configs)

## Configs
- Each machine has a "root" config file, found at machines/{machine name}.toml
- Further config files are located in the configs/ subdirectory, and can be imported by file name using `imports = ["foo", "bar"]`
- These config files can also import other config files
- Specifying items is done by using `{manager name} = ["foo", "bar"]` in any config file
- All arrays can also be replaced by single-item strings

## File structure
```
~/.config/atem/
├── configs/
│   ├── common.toml
│   └── rust.toml
├── machines/
│   ├── laptop.toml
│   └── pc.toml
├── managers/
│   ├── cargo.toml
│   ├── files.toml
│   ├── rustup_component.toml
│   ├── rustup_toolchain.toml
│   ├── service.toml
│   ├── service_startup.toml
│   └── pacman.toml
├── files/
│   ├── common/
│   ├── laptop/
│   ├── pc/
└── manager_order
```

## Installing
`cargo +nightly install atem`

## Usage
```
Usage: atem [OPTIONS] <COMMAND>

Commands:
  build    Build the current configuration
  diff     Print the difference between the system and the config
  upgrade  Upgrade all managers
  help     Print this message or the help of the given subcommand(s)

Options:
  -m, --managers <MANAGERS>  The managers to run the command for
  -n, --non-specified        Run all non-specified managers
  -h, --help                 Print help
  -V, --version              Print version
```
## TODO
- add install-manager: optionally run a script to install the manager

![brainmademark](https://brainmade.org/black-logo.svg)
