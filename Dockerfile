#
# STEP 1: Install wasm tools
#
FROM rust:1-slim-bullseye as rust-builder

# Set environment variables to avoid prompts
ENV DEBIAN_FRONTEND=noninteractive

# Update and install necessary build dependencies
RUN apt-get update && apt-get install -y curl ca-certificates curl wget build-essential pkg-config sudo jq
RUN curl -s https://packagecloud.io/install/repositories/wasmcloud/core/script.deb.sh | bash
RUN apt install wash

# Install wasm tools
RUN cargo install --git https://github.com/bytecodealliance/wit-bindgen wit-bindgen-cli && \
    cargo install wasm-tools --version 1.0.40 && \
    cargo install wit-deps-cli just

# Install cosmo from latest
RUN bash -c "$(curl -fsSL https://cosmonic.sh/install.sh)"

#
# STEP 2: Build user devcontainer
#
FROM rust:1-slim-bullseye as workshop-container
COPY --from=rust-builder /usr/local/cargo/bin/ /usr/local/cargo/bin/
COPY --from=rust-builder /usr/local/bin/ /usr/local/bin/
COPY --from=rust-builder /root/.cosmo/bin /root/.cosmo/bin

# Set environment variables to avoid prompts
ENV DEBIAN_FRONTEND=noninteractive

# Update and install necessary dependencies
RUN apt-get update && apt-get install -y curl ca-certificates curl wget build-essential pkg-config sudo jq

RUN rustup target add wasm32-unknown-unknown && \
    rustup target add wasm32-wasi

ENV PATH="/root/.cosmo/bin:/usr/local/go/bin:${PATH}"

# Install Tinygo 0.29.0
RUN arch=$(uname -m) && \
    case "$arch" in \
        x86_64) arch="amd64" ;; \
        aarch64) arch="arm64" ;; \
        *) arch="$raw_arch" ;; \
    esac && \
    # Install Golang 1.20.7
    wget https://golang.org/dl/go1.20.7.linux-$arch.tar.gz && \
    tar -C /usr/local -xzf go1.20.7.linux-$arch.tar.gz && \
    rm go1.20.7.linux-$arch.tar.gz && \
    wget https://github.com/tinygo-org/tinygo/releases/download/v0.29.0/tinygo_0.29.0_$arch.deb && \
    mkdir /wasmcon-workshop && \
    # Install TinyGo 0.29.0
    mv tinygo_0.29.0_$arch.deb /wasmcon-workshop/tinygo_0.29.0_$arch.deb && \
    cd /wasmcon-workshop && \
    sudo dpkg -i tinygo_0.29.0_$arch.deb && \
    rm tinygo_0.29.0_$arch.deb && \
    # Install latest yq
    wget https://github.com/mikefarah/yq/releases/latest/download/yq_linux_$arch -O /usr/bin/yq && \
    chmod +x /usr/bin/yq && \
    # Install NATS CLI
    go install github.com/nats-io/natscli/nats@latest

RUN apt-get clean && rm -rf /var/lib/apt/lists/*

CMD ["/bin/bash"]