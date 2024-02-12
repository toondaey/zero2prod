FROM lukemathwalker/cargo-chef:0.1.62-rust-slim-bullseye as c-chef

ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get -qq update \
    && apt-get -qq install clang lld libssl-dev pkg-config \
    && apt-get -qq autoremove \
    && apt-get -qq clean \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

FROM c-chef as planner
COPY . .
RUN cargo chef prepare --recipe-path=recipe.json

FROM c-chef as builder
ENV DEBIAN_FRONTEND=noninteractive
COPY --from=planner /app/recipe.json recipe.json

RUN cargo chef cook --release --recipe-path=recipe.json
COPY . .
ENV SQLX_OFFLINE=true
RUN cargo build --release --bin zero2prod

FROM debian:stable-slim as runtime
ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get -qq update \
    && apt-get -qq install --no-install-recommends openssl ca-certificates \
    && apt-get -qq autoremove \
    && apt-get -qq clean \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/zero2prod zero2prod
COPY config config

# ENV APP_ENVIRONMENT=production
# ENV APP__DATABASE__HOST=localhost
# ENV APP__APP__HOST=127.0.0.1
ENTRYPOINT [ "/app/zero2prod" ]
