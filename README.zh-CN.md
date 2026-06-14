# konachan-yew

<p align="center">
  <img src="./screenshot.png" alt="Konachan Yew 截图" width="800" />
</p>

<p align="center">
  <a href="https://www.rust-lang.org/"><img src="https://img.shields.io/badge/Rust-1.94+-dea584?style=flat&logo=rust" alt="Rust" /></a>
  <a href="https://yew.rs/"><img src="https://img.shields.io/badge/Yew-0.23.0-88b86e?style=flat" alt="Yew" /></a>
  <a href="https://github.com/lf-wxp/konachan-yew"><img src="https://img.shields.io/github/license/lf-wxp/konachan-yew" alt="许可证" /></a>
  <a href="https://github.com/lf-wxp/konachan-yew/releases"><img src="https://img.shields.io/github/v/release/lf-wxp/konachan-yew?include_prereleases" alt="发布版本" /></a>
</p>

<p align="center">
  一个现代、快速、美观的 <a href="https://konachan.net/">Konachan</a> 网页前端，使用 Rust 和 WebAssembly 基于 <a href="https://yew.rs/">Yew</a> 框架构建。
</p>

<p align="center">
  <a href="./README.md">English Documentation</a>
</p>

---

## 📖 项目概述

**konachan-yew** 是一个高性能的单页应用（SPA），为 Konachan 图片板提供增强的浏览体验。借助 Rust 和 WebAssembly 的强大性能，带来闪电般的运行速度和现代化的用户界面。

