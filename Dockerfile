# Build the SvelteKit frontend. adapter-node output is self-contained (the app
# has no runtime dependencies), so only the build directory is needed at runtime.
FROM node:22-slim AS node_builder
WORKDIR /app/frontend
COPY frontend/package.json frontend/package-lock.json ./
RUN npm ci
COPY frontend/ ./
RUN npm run build

# Build the Rust API.
FROM rust:1.96-bookworm AS rust_builder
WORKDIR /usr/src/murmelbahn
COPY . .
RUN cargo install --path web

# Final image: Node (for the SvelteKit server) plus the Rust binary, both run by
# supervisord. SvelteKit is the public face on :3000 and proxies /api and
# /metrics to the Rust API on localhost:8080. supervisord (unlike s6-overlay)
# does not require PID 1, so it works under Fly Machines' init.
FROM node:22-slim
RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates libssl3 supervisor \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
# Rust API binary and the set definitions it reads (SETS_DIRECTORY=data/sets).
COPY --from=rust_builder /usr/local/cargo/bin/murmelbahn-web /usr/local/bin/murmelbahn-web
COPY data ./data
# SvelteKit server. package.json provides "type": "module" for node to run build/.
COPY --from=node_builder /app/frontend/build ./frontend/build
COPY --from=node_builder /app/frontend/package.json ./frontend/package.json
COPY docker/supervisord.conf /etc/supervisord.conf

ENV BIND_ADDRESS=127.0.0.1:8080 \
    INTERNAL_API=http://127.0.0.1:8080 \
    SETS_DIRECTORY=data/sets \
    HOST=0.0.0.0 \
    PORT=3000 \
    NODE_ENV=production

EXPOSE 3000
ENTRYPOINT ["supervisord", "-c", "/etc/supervisord.conf"]
