# konachan-yew

<p align="center">
  <img src="./screenshot.png" alt="Konachan Yew Screenshot" width="800" />
</p>

<p align="center">
  <a href="https://www.rust-lang.org/"><img src="https://img.shields.io/badge/Rust-1.94+-dea584?style=flat&logo=rust" alt="Rust" /></a>
  <a href="https://yew.rs/"><img src="https://img.shields.io/badge/Yew-0.23.0-88b86e?style=flat" alt="Yew" /></a>
  <a href="https://github.com/lf-wxp/konachan-yew"><img src="https://img.shields.io/github/license/lf-wxp/konachan-yew" alt="License" /></a>
  <a href="https://github.com/lf-wxp/konachan-yew/releases"><img src="https://img.shields.io/github/v/release/lf-wxp/konachan-yew?include_prereleases" alt="Release" /></a>
</p>

<p align="center">
  A modern, fast, and beautiful web frontend for <a href="https://konachan.net/">Konachan</a>, built with Rust and WebAssembly using the <a href="https://yew.rs/">Yew</a> framework.
</p>

## 📖 Overview

**konachan-yew** is a high-performance single-page application (SPA) that provides an enhanced browsing experience for Konachan image boards. Leveraging the power of Rust and WebAssembly, it delivers lightning-fast performance with a modern UI.

