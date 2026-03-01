# Build stage
FROM rust:1.92-slim-bookworm AS builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    curl \
    build-essential \
    && rm -rf /var/lib/apt/lists/*

# Install Node.js for TailwindCSS
RUN curl -fsSL https://deb.nodesource.com/setup_20.x | bash - && \
    apt-get install -y nodejs

# Install Trunk and Wasm target
RUN curl -L https://github.com/trunk-rs/trunk/releases/latest/download/trunk-x86_64-unknown-linux-gnu.tar.gz | tar -xzf- -C /usr/local/bin
RUN rustup target add wasm32-unknown-unknown

WORKDIR /usr/src/app

# Copy dependency manifests
COPY Cargo.toml Cargo.lock ./
COPY src-tauri/nutri-core/Cargo.toml ./src-tauri/nutri-core/
COPY src-tauri/nutri-app/Cargo.toml ./src-tauri/nutri-app/
COPY src-tauri/nutri-server/Cargo.toml ./src-tauri/nutri-server/

# Copy source code and build assets
COPY . .

# Build CSS and Application
RUN npm install
RUN trunk build --release

# Runtime stage
FROM nginx:alpine

# Copy custom nginx config
COPY nginx.conf /etc/nginx/conf.d/default.conf

# Copy build artifacts from builder stage
COPY --from=builder /usr/src/app/dist /usr/share/nginx/html

EXPOSE 80

CMD ["nginx", "-g", "daemon off;"]