本仓库仅包含**网页前端**。提供图片数据的后端 API 服务器请访问 [konachan-api](https://github.com/lf-wxp/konachan-api)。

---

## ✨ 功能特性

| 功能 | 描述 |
|-------|------|
| 🖼️ **瀑布流布局** | Pinterest 风格的瀑布流布局，优化图片浏览体验 |
| 📥 **下载管理** | 内置下载队列，支持进度跟踪 |
| 🎨 **动态壁纸** | 设置图片为动态壁纸 |
| 🌍 **多语言支持** | 支持多种语言（英文和中文） |
| 🎭 **安全模式** | 可配置的内容过滤，安全浏览 |
| 🌓 **主题支持** | 可自定义 UI 主题颜色 |
| 🔍 **高级搜索** | 强大的基于标签的搜索和过滤功能 |
| ⚡ **极速性能** | 基于 Rust/WASM，提供接近原生的性能 |
| 📱 **响应式设计** | 在桌面和移动设备上无缝运行 |
| 🐳 **Docker 支持** | 一行命令即可使用 Docker 部署 |

---

## 🛠️ 技术栈

- **框架**: [Yew 0.23.0](https://yew.rs/) - 使用 WebAssembly 创建多线程前端 Web 应用的现代 Rust 框架
- **语言**: Rust (Edition 2024)
- **样式**: [Stylist](https://github.com/futursolo/stylist-rs) - 与 Yew 集成的 CSS-in-Rust
- **状态管理**: [Bounce](https://github.com/yewstack/bounce) - Yew 的状态管理
- **WASM 绑定**: 
  - [wasm-bindgen](https://github.com/rustwasm/wasm-bindgen) - WASM 和 JavaScript 之间的高级交互
  - [gloo](https://github.com/rustwasm/gloo) - WASM 友好工具集合
- **构建工具**: [Trunk](https://trunkrs.dev/) - WASM Web 应用打包器
- **桌面端（可选）**: [Tauri](https://tauri.app/) - 构建更小、更快、更安全的桌面应用程序

---

## 🚀 快速开始

### 前置要求

- [Rust](https://www.rust-lang.org/) 工具链（1.94+）
- `wasm32-unknown-unknown` 编译目标: 
  ```bash
  rustup target add wasm32-unknown-unknown
  ```
- [Trunk](https://trunkrs.dev/): 
  ```bash
  cargo install trunk@0.21.14
  ```
- （可选）[cargo-make](https://github.com/sagiegurari/cargo-make): 
  ```bash
  cargo install cargo-make
  ```

### 功能标志

| 标志 | 描述 |
|------|------|
| `fake` | 使用模拟数据开发，无需后端 |
| `safe` | 安全模式构建，输出到 `dist_safe` 目录 |
| `web` | 生产环境网页构建，带 API 代理 |
| `tauri` | 启用 Tauri 集成以构建桌面应用 |

### 开发

#### 直接使用 Trunk

```bash
# 使用模拟数据的开发服务器（无需后端），端口 8888
trunk serve --features fake

# 使用 web 功能的开发服务器（需要后端运行在 localhost:8000）
trunk serve --features web

# 使用安全模式配置的开发服务器
trunk serve --config Trunk.safe.toml
```

#### 使用 cargo-make（推荐）

```bash
# 使用模拟数据开发
cargo make dev-fake

# 使用 web 功能开发（需要后端）
cargo make dev-web

# 使用安全模式开发
cargo make dev-safe
```

开发服务器默认运行在**端口 8888**。对 `/api/` 的 API 请求将被代理到 `http://localhost:8000`。

### 可用任务

```bash
# 格式化代码
cargo make format

# 运行 clippy 代码检查
cargo make clippy

# 格式化 + clippy 检查
cargo make check

# 运行测试
cargo make test

# 清理构建产物
cargo make clean
```

---

## 📦 生产构建

### 网页构建

```bash
trunk build --release --features web
```

输出的静态文件将在 `dist/` 目录中。

### Docker 部署

#### 构建 Docker 镜像

```bash
# 使用 cargo-make
cargo make docker-build

# 或直接运行
docker build -t konachan-yew:latest .
```

#### 运行容器

```bash
# 使用 cargo-make（映射到端口 8080）
cargo make docker-run

# 或直接运行
docker run --rm -p 8080:80 --name konachan-yew konachan-yew:latest
```

#### Docker 架构

Dockerfile 使用**多阶段构建**：

1. **构建阶段** (`rust:1.94-bookworm`): 使用 Trunk 编译 Rust/WASM
2. **运行阶段** (`nginx:1.27-bookworm`): 使用 nginx 提供静态文件服务

nginx 配置包括：
- 对 WASM、JS、CSS 和 SVG 启用 Gzip 压缩
- 静态资源激进缓存（1 年，不可变）
- SPA 回退（所有路由都返回 `index.html`）
- 反向代理：`/api/*` → `http://backend:8000/`

#### 后端服务设置

nginx 配置将对 `/api/` 的请求代理到 `http://backend:8000/`。使用 **Docker Compose** 来连接前端和后端：

```yaml
version: "3.8"

services:
  frontend:
    build: .
    ports:
      - "8080:80"
    depends_on:
      - backend

  backend:
    image: your-backend-image:latest
    # 或者：build: ./path-to-backend
    ports:
      - "8000:8000"
```

或者手动创建 Docker 网络：

```bash
docker network create app-net
docker run -d --name backend --network app-net your-backend-image
docker run -d --name frontend --network app-net -p 8080:80 konachan-yew:latest
```

#### 停止容器

```bash
# 使用 cargo-make
cargo make docker-stop

# 或直接运行
docker stop konachan-yew
```

---

## 📁 项目结构

```
konachan-yew/
├── src/
│   ├── components/     # Yew 组件（导航、列表、搜索等）
│   ├── hook/           # 自定义 Yew hooks
│   ├── model/          # 数据模型和结构体
│   ├── store/          # 状态管理（Bounce）
│   ├── utils/          # 工具函数和国际化的实现
│   └── main.rs         # 应用程序入口点
├── static/             # 静态资源
│   ├── css/            # 样式表
│   ├── font/           # 字体文件
│   ├── image/          # 图片资源
│   ├── mock/           # 开发用的模拟数据
│   └── script/         # JavaScript 工具
├── dist/               # 构建输出（生产环境）
├── Cargo.toml          # Rust 依赖
├── Trunk.toml          # Trunk 配置
├── Trunk.safe.toml     # 安全模式配置
├── Dockerfile          # Docker 多阶段构建
└── Makefile.toml       # cargo-make 任务
```

---

## 🤝 贡献指南

欢迎贡献！请随时提交 Pull Request。对于重大更改，请先打开 Issue 讨论您想要更改的内容。

### 贡献步骤

1. Fork 本仓库
2. 创建您的功能分支（`git checkout -b feature/AmazingFeature`）
3. 提交您的更改（`git commit -m 'Add some AmazingFeature'`）
4. 推送到分支（`git push origin feature/AmazingFeature`）
5. 打开 Pull Request

### 代码风格

本项目遵循：
- [Rust Style Guide](https://doc.rust-lang.org/1.0.0/style/README.html)
- [Airbnb CSS Style Guide](https://github.com/airbnb/css)
- 所有代码使用 `rustfmt` 格式化，并使用 `clippy` 进行代码检查

---

## 📝 许可证

本项目采用 MIT 许可证 - 详情请参阅 [LICENSE](LICENSE) 文件。

---

## 🔗 相关项目

| 项目 | 描述 |
|-------|------|
| [konachan-api](https://github.com/lf-wxp/konachan-api) | Konachan 图片数据的后端 API 服务器 |
| [konachan-tauri](https://github.com/lf-wxp/konachan-tauri) | 使用 Tauri 框架的桌面应用版本 |

---

## 🙏 致谢

- [Konachan](https://konachan.net/) 提供图片板 API
- [Yew](https://yew.rs/) 团队提供出色的 Rust/WASM 框架
- 所有为这个项目提供帮助的贡献者

---

## 📧 联系方式

- GitHub Issues: [报告错误或请求功能](https://github.com/lf-wxp/konachan-yew/issues)
- 项目链接: [https://github.com/lf-wxp/konachan-yew](https://github.com/lf-wxp/konachan-yew)

---

<p align="center">
  使用 Rust + WebAssembly ❤️ 制作
</p>
