
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

## Managers to implement (not yet in [meta-managers](https://github.com/jullanggit/meta-managers))
  - Patch
  - Copy (for stuff that doesnt like being a symlink)

## File structure
```
configs/
└── config.toml
machines/
├── laptop/
│   └── config.toml
└── pc/
    └── config.toml
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
settings.toml
```
