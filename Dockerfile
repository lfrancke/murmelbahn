FROM rust:1.66 as builder
WORKDIR /usr/src/murmelbahn
COPY . .
RUN cargo install --path web

FROM debian:bullseye-slim
EXPOSE 3000
RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/murmelbahn-web /usr/local/bin/murmelbahn-web
COPY data /data
CMD ["murmelbahn-web"]
