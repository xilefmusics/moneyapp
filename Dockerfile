FROM alpine/git:v2.52.0 AS dependency_downloader

WORKDIR /fancy_yew
RUN git clone --depth 1 --branch 0.5.0 https://github.com/xilefmusics/fancy_yew.git .


FROM rust:bookworm AS builder

RUN apt-get update && apt-get install -y --no-install-recommends pkg-config libssl-dev \
    && rm -rf /var/lib/apt/lists/* \
    && rustup target add wasm32-unknown-unknown \
    && cargo install cargo-binstall \
    && cargo binstall trunk --version 0.21.14 --no-confirm

COPY --from=dependency_downloader /fancy_yew /fancy_yew

WORKDIR /money_app
COPY ./shared ./shared
COPY ./backend ./backend
WORKDIR /money_app/backend
RUN cargo build --release

WORKDIR /money_app
COPY ./frontend ./frontend
WORKDIR /money_app/frontend
RUN trunk build --release


FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /money_app/frontend/dist/ /app/static
COPY --from=builder /money_app/backend/target/release/backend /app/backend
COPY ./backend/db-migrations /app/db-migrations

ENV HOST="0.0.0.0" \
    PORT="8080" \
    STATIC_DIR="/app/static" \
    ATTACHMENT_DIR="/app/attachments" \
    DB_MIGRATION_PATH="/app/db-migrations" \
    DB_ADDRESS="http://127.0.0.1:8000" \
    DB_NAMESPACE="moneyapp" \
    DB_DATABASE="moneyapp"

VOLUME "/app/attachments"

ENTRYPOINT ["/app/backend"]
