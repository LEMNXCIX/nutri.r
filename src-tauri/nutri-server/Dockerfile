# Build stage
FROM rust:1.92-slim-bookworm AS builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/app

# Copy the entire src-tauri directory (context should be ./src-tauri)
COPY . .

# Build the server in release mode
WORKDIR /usr/src/app/nutri-server
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    libssl3 \
    ca-certificates \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Create data directory
RUN mkdir -p /data
ENV NUTRI_DATA_DIR=/data

# Copy the binary from the builder stage
# Since we build from within nutri-server and it's not a workspace root in this context,
# the target directory is created inside the nutri-server folder.
COPY --from=builder /usr/src/app/nutri-server/target/release/nutri-server /usr/local/bin/nutri-server

EXPOSE 3001

# Command to run the server
CMD ["nutri-server"]
