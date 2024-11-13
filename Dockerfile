# Build stage
FROM rust:slim-bullseye as builder

# Let's switch our working directory to `app` (equivalent to `cd app`)
# The `app` folder will be created for us by Docker in cas it does not
# exist already
WORKDIR /app
# Install the required system dependencies for our linking configuration
RUN apt update && apt install lld clang -y
# Copy all files from our working environment to our Docker image
COPY . .
ENV SQLX_OFFLINE true
# Lets build our binary!
# We'll use the release profile to make it faaast
RUN cargo build --release

# Runtime stage
FROM debian:bullseye-slim as runtime
WORKDIR /app

# Install OpenSSL - it is dynamically linked by some of our dependencies
# Install ca-certificates - it is needed to verify TLS certificates
# when establishing HTTPS connections
RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl ca-certificates \
    # Clean up
    && apt-get autoremove \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

# Copy the compiled binary from the builder environment
# to our runtime environtment
COPY --from=builder /app/target/release/zero2prod zero2prod

# We need the configuration file at runtime!
COPY configuration configuration

ENV APP_ENVIRONMENT production
# When  `docker run` is executed, launch the binary!
ENTRYPOINT ["./zero2prod"]