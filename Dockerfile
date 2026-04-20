FROM alpine/git:v2.52.0 AS dependency_downloader

WORKDIR /fancy_surreal
RUN git clone --depth 1 --branch 0.1.4 https://github.com/xilefmusics/fancy_surreal.git .

WORKDIR /fancy_yew
RUN git clone --depth 1 --branch 0.5.0 https://github.com/xilefmusics/fancy_yew.git .


FROM rust:1.95-bookworm AS builder

RUN export CARGO_BUILD_JOBS=$(nproc) && \
    cargo install cargo-binstall && \
    cargo binstall trunk --version 0.21.14 --no-confirm && \
    rustup target add wasm32-unknown-unknown

COPY --from=dependency_downloader /fancy_surreal /fancy_surreal
COPY --from=dependency_downloader /fancy_yew /fancy_yew

WORKDIR /money_app
COPY ./shared ./shared

WORKDIR /money_app
COPY ./backend ./backend
WORKDIR /money_app/backend
RUN cargo build --release

WORKDIR /money_app
COPY ./frontend ./frontend
WORKDIR /money_app/frontend
RUN trunk build --release

FROM ubuntu:24.04

COPY --from=builder /money_app/frontend/dist/ /app/static
COPY --from=builder /money_app/backend/target/release/backend /app/worship_viewer

ENV PORT="8000" \
    DB_HOST="db" \
    DB_PORT="8000" \
    DB_USER="root" \
    DB_PASSWORD="root" \
    DB_NAMESPACE="test" \
    DB_DATABASE="test" \
    STATIC_DIR="/app/static" \
    ATTACHMENT_DIR="/app/attachments"

VOLUME "/app/attachments"

ENTRYPOINT ["/app/worship_viewer"]
