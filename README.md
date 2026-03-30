# konachan-yew

A better experience web frontend for [konachan](https://konachan.net/), built with [Yew](https://yew.rs/) (Rust/WASM).
This is the web frontend only — [konachan-api](https://github.com/lf-wxp/konachan-api) is the backend server supplying image data.

## Screenshot

![screenshot](./screenshot.png)

## Features

| Feature | Description |
|---------|-------------|
| `fake`  | Use fake/mock data for development without a backend |
| `safe`  | Safe mode, builds to a separate `dist_safe` directory |
| `web`   | Production web build, proxies API requests to backend |
| `tauri` | Enable [tauri-sys](https://github.com/JonasKruckenberg/tauri-sys) integration for desktop builds |

## Prerequisites

- [Rust](https://www.rust-lang.org/) toolchain
- `wasm32-unknown-unknown` target: `rustup target add wasm32-unknown-unknown`
- [Trunk](https://trunkrs.dev/): `cargo install trunk`
- (Optional) [cargo-make](https://github.com/sagiegurari/cargo-make): `cargo install cargo-make`

## Development

### Using Trunk directly

```bash
# Dev server with fake data (no backend needed), port 8888
trunk serve --features fake

# Dev server with web feature (requires backend at localhost:8000)
trunk serve --features web

# Dev server with safe mode config
trunk serve --config Trunk.safe.toml
```

### Using cargo-make

```bash
# Dev with fake data
cargo make dev-fake

# Dev with web feature (requires backend)
cargo make dev-web

# Dev with safe mode
cargo make dev-safe

# Format code
cargo make format

# Run clippy
cargo make clippy

# Format + clippy check
cargo make check

# Run tests
cargo make test

# Clean build artifacts
cargo make clean
```

The dev server runs on **port 8888** by default. API requests to `/api/` are proxied to `http://localhost:8000/`.

## Production Build

```bash
trunk build --release --features web
```

The output static files will be in the `dist/` directory.

## Docker Deployment

### Build the Docker image

```bash
# Using cargo-make
cargo make docker-build

# Or directly
docker build -t konachan-yew:latest .
```

### Run the container

```bash
# Using cargo-make (maps to port 8080)
cargo make docker-run

# Or directly
docker run --rm -p 8080:80 --name konachan-yew konachan-yew:latest
```

### Docker architecture

The Dockerfile uses a **multi-stage build**:

1. **Builder stage** (`rust:1.94-bookworm`): Compiles Rust/WASM with Trunk
2. **Runner stage** (`nginx:1.27-bookworm`): Serves static files with nginx

The nginx configuration includes:
- Gzip compression for WASM, JS, CSS, and SVG
- Aggressive caching for static assets (1 year, immutable)
- SPA fallback (all routes serve `index.html`)
- Reverse proxy: `/api/*` → `http://backend:8000/`

### Backend service setup

The nginx config proxies `/api/` requests to `http://backend:8000/`. The hostname `backend` is resolved via Docker networking. Use **Docker Compose** to connect frontend and backend:

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

### Stop the container

```bash
# Using cargo-make
cargo make docker-stop

# Or directly
docker stop konachan-yew
```

## Reference

- [konachan-api](https://github.com/lf-wxp/konachan-api) — the konachan image data server
- [konachan-tauri](https://github.com/lf-wxp/konachan-tauri) — the desktop version supported by tauri framework
