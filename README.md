# SSHCTL

CLI tool for managing SSH connections easily and efficiently.

## Installation

```bash
cargo build --release
```

## Commands

### Add connection

```bash
sshctl connection add <name> 
```

Example:
```bash
sshctl connection add prod-server 
```

### Remove connection 
```bash
sshctl connection remove <name>
```

Example:
```bash
sshctl connection remove prod-server
```

## Configuration

Connections are saved in `~/.config/sshctl/connections.toml` (or OS equivalent).

Config file structure:
```toml
[connections.prod-server]
host = "192.168.1.100"
user = "admin"
port = 2222
description = "This is the prod server. OVH VPS"

[connections.dev-local]
host = "localhost"
user = "developer"
port = 22
```

## Development

### Main dependencies

- `clap` : CLI argument parsing
- `serde` + `toml` : TOML config serialization
- `dirs` : cross-platform config file paths
- `dialoguer` : interactive prompts
- `anyhow` : error handling
- `thiserror` : custom error types

### TODO v0.1.x

- [x] Command `sshctl connection add <NAME>`
- [x] Command `sshctl connection remove <NAME>`
- [ ] Command `sshctl connection edit <NAME>`
- [ ] Command `sshctl connection list`
- [ ] Command `sshctl <NAME>`

