# Base image for setting up Rust environment
FROM rustlang/rust:nightly-slim AS rust-base-builder

# Update apt sources and install dependencies
RUN sed -i 's/deb.debian.org/mirrors.aliyun.com/g' /etc/apt/sources.list.d/debian.sources && \
    apt-get update && apt-get install -y \
    git openssl libssl-dev pkg-config && \
    git config --global http.sslVerify false && \
    rm -rf /var/lib/apt/lists/*

# Cache Rust dependencies
FROM rust-base-builder AS base-deps

WORKDIR /usr/src/app
COPY backend/ ./
RUN cargo build --release

# Build the Rust project
FROM base-deps AS project-builder
WORKDIR /usr/src/app
COPY backend/ ./
RUN cargo build --release

# Base image for building frontend projects
FROM node:20.14 AS node-base

# Set up npm mirror and install pnpm globally
RUN npm config set registry http://registry.npmmirror.com && npm install -g pnpm

# Cache npm dependencies
FROM node-base AS dependencies

WORKDIR /app
COPY frontend/package.json frontend/pnpm-lock.yaml ./frontend/
COPY chat-front/package.json chat-front/pnpm-lock.yaml ./chat-front/
RUN cd frontend && pnpm install && cd ../chat-front && pnpm install

# Build the frontend projects
FROM dependencies AS build
COPY frontend/ ./frontend
COPY chat-front/ ./chat-front
RUN cd frontend && pnpm build && cd ../chat-front && pnpm build

# Final runtime image with a minimal base
FROM debian:bookworm-slim AS base-runtime

RUN sed -i 's/deb.debian.org/mirrors.aliyun.com/g' /etc/apt/sources.list.d/debian.sources && \
    apt-get update && apt-get install -y openssl nginx && \
    rm -rf /var/lib/apt/lists/*

# Runtime stage
FROM base-runtime AS runner
WORKDIR /app
COPY --from=project-builder /usr/src/app/target/release/backend /app/backend
COPY --from=build /app/frontend/dist /app/frontend
COPY --from=build /app/chat-front/dist /app/chat-front
COPY backend/config ./config
COPY backend/static ./static
COPY backend/public ./public
COPY backend/.env ./.env
COPY nginx.conf /etc/nginx/sites-available/default
RUN ln -s /etc/nginx/sites-available/default /etc/nginx/sites-enabled/default && \
    sed -i 's/user www-data;/user root;/g' /etc/nginx/nginx.conf
EXPOSE 8888 8889
CMD ["/app/backend", "&", "nginx", "-g", "daemon off;"]