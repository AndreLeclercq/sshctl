# 🔐 SSHCTL
[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/License-MIT-yellow.svg?style=for-the-badge)](https://opensource.org/licenses/MIT)
[![AUR SSHCTL](https://img.shields.io/aur/version/sshctl?style=for-the-badge&logo=arch-linux)](https://aur.archlinux.org/packages/sshctl)
[![AUR](https://img.shields.io/aur/version/sshctl-git?style=for-the-badge&logo=arch-linux&label=aur%20git)](https://aur.archlinux.org/packages/sshctl-git)
[![GitHub release](https://img.shields.io/github/release/AndreLeclercq/sshctl.svg?style=for-the-badge)](https://github.com/AndreLeclercq/sshctl/releases)

**SSHCTL** is a lightweight CLI tool designed to simplify and streamline SSH connection management. It allows you to store, edit, and quickly connect to your servers using intuitive commands. ⚡

**Note:** This project is in its early stages of development. While the core functionality is already usable Currently, the focus is on Linux compatibility, as it aligns with my personal workflow. However, support for **Windows and macOS** is planned if there is community interest—feel free to open an issue or discussions! 💬

## 📋 Versioning & Stability
- **Stable versions:** Tagged releases (e.g., v0.1.0, v0.2.0) are considered stable and production-ready ✅
- **Development version:** The `main` branch contains the latest development code and is **not considered stable** ⚠️

## 📦 Installation
### 🔧 From Source
```bash
cargo build --release
```
### 🐧 Arch Linux
- **Stable version:** [sshctl](https://aur.archlinux.org/packages/sshctl) (binary package from latest stable release)
- **Development version:** [sshctl-git](https://aur.archlinux.org/packages/sshctl-git) (builds from `main` branch)
```bash
# For stable version
yay -S sshctl
# For development version (main branch)
yay -S sshctl-git
```

## 🚀 Commands
```bash
# Connect 🔌
sshctl <connection-name> #ex: sshctl prod-server

# Add connection ➕
sshctl connection add <connection-name> #ex: sshctl connection add prod-server 

# Edit connection ✏️
sshctl connection edit <connection-name> #ex: sshctl connection edit prod-server

# Remove connection ❌
sshctl connection remove <connection-name> #ex: sshctl connection remove prod-server

# Show connection 👀
sshctl connection show <connection-name> #ex: sshctl connection show prod-server

# List connections 📝
sshctl connection list
```

## ⚙️ Configuration
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

## 👨‍💻 Development
### 📚 Main dependencies
- `clap` : CLI argument parsing
- `serde` + `toml` : TOML config serialization
- `dirs` : cross-platform config file paths
- `dialoguer` : interactive prompts
- `anyhow` : error handling
- `thiserror` : custom error types

### 🏗️ Architecture
Built with performance-first mindset using Rust:
- **Zero-copy parsing** with serde
- **Minimal allocations** for config handling
- **Cross-platform** compatibility via dirs crate
- **Interactive UX** with dialoguer

### 📋 TODO v0.2.x
- [ ] Manage ssh keys
- [ ] Improve connection commands

---
*Part of my transition journey from Fullstack Development to AI/Data Engineering, showcasing Rust for performance-critical applications.* 🚀
