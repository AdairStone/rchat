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
    && rm -rf /var/lib/apt/lists/* \
    && cargo build --release

FROM debian:bookworm
WORKDIR /app
COPY backend/config ./config
COPY backend/static ./static
COPY backend/public ./public
COPY backend/.env ./.env
COPY --from=builder /usr/src/app/target/release/backend /app/backend
RUN sed -i 's/deb.debian.org/mirrors.aliyun.com/g' /etc/apt/sources.list.d/debian.sources && \
    apt-get update \
    && apt-get install -y \
    openssl \
    libssl-dev \
    pkg-config \
    && rm -rf /var/lib/apt/lists/* \
CMD ["/app/backend"]