This repository contains the **web frontend only**. The backend API server that supplies image data can be found at [konachan-api](https://github.com/lf-wxp/konachan-api).

## ✨ Features

| Feature | Description |
|---------|-------------|
| 🖼️ **Waterfall Layout** | Pinterest-style masonry layout for optimal image browsing |
| 📥 **Download Management** | Built-in download queue with progress tracking |
| 🎨 **Dynamic Wallpaper** | Set images as dynamic wallpapers |
| 🌍 **i18n Support** | Multi-language support (English & Chinese) |
| 🎭 **Safe Mode** | Configurable content filtering for safe browsing |
| 🌓 **Theme Support** | Customizable UI with theme colors |
| 🔍 **Advanced Search** | Powerful tag-based search and filtering |
| ⚡ **Blazing Fast** | Powered by Rust/WASM for near-native performance |
| 📱 **Responsive** | Works seamlessly on desktop and mobile devices |
| 🐳 **Docker Ready** | One-command deployment with Docker |

## 🛠️ Technology Stack

- **Framework**: [Yew 0.23.0](https://yew.rs/) - Modern Rust framework for creating multi-threaded front-end web apps with WebAssembly
- **Language**: Rust (Edition 2024)
- **Styling**: [Stylist](https://github.com/futursolo/stylist-rs) - CSS-in-Rust with Yew integration
- **State Management**: [Bounce](https://github.com/yewstack/bounce) - State management for Yew
- **WASM Bindings**: 
  - [wasm-bindgen](https://github.com/rustwasm/wasm-bindgen) - High-level interactions between WASM and JavaScript
  - [gloo](https://github.com/rustwasm/gloo) - Collection of WASM-friendly utilities
- **Build Tool**: [Trunk](https://trunkrs.dev/) - WASM web application bundler
- **Desktop (Optional)**: [Tauri](https://tauri.app/) - Build smaller, faster, and more secure desktop applications

## 🚀 Quick Start

### Prerequisites

- [Rust](https://www.rust-lang.org/) toolchain (1.94+)
- `wasm32-unknown-unknown` target: 
  ```bash
  rustup target add wasm32-unknown-unknown
  ```
- [Trunk](https://trunkrs.dev/): 
  ```bash
  cargo install trunk@0.21.14
  ```
- (Optional) [cargo-make](https://github.com/sagiegurari/cargo-make): 
  ```bash
  cargo install cargo-make
  ```

### Feature Flags

| Flag | Description |
|------|-------------|
| `fake` | Use mock data for development without a backend |
| `safe` | Safe mode build, outputs to `dist_safe` directory |
| `web` | Production web build with API proxy |
| `tauri` | Enable Tauri integration for desktop builds |

### Development

#### Using Trunk directly

```bash
# Dev server with fake data (no backend needed), port 8888
trunk serve --features fake

# Dev server with web feature (requires backend at localhost:8000)
trunk serve --features web

# Dev server with safe mode config
trunk serve --config Trunk.safe.toml
```

#### Using cargo-make (Recommended)

```bash
# Dev with fake data
cargo make dev-fake

# Dev with web feature (requires backend)
cargo make dev-web

# Dev with safe mode
cargo make dev-safe
```

The dev server runs on **port 8888** by default. API requests to `/api/` are proxied to `http://localhost:8000`.

### Available Tasks

```bash
# Format code
cargo make format

# Run clippy lints
cargo make clippy

# Format + clippy check
cargo make check

# Run tests
cargo make test

# Clean build artifacts
cargo make clean
```

## 📦 Production Build

### Web Build

```bash
trunk build --release --features web
```

The output static files will be in the `dist/` directory.

### Docker Deployment

#### Build the Docker image

```bash
# Using cargo-make
cargo make docker-build

# Or directly
docker build -t konachan-yew:latest .
```

#### Run the container

```bash
# Using cargo-make (maps to port 8080)
cargo make docker-run

# Or directly
docker run --rm -p 8080:80 --name konachan-yew konachan-yew:latest
```

#### Docker Architecture

The Dockerfile uses a **multi-stage build**:

1. **Builder stage** (`rust:1.94-bookworm`): Compiles Rust/WASM with Trunk
2. **Runner stage** (`nginx:1.27-bookworm`): Serves static files with nginx

The nginx configuration includes:
- Gzip compression for WASM, JS, CSS, and SVG
- Aggressive caching for static assets (1 year, immutable)
- SPA fallback (all routes serve `index.html`)
- Reverse proxy: `/api/*` → `http://backend:8000/`

#### Backend Service Setup

The nginx config proxies `/api/` requests to `http://backend:8000/`. Use **Docker Compose** to connect frontend and backend:

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
    # Or: build: ./path-to-backend
    ports:
      - "8000:8000"
```

Or use a manual Docker network:

```bash
docker network create app-net
docker run -d --name backend --network app-net your-backend-image
docker run -d --name frontend --network app-net -p 8080:80 konachan-yew:latest
```

#### Stop the container

```bash
# Using cargo-make
cargo make docker-stop

# Or directly
docker stop konachan-yew
```

## 📁 Project Structure

```
konachan-yew/
├── src/
│   ├── components/     # Yew components (Nav, List, Search, etc.)
│   ├── hook/           # Custom Yew hooks
│   ├── model/          # Data models and structs
│   ├── store/          # State management (Bounce)
│   ├── utils/          # Utility functions and i18n
│   └── main.rs         # Application entry point
├── static/             # Static assets
│   ├── css/            # Stylesheets
│   ├── font/           # Font files
│   ├── image/          # Image assets
│   ├── mock/           # Mock data for development
│   └── script/         # JavaScript utilities
├── dist/               # Build output (production)
├── Cargo.toml          # Rust dependencies
├── Trunk.toml          # Trunk configuration
├── Trunk.safe.toml     # Safe mode configuration
├── Dockerfile          # Docker multi-stage build
└── Makefile.toml       # cargo-make tasks
```

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

### Code Style

This project follows:
- [Rust Style Guide](https://doc.rust-lang.org/1.0.0/style/README.html)
- [Airbnb CSS Style Guide](https://github.com/airbnb/css)
- All code is formatted with `rustfmt` and linted with `clippy`

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🔗 Related Projects

| Project | Description |
|---------|-------------|
| [konachan-api](https://github.com/lf-wxp/konachan-api) | Backend API server for Konachan image data |
| [konachan-tauri](https://github.com/lf-wxp/konachan-tauri) | Desktop application version using Tauri framework |

## 🙏 Acknowledgments

- [Konachan](https://konachan.net/) for providing the image board API
- [Yew](https://yew.rs/) team for the amazing Rust/WASM framework
- All contributors who have helped with this project

## 📧 Contact

- GitHub Issues: [Report a bug or request a feature](https://github.com/lf-wxp/konachan-yew/issues)
- Project Link: [https://github.com/lf-wxp/konachan-yew](https://github.com/lf-wxp/konachan-yew)

---

<p align="center">
  Made with ❤️ using Rust + WebAssembly
</p>
