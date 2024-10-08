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

# Base image for building frontend projects
FROM node:20.14 AS node-base

# Set up npm mirror and install pnpm globally
RUN npm config set registry http://192.168.0.105:4873/ && \
    npm install -g pnpm

# Build the frontend projects
FROM node-base AS build
WORKDIR /app
COPY frontend/ /app/frontend
COPY chat-front/ /app/chat-front
RUN npm config set registry http://192.168.0.105:4873/ && \
    cd /app/frontend && pnpm install && pnpm build && \
    cd /app/chat-front && pnpm install && pnpm build

# Final runtime image with a minimal base
FROM debian:bookworm-slim AS base-runtime

RUN sed -i 's/deb.debian.org/mirrors.aliyun.com/g' /etc/apt/sources.list.d/debian.sources && \
    apt-get update && apt-get install -y openssl nginx vim procps && \
    rm -rf /var/lib/apt/lists/*

# Runtime stage
FROM base-runtime AS runner
WORKDIR /app
COPY --from=base-deps /usr/src/app/target/release/backend /app/backend/backend
COPY --from=build /app/frontend/dist /app/frontend
COPY --from=build /app/chat-front/dist /app/chat-front
COPY backend/config ./backend/config
COPY backend/static ./backend/static
COPY backend/public ./backend/public
COPY backend/.env ./backend/.env
COPY nginx.conf /etc/nginx/sites-available/default
RUN sed -i 's/user www-data;/user root;/g' /etc/nginx/nginx.conf
WORKDIR /app/backend
EXPOSE 8888 8889
CMD /usr/sbin/nginx & ./backend