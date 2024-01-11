FROM rust:1.75.0-slim as builder

ENV DEBIAN_FRONTEND=noninteractive

# Let's switch our working directory to `app` (equivalent to `cd app`)
# The `app` folder will be created for us by Docker in case it does not 
# exist already.
WORKDIR /app
# Install the required system dependencies for our linking configuration
RUN apt-get -qq update \
    && apt-get -qq install lld clang openssl ca-certificates libssl-dev pkg-config
# Copy all files from our working environment to our Docker image 
COPY . .
# Let's build our binary!
# We'll use the release profile to make it faaaast
ENV SQLX_OFFLINE=true
RUN cargo build --release

# FROM alpine:3.19 as runtime
# RUN apk add --no-cache openssl ca-certificates pkgconfig libc6-compat

# WORKDIR /app

# COPY --from=builder /app/target/release/zero2prod zero2prod
# COPY config config

# ENV APP_ENVIRONMENT=production
# # When `docker run` is executed, launch the binary!
# ENTRYPOINT [ "/app/zero2prod" ]
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

ENV APP_ENVIRONMENT=production
# When `docker run` is executed, launch the binary!
ENTRYPOINT [ "/app/zero2prod" ]

