[中文版说明 »](./README.md)

# LeetCode Problems Project (Devcontainer Enabled)

This is a Rust project configured for containerized development using 
[Devcontainer](https://containers.dev/), ideal for use with VS Code or VSCodium
to practice Leetcode problem.

## 📦 Project Structure

```
.
├── .devcontainer/         # Devcontainer configuration
│   └── devcontainer.json  # Container image and plugin setup
├── .git/                  # Git repository metadata
├── .gitignore             # Git ignore rules
├── Cargo.toml             # Rust project configuration
├── rustfmt.toml           # Configuration for Rust code formatting
├── rust-toolchain.toml    # Configuration for Rust toolchain version
└── leetcode-**/
    └── src/lib.rs         # lib program with unit tests
```

## 🚀 Development Environment

- Container image: `mcr.microsoft.com/devcontainers/rust:latest`
- Default Shell: Fish shell (installed via devcontainer features)
- Extensions (from OpenVSX):
  - `rust-lang.rust-analyzer` — Rust language support
  - `vadimcn.vscode-lldb` — Debugging support
  - `tamasfe.even-better-toml` — TOML syntax and linting

## 🛠 How to Use

1. Install VS Code or [VSCodium](https://vscodium.com/)
2. Install the [Dev Containers extension](https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.remote-containers)
3. Open this project in VS Code and select **Reopen in Container**
4. Wait for the environment to build and launch

## 🧪 Run the Project

Once inside the container terminal, run:

```bash
cargo test --lib
```

You should see the test results of leetcode solutions in the project.
