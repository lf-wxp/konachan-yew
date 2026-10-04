# =============================================================================
# Stage 1: Builder - Compile Rust/WASM with Trunk
# =============================================================================
FROM rust:1.94-bookworm AS builder

# Install wasm target and trunk
# `--locked` pins trunk's deps (avoids the cssparser/lightningcss type mismatch).
# LTO is disabled to keep peak memory low: trunk's fat-LTO link step can OOM
# inside memory-constrained build environments (Docker Desktop / CI runners).
RUN rustup target add wasm32-unknown-unknown \
    && CARGO_PROFILE_RELEASE_LTO=off \
       CARGO_PROFILE_RELEASE_CODEGEN_UNITS=16 \
       cargo install --locked trunk --version 0.21.14

# Install system dependencies for sass compilation
RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Cache dependencies by copying manifests first
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo 'fn main() {}' > src/main.rs \
    && cargo fetch --target wasm32-unknown-unknown \
    && rm -rf src \
    && rm -f target/release/konachan target/release/deps/konachan*

# Copy source code and static assets
COPY src/ src/
COPY static/ static/
COPY index.html .
COPY Trunk.toml .
COPY rustfmt.toml .

# Build with trunk for production (web feature)
RUN trunk build --release --features web

# =============================================================================
# Stage 2: Runner - Serve static files with nginx
# Both stages use Debian Bookworm for consistency
# =============================================================================
FROM nginx:1.27-bookworm AS runner

# Remove default nginx config
RUN rm /etc/nginx/conf.d/default.conf

# Copy custom nginx config
COPY <<'EOF' /etc/nginx/conf.d/app.conf
server {
    listen 80;
    server_name _;

    root /usr/share/nginx/html;
    index index.html;

    # Enable gzip compression for WASM and JS
    gzip on;
    gzip_types application/wasm application/javascript text/css text/html image/svg+xml;
    gzip_min_length 256;

    # Runtime configuration is rewritten on container start, so it must never
    # be cached (exact match takes precedence over the asset regex below).
    location = /config.js {
        add_header Cache-Control "no-store, must-revalidate";
        expires -1;
    }

    # Cache static assets aggressively
    location ~* \.(wasm|js|css|svg|png|jpg|jpeg|gif|ico|woff|woff2|ttf)$ {
        expires 1y;
        add_header Cache-Control "public, immutable";
    }

    # SPA fallback - serve index.html for all routes
    location / {
        try_files $uri $uri/ /index.html;
    }
}
EOF

# Copy built static files from builder
COPY --from=builder /app/dist /usr/share/nginx/html

# Regenerate the runtime config from environment variables on container start
# (e.g. `docker run -e KONACHAN_SAFE=true ...`), so one image can serve both
# the regular and the safe mode.
COPY docker/30-konachan-config.sh /docker-entrypoint.d/30-konachan-config.sh
RUN chmod +x /docker-entrypoint.d/30-konachan-config.sh

EXPOSE 80

CMD ["nginx", "-g", "daemon off;"]
