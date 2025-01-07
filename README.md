A meta package manager inspired by rebos

## Features i want to implement:
  - Managers
    - User-defined
    - Ordering
    - Current state dynamically determined by managers
    - Maybe user-defined additional options
  - Configs
    - Modular
    - Different machines
  - No generation tracking, should be handled by git

## Managers i want to provide:
  - Cargo
  - Files
  - Rustup component & toolchain
  - Systemd services (normal & startup)
  - Paru
  - Patch

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
order.toml
settings.toml (recommended for global options, altough they can be set everywhere)
```
