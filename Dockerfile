# ---------- builder stage ----------------------------------------------------
FROM mcr.microsoft.com/devcontainers/rust:1 AS builder

ARG SDK_BRANCH=polkadot-stable2503-6 
ARG USER_HOME=/home/vscode

# Rust toolchain & build deps already installed by the rust Feature layer.
# Add a couple of system libs substrate still needs at link‑time.
RUN apt-get update && apt install --assume-yes git clang curl libssl-dev protobuf-compiler

USER vscode
RUN rustup update stable && \
    rustup default stable && \
    rustup target add wasm32-unknown-unknown && \
    rustup component add rust-src

WORKDIR $USER_HOME

# 1) Clone SDK at the branch we care about
RUN git clone -q --depth 1 -b $SDK_BRANCH https://github.com/paritytech/polkadot-sdk.git

# 2) Compile substrate‑node + eth‑rpc (release = optimized)
WORKDIR $USER_HOME/polkadot-sdk
RUN cargo build --release --bin substrate-node && \
    cargo build --release -p pallet-revive-eth-rpc --bin eth-rpc

# 3) Grab the *latest* resolc binary (GitHub API)
#    If GitHub throttling is a worry, pin RESOLC_VERSION and hard‑code the tag.
RUN RESOLC_URL=$(curl -s https://api.github.com/repos/paritytech/revive/releases/latest \
       | grep 'browser_download_url.*resolc-x86_64-unknown-linux-musl"' \
       | cut -d '"' -f 4) && \
    curl -L $RESOLC_URL -o $USER_HOME/resolc-x86_64-unknown-linux-musl && \
    chmod +x $USER_HOME/resolc-x86_64-unknown-linux-musl

# 4) (doing everything else here b/c we don't want to repeat earlier steps)
RUN rustup toolchain install nightly --component rust-src && \
    cargo +nightly build --release --package subkey          # produces target/release/subkey

# ---------- runtime stage ----------------------------------------------------
FROM mcr.microsoft.com/devcontainers/base:ubuntu-22.04

USER vscode
ARG USER_HOME=/home/vscode
WORKDIR $USER_HOME

# copy only the final artifacts (keep image small)
COPY --from=builder ${USER_HOME}/polkadot-sdk/target/release/substrate-node $USER_HOME/polkadot-sdk/target/release/substrate-node
COPY --from=builder ${USER_HOME}/polkadot-sdk/target/release/eth-rpc        $USER_HOME/polkadot-sdk/target/release/eth-rpc
COPY --from=builder ${USER_HOME}/resolc-x86_64-unknown-linux-musl           $USER_HOME/resolc-x86_64-unknown-linux-musl
COPY --from=builder ${USER_HOME}/polkadot-sdk/target/release/subkey         /usr/local/bin/subkey

# copy the scripts
COPY ./devcontainer-startup.sh $USER_HOME/devcontainer-startup.sh
COPY ./check-balance.sh $USER_HOME/check-balance.sh
