FROM rust:1 as builder
WORKDIR /usr/src/myapp
COPY . .
RUN cargo install --path .

FROM debian:bullseye-slim
RUN apt-get update && apt-get install -y && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/discord-bot-nickname-activity-update-example /usr/local/bin/myapp
CMD ["myapp"]