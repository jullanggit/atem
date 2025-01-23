
## About
A meta package manager inspired by [rebos](https://gitlab.com/Oglo12/rebos)

Mostly made for the same reasons as [rebos](https://gitlab.com/Oglo12/rebos), but with a different approach to determining the system's state:
Meta has no generation tracking. Instead system state is determined using each managers 'list' command.
This solves one of the main problems I've had when using rebos: the 'built' generation and the actual system getting out of sync.
While this does increase the burden the individual managers have to bear, [all of the ones I personally use](https://github.com/jullanggit/meta-managers) already have a built-in way to get their items

## Features i want to implement:
  - Managers
    - User-defined
    - Ordering
    - System state *always* dynamically determined by managers
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
- upgrade: command for upgrading all items (does not receive any items for meta)
- list: command for listing all installed items, separated by newlines
  - used for determining the system state
### Options
- remove_then_add: first remove then add items
### Implemented Managers
Can be found in [meta-managers](https://github.com/jullanggit/meta-managers)

## Configs
- Each machine has a "root" config file, found at machines/{machine name}.toml
- Further config files are located in the configs/ subdirectory, and can be imported by file name using `imports = ["foo", "bar"]`
- These config files can also import other config files
- Specifying items is done by using `{manager name} = ["foo", "bar"]` in any config file
- All arrays can also be replaced by single-item strings

## File structure
```
configs/
├── common.toml
└── rust.toml
machines/
├── laptop.toml
└── pc.toml
managers/
├── cargo.toml
├── files.toml
├── rustup_component.toml
├── rustup_toolchain.toml
├── service.toml
├── service_startup.toml
└── pacman.toml
files/
├── common/
├── laptop/
├── pc/
manager_order
```
