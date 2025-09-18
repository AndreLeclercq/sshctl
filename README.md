# SSHCTL

**SSHCTL** is a lightweight CLI tool designed to simplify and streamline SSH connection management. It allows you to store, edit, and quickly connect to your servers using intuitive commands.

**Note:** This project is in its early stages of development. While the core functionality is already usable, stable releases are coming soon. Currently, the focus is on Linux compatibility, as it aligns with my personal workflow. However, support for **Windows and macOS** is planned if there is community interest—feel free to open an issue or discussions!

## Versioning & Stability

- **Stable versions:** Tagged releases (e.g., v0.1.0, v0.2.0) are considered stable and production-ready
- **Development version:** The `main` branch contains the latest development code and is **not considered stable**

## Installation

### From Source
```bash
cargo build --release
```

### Arch Linux
- **Stable version:** [sshctl](https://aur.archlinux.org/packages/sshctl) (binary package from latest stable release)
- **Development version:** [sshctl-git](https://aur.archlinux.org/packages/sshctl-git) (builds from `main` branch)

```bash
# For stable version
yay -S sshctl

# For development version (main branch)
yay -S sshctl-git
```

## Commands

### Connect 
```bash
sshctl <connection-name>
```
Example:
```bash
sshctl prod-server
```

### Add connection
```bash
sshctl connection add <connection-name>
```
Example:
```bash
sshctl connection add prod-server 
```

### Edit connection
```bash
sshctl connection edit <connection-name>
```
Example:
```bash
sshctl connection edit prod-server
```

### Remove connection 
```bash
sshctl connection remove <connection-name>
```
Example:
```bash
sshctl connection remove prod-server
```

### Show connection
```bash
sshctl connection show <connection-name>
```
Example:
```bash
sshctl connection show prod-server
```

### List connections
```bash
sshctl connection list
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

### TODO v0.2.x
- [ ] Manage ssh keys
- [ ] Improve connection commands
