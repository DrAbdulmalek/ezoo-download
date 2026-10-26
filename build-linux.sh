#!/usr/bin/env bash
# Ezoo Download — Linux (Manjaro/Arch) build & install helper
#
# Usage:
#   ./build-linux.sh                  # build the release binary only
#   ./build-linux.sh --install-deps   # install build/runtime deps via pacman
#   ./build-linux.sh --install        # build + install into ~/.local
#
# The Linux port uses the SYSTEM helpers (yt-dlp, ffmpeg, aria2c) found on
# PATH instead of downloading Windows builds — see README-LINUX.md.
set -euo pipefail
cd "$(dirname "$0")"

BUILD_PKGS="base-devel rust openssl gtk3 libxkbcommon fontconfig wayland libglvnd"
RUNTIME_PKGS="yt-dlp ffmpeg aria2"

info() { printf '\033[1;34m[ezoo]\033[0m %s\n' "$*"; }
die()  { printf '\033[1;31m[ezoo]\033[0m %s\n' "$*" >&2; exit 1; }
have() { command -v "$1" >/dev/null 2>&1; }

install_deps() {
  if ! have pacman; then
    info "no pacman found — not Manjaro/Arch; install these manually and rerun:"
    info "  build:   rust openssl gtk3 libxkbcommon fontconfig wayland libglvnd base-devel"
    info "  runtime: yt-dlp ffmpeg aria2"
    return 0
  fi
  info "installing build dependencies: $BUILD_PKGS"
  sudo pacman -S --needed $BUILD_PKGS
  info "installing runtime helpers (the app finds them automatically): $RUNTIME_PKGS"
  sudo pacman -S --needed $RUNTIME_PKGS
}

build() {
  if ! have cargo; then
    die "cargo not found. Install Rust first:  sudo pacman -S rust   (or: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh)"
  fi
  info "building EzooDownload (release) — the first build takes several minutes…"
  cargo build --release --workspace
  info "build finished:"
  ls -lh target/release/EzooDownload
}

install_local() {
  local dest="$HOME/.local"
  [ -x target/release/EzooDownload ] || die "binary not built yet — run a build first"
  install -Dm755 target/release/EzooDownload "$dest/bin/ezoo-download"
  install -Dm644 crates/app/ui/icon.png "$dest/share/pixmaps/ezoo-download.png"
  mkdir -p "$dest/share/applications"
  sed "s|^Exec=.*|Exec=$dest/bin/ezoo-download|" packaging/ezoo-download.desktop \
    > "$dest/share/applications/ezoo-download.desktop"
  info "installed: $dest/bin/ezoo-download"
  info "launcher entry: 'Ezoo Download' (menu) — make sure $dest/bin is in PATH"
}

case "${1:-build}" in
  "") build ;;
  build) build ;;
  --install-deps) install_deps ;;
  --install) install_deps; build; install_local ;;
  *) die "unknown option: $1  (use: build | --install-deps | --install)" ;;
esac
