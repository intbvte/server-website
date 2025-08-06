FROM docker.io/rust:1-slim-bookworm AS backend-build

ARG pkg=railways-server-website

RUN apt-get update && apt-get install -y pkg-config libssl-dev

WORKDIR /build

COPY backend/ .

ENV SQLX_OFFLINE=true

RUN set -eux; \
    cargo build --release; \
    objcopy --compress-debug-sections target/release/$pkg ./main

################################################################################

FROM node:20-slim AS frontend-build

ENV PNPM_HOME="/pnpm"
ENV PATH="$PNPM_HOME:$PATH"
RUN corepack enable

WORKDIR /frontend

COPY frontend/ .

RUN pnpm install
RUN pnpm build

################################################################################

FROM docker.io/debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app

## Copy the main binary
COPY --from=backend-build /build/main ./

## Copy frontend
COPY --from=frontend-build /frontend/build ./static

## Ensure the container listens globally on port 8080
ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8080

CMD ["./main"]