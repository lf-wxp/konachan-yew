# =============================================================================
# Stage 1: Builder - Compile Rust/WASM with Trunk
# =============================================================================
FROM rust:1.94-bookworm AS builder

# Install wasm target and trunk
RUN rustup target add wasm32-unknown-unknown \
    && cargo install trunk --version 0.21.14

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

    # Cache static assets aggressively
    location ~* \.(wasm|js|css|svg|png|jpg|jpeg|gif|ico|woff|woff2|ttf)$ {
        expires 1y;
        add_header Cache-Control "public, immutable";
    }

    # SPA fallback - serve index.html for all routes
    location / {
        try_files $uri $uri/ /index.html;
    }

    # Proxy API requests to backend
    location /api/ {
        proxy_pass http://backend:8000/;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }
}
EOF

# Copy built static files from builder
COPY --from=builder /app/dist /usr/share/nginx/html

EXPOSE 80

CMD ["nginx", "-g", "daemon off;"]
