#!/usr/bin/env bash
set -euo pipefail

# 1) Installation of the dot binary
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


# 2) Installation of completion shims
RAW_BASE="https://raw.githubusercontent.com/$OWNER/$REPO/main/shims"  # swap to tag or new repo when you move

append_once() { # append $2 to file $1 if not already present
  local file="$1" line="$2"
  grep -Fqx "$line" "$file" 2>/dev/null || printf "\n%s\n" "$line" >> "$file"
}

# bash
if command -v bash >/dev/null 2>&1; then
  mkdir -p "$HOME/.config/polkadot"
  curl -fsSL "$RAW_BASE/completion.bash" -o "$HOME/.config/polkadot/completion.bash"

  # Ensure non-login bash picks it up
  append_once "$HOME/.bashrc" '[ -f "$HOME/.config/polkadot/completion.bash" ] && . "$HOME/.config/polkadot/completion.bash"'
  # Ensure login bash (Terminal on macOS) picks it up
  append_once "$HOME/.bash_profile" '[ -f "$HOME/.bashrc" ] && . "$HOME/.bashrc"'
  echo "bash completion installed"
fi

# zsh
if command -v zsh >/dev/null 2>&1; then
  local_zsh_comp="$HOME/.zsh/completions"
  mkdir -p "$local_zsh_comp"
  curl -fsSL "$RAW_BASE/completion.zsh" -o "$local_zsh_comp/_dot"
  append_once "$HOME/.zshrc" 'fpath+=("$HOME/.zsh/completions")'
  echo "zsh completion installed"
fi

# fish
if command -v fish >/dev/null 2>&1; then
  fish_dir="$HOME/.config/fish/completions"
  mkdir -p "$fish_dir"
  curl -fsSL "$RAW_BASE/completion.fish" -o "$fish_dir/dot.fish"
  echo "fish completion installed"
fi
