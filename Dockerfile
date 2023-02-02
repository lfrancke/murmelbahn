# Node part: Frontend
FROM node:16-alpine as node_builder
WORKDIR /app/frontend
COPY ["frontend/package.json", "frontend/package-lock.json*", "./"]
RUN npm install
COPY ./frontend .
ENV NODE_ENV=production
RUN npm run build


# Rust part: Backend
FROM rust:1.66 as rust_builder
WORKDIR /usr/src/murmelbahn
COPY . .
COPY --from=node_builder /app/frontend/dist /usr/src/murmelbahn/frontend/dist
RUN cargo install --path web

# Final image
FROM debian:bullseye-slim
EXPOSE 3000
RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=rust_builder /usr/local/cargo/bin/murmelbahn-web /usr/local/bin/murmelbahn-web
COPY data /data
CMD ["murmelbahn-web"]
