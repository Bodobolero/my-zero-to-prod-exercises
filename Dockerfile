# use cargo-chef to cache build of dependencies
FROM lukemathwalker/cargo-chef:latest as chef

# Let's switch our working directory to `app` (equivalent to `cd app`)
# The `app` folder will be created for us by Docker in case it does not 
# exist already.
WORKDIR /app

# Install the required system dependencies for our linking configuration
RUN apt update && apt install lld clang -y

#########################################
### Stage 1: determine which dependencies to build
FROM chef as planner
COPY . .
# Compute a lock-like file for our project
RUN cargo chef prepare --recipe-path recipe.json

#########################################
### Stage 2: build our dependencies
FROM chef as builder
COPY --from=planner /app/recipe.json recipe.json
# Build our project dependencies, not our application!
RUN cargo chef cook --release --recipe-path recipe.json

#########################################
# Up to this point, if our dependency tree stays the same,
# all layers should be cached. 
#########################################

#########################################
### Stage 3: build our project - only this should be done in subsequent builds
COPY . .
ENV SQLX_OFFLINE true
# Build our project
RUN cargo build --release --bin my-zero-to-prod-exercises


#########################################
### Stage 4: package our production image from the binary we just built
FROM debian:bullseye-slim AS runtime
WORKDIR /app
# Install OpenSSL - it is dynamically linked by some of our dependencies
# Install ca-certificates - it is needed to verify TLS certificates
# when establishing HTTPS connections
RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl ca-certificates \
    # Clean up
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*
# Copy the compiled binary from the builder environment 
# to our runtime environment
COPY --from=builder /app/target/release/my-zero-to-prod-exercises my-zero-to-prod-exercises
# We need the configuration file at runtime!
COPY configuration configuration
# use configuration/production.yaml instead of configuration/local.yaml
ENV APP_ENVIRONMENT production
ENTRYPOINT ["./my-zero-to-prod-exercises"]