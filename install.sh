#!/usr/bin/env bash
set -euo pipefail

OWNER=charlesHetterich REPO=tmp-rnd BIN=dot
OS=$(uname -s); case "$OS" in Linux) OS=linux;; Darwin) OS=macos;; *) echo "unsupported OS"; exit 1;; esac
ARCH=$(uname -m); case "$ARCH" in x86_64|amd64) ARCH=amd64;; arm64|aarch64) ARCH=arm64;; *) echo "unsupported arch"; exit 1;; esac

TAG=${DOTCLI_TAG:-$(curl -fsSL "https://api.github.com/repos/$OWNER/$REPO/releases/latest" \
      | sed -n 's/.*"tag_name":[[:space:]]*"\(.*\)".*/\1/p' | head -n1)}

ASSET="$BIN-$OS-$ARCH"
URL="https://github.com/$OWNER/$REPO/releases/download/$TAG/$ASSET"

mkdir -p "$HOME/.polkadot/bin" "$HOME/.local/bin"
curl -fL "$URL" -o "$HOME/.polkadot/bin/$BIN"
chmod +x "$HOME/.polkadot/bin/$BIN"
ln -sf "$HOME/.polkadot/bin/$BIN" "$HOME/.local/bin/$BIN"

case ":$PATH:" in *":$HOME/.local/bin:"*) ;; *) echo "Add \$HOME/.local/bin to PATH";; esac
echo "Installed $BIN ($OS/$ARCH) from $TAG -> $HOME/.polkadot/bin/$BIN"
