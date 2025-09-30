FROM --platform=linux/amd64 ubuntu:latest
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    build-essential \
    gdb \
    nasm \
    make \
    git \
    && rm -rf /var/lib/apt/lists/*
WORKDIR /src

# docker build -t x86dev . 