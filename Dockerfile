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
# the s6-overlay supervisor. SvelteKit is the public face on :3000 and proxies
# /api and /metrics to the Rust API on localhost:8080.
FROM node:22-slim
ARG S6_OVERLAY_VERSION=3.2.0.2
RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates libssl3 xz-utils \
    && rm -rf /var/lib/apt/lists/*
ADD https://github.com/just-containers/s6-overlay/releases/download/v${S6_OVERLAY_VERSION}/s6-overlay-noarch.tar.xz /tmp/
RUN tar -C / -Jxpf /tmp/s6-overlay-noarch.tar.xz && rm /tmp/s6-overlay-noarch.tar.xz
ADD https://github.com/just-containers/s6-overlay/releases/download/v${S6_OVERLAY_VERSION}/s6-overlay-x86_64.tar.xz /tmp/
RUN tar -C / -Jxpf /tmp/s6-overlay-x86_64.tar.xz && rm /tmp/s6-overlay-x86_64.tar.xz

WORKDIR /app
# Rust API binary and the set definitions it reads (SETS_DIRECTORY=data/sets).
COPY --from=rust_builder /usr/local/cargo/bin/murmelbahn-web /usr/local/bin/murmelbahn-web
COPY data ./data
# SvelteKit server. package.json provides "type": "module" for node to run build/.
COPY --from=node_builder /app/frontend/build ./frontend/build
COPY --from=node_builder /app/frontend/package.json ./frontend/package.json
# s6 service definitions (run the Rust API and the SvelteKit server side by side).
COPY docker/s6-rc.d /etc/s6-overlay/s6-rc.d
RUN chmod +x /etc/s6-overlay/s6-rc.d/api/run /etc/s6-overlay/s6-rc.d/web/run

ENV BIND_ADDRESS=127.0.0.1:8080 \
    INTERNAL_API=http://127.0.0.1:8080 \
    SETS_DIRECTORY=data/sets \
    HOST=0.0.0.0 \
    PORT=3000 \
    NODE_ENV=production

EXPOSE 3000
ENTRYPOINT ["/init"]
