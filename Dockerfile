FROM --platform=linux/amd64 ubuntu:latest
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    build-essential \
    gdb \
    nasm \
    make \
    git \
    python3 \
    curl \
    nano \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y --no-modify-path && \
    ~/.cargo/bin/rustup --version

ENV PATH="/root/.cargo/bin:${PATH}"

WORKDIR /workspace

# docker build -t x86dev . 