FROM rustlang/rust:nightly-slim AS builder
WORKDIR /usr/src/app
COPY backend/Cargo.toml backend/Cargo.lock ./
COPY backend/.cargo ./.cargo
COPY backend/aop_macro ./aop_macro
COPY backend/src ./src
COPY backend/config ./config
COPY backend/local ./local
COPY backend/public ./public
COPY backend/templates ./templates
RUN sed -i 's/deb.debian.org/mirrors.aliyun.com/g' /etc/apt/sources.list.d/debian.sources && \
    apt-get update \
    && apt-get install -y git \
    openssl \
    libssl-dev \
    pkg-config \
    && git config --global http.sslVerify false \
    && rm -rf /var/lib/apt/lists/*
RUN cargo build --release

FROM node:20.14 AS front-build
RUN npm config set registry http://registry.npmmirror.com \
    && npm install -g pnpm
WORKDIR /app
COPY frontend/ ./frontend
RUN cd frontend && \
    pnpm install
RUN cd frontend && \
    pnpm build

COPY chat-front/ ./chat-front
RUN cd chat-front && \
    pnpm install
RUN cd chat-front && \
    pnpm build

FROM debian:bookworm
WORKDIR /app
COPY backend/config ./config
COPY backend/static ./static
COPY backend/public ./public
COPY backend/.env ./.env
COPY --from=builder /usr/src/app/target/release/backend /app/backend
COPY --from=front-build /app/frontend/dist /app/frontend
COPY --from=front-build /app/chat-front/dist /app/chat-front
COPY nginx.conf /etc/nginx/sites-available/default
RUN sed -i 's/deb.debian.org/mirrors.aliyun.com/g' /etc/apt/sources.list.d/debian.sources && \
    apt-get update \
    && apt-get install -y \
    openssl \
    nginx \
    libssl-dev \
    pkg-config \
    && rm -rf /var/lib/apt/lists/* \
CMD ["/app/backend"] & nginx -g 'daemon off;'