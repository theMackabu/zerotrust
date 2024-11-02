# prepare cargo cache
FROM rust:alpine3.20 AS chef
RUN apk add --no-cache musl-dev git
RUN cargo install cargo-chef 

WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# nodejs install stage
FROM node:lts-alpine3.20 AS base

ENV PNPM_HOME="/pnpm"
ENV PATH="$PNPM_HOME:$PATH"

RUN corepack enable
RUN corepack prepare pnpm@latest-9 --activate

COPY ./src/pages /pages
WORKDIR /pages

# pages build stage
FROM base AS build
RUN apk add --no-cache tar

RUN --mount=type=cache,id=pnpm,target=/pnpm/store pnpm install --frozen-lockfile
RUN pnpm run build

# cargo build stage
FROM chef AS builder

ARG TARGETARCH
RUN case "$TARGETARCH" in \
    amd64)  echo "x86_64-unknown-linux-musl" > /rust.target ;; \
    arm64)  echo "aarch64-unknown-linux-musl" > /rust.target ;; \
    *) echo "Unsupported architecture: $TARGETARCH" && exit 1 ;; \
    esac && \
    rustup target add $(cat /rust.target)

RUN apk add --no-cache \
  perl \
  libpq \
  build-base \
  libpq-dev \
  postgresql-dev \
  openssl-dev \
  openssl-libs-static

WORKDIR /app
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --target=$(cat /rust.target) --recipe-path recipe.json

COPY . .
COPY --from=build /pages/dist src/pages/dist

RUN cargo build --release --target=$(cat /rust.target)
RUN cp /app/target/$(cat /rust.target)/release/zerotrust .

# final container
FROM alpine:3.20 AS runner

WORKDIR /app
COPY --from=builder /app/zerotrust ./zerotrust

RUN chmod +x ./zerotrust
CMD ["./zerotrust"]
