FROM rust:1.75.0 as builder

ENV DEBIAN_FRONTEND=noninteractive

# Let's switch our working directory to `app` (equivalent to `cd app`)
# The `app` folder will be created for us by Docker in case it does not 
# exist already.
WORKDIR /app
# Install the required system dependencies for our linking configuration
RUN apt-get -qq update \
    && apt-get -qq install lld clang
# Copy all files from our working environment to our Docker image 
COPY . .
# Let's build our binary!
# We'll use the release profile to make it faaaast
ENV SQLX_OFFLINE=true
RUN cargo build --release

FROM rust:1.75.0 as runtime
WORKDIR /app

COPY --from=builder /app/target/release/zero2prod zero2prod
COPY config config

ENV APP_ENVIRONMENT=production
# When `docker run` is executed, launch the binary!
ENTRYPOINT [ "./zero2prod" ]
