# SSHCTL

CLI tool for managing SSH connections easily and efficiently.

## Installation

```bash
cargo build --release
```

## Commands

### Add connection

```bash
sshctl connection add <NAME> 
```

Example:
```bash
sshctl connection add prod-server 
```
### Edit connection
```bash
sshctl connection edit <NAME>
```

Example:
```bash
sshctl connection edit prod-server
```

### Remove connection 
```bash
sshctl connection remove <NAME>
```

Example:
```bash
sshctl connection remove prod-server
```

### Show connection
```bash
sshctl connection show <NAME>
```

Example:
```bash
sshctl connection show prod-server
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
- [x] Command `sshctl connection edit <NAME>`
- [x] Command `sshctl connection show <NAME>`
- [ ] Command `sshctl connection list`
- [ ] Command `sshctl <NAME>`

