FROM rust:1.95-slim AS builder

LABEL org.opencontainers.image.source=https://github.com/keyvizsla/floo

RUN apt-get update && apt-get install -y --no-install-recommends \
    libssl-dev \
    pkg-config \
    ca-certificates \
    libsqlite3-dev \
    python3 \
    python3-pip \
    python3-venv \
    && rm -rf /var/lib/apt/lists/*

ENV VIRTUAL_ENV=/opt/venv
RUN python3 -m venv $VIRTUAL_ENV
ENV PATH="$VIRTUAL_ENV/bin:$PATH"
RUN pip install --no-cache-dir pexpect

WORKDIR /app

CMD ["cargo", "test", "--features", "test-docker-linux-bash"]
