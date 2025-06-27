[English Version »](./README.en.md)

# LeetCode Problems Rust 项目（支持 Devcontainer）

这是一个使用 [Devcontainer](https://containers.dev/) 快速启动的 Rust 项目，适合在
 VS Code / VSCodium 中进行容器化开发，练习Leetcode算法题。

## 📦 项目结构

```
.
├── .devcontainer/         # Devcontainer 配置文件
│   └── devcontainer.json  # 容器镜像与插件等配置
├── .git/                  # Git 仓库元数据
├── .gitignore             # Git 忽略文件规则
├── Cargo.toml             # Rust 项目配置文件
├── rustfmt.toml           # Rust 格式化工具配置
├── rust-toolchain.toml    # Rust 工具链版本配置
└── leetcode-**/
    └── src/lib.rs         # 解题库的单元测试
```

## 🚀 开发环境说明

- 容器镜像：`mcr.microsoft.com/devcontainers/rust:latest`
- 默认 Shell：Fish shell（使用 devcontainer-features 安装）
- 编辑器插件（基于 OpenVSX）：
  - `rust-lang.rust-analyzer` — Rust 语言支持
  - `vadimcn.vscode-lldb` — 调试支持
  - `tamasfe.even-better-toml` — TOML 文件语法高亮与提示

## 🛠 如何使用

1. 安装 VS Code 或 [VSCodium](https://vscodium.com/)
2. 安装 [Dev Containers 扩展](https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.remote-containers)
3. 使用 VS Code 打开本项目，选择“使用 Dev Container 重新打开项目”
4. 等待环境自动构建并进入容器

## 🧪 运行项目

容器启动后，在终端运行以下命令：

```bash
cargo test --lib
```

输出应为项目内 Leetcode 题目的测试结果。