#!/usr/bin/env bash
# Ezoo Download — prebuilt package installer (no compilation, no root)
#
# Usage:
#   ./install.sh              install into ~/.local (bin + icon + menu entry)
#   ./install.sh --uninstall  remove everything this script installed
#
# التنزيل والتثبيت بدون ترجمة وبدون صلاحيات جذر — أنشئه DrAbdulmalek
# as part of the Manjaro/Linux port of Ezoo Download.
set -euo pipefail

PREFIX="${HOME}/.local"
BIN_DST="$PREFIX/bin/ezoo-download"
ICON_DST="$PREFIX/share/icons/hicolor/256x256/apps/ezoo-download.png"
DESK_DST="$PREFIX/share/applications/ezoo-download.desktop"
AUTO_DST="$HOME/.config/autostart/ezoo-download.desktop"

BLUE='\033[1;34m'; RED='\033[1;31m'; OFF='\033[0m'
say() { printf "${BLUE}[ezoo]${OFF} %s\n" "$*"; }
die() { printf "${RED}[ezoo]${OFF} %s\n" "$*" >&2; exit 1; }

here="$(cd "$(dirname "$0")" && pwd)"
[ -x "$here/EzooDownload" ] || die "EzooDownload not found next to install.sh"

uninstall() {
  rm -f "$BIN_DST" "$ICON_DST" "$DESK_DST" "$AUTO_DST"
  command -v update-desktop-database >/dev/null 2>&1 && update-desktop-database "$PREFIX/share/applications" 2>/dev/null || true
  say "uninstalled / تمت الإزالة: $BIN_DST + أيقونة + مدخل القائمة"
}

[ "${1:-}" = "--uninstall" ] && { uninstall; exit 0; }
[ "${1:-}" = "" ] || die "unknown option: $1 (use: --uninstall)"

install -Dm755 "$here/EzooDownload" "$BIN_DST"
[ -f "$here/ezoo-download.png" ] && install -Dm644 "$here/ezoo-download.png" "$ICON_DST"
if [ -f "$here/ezoo-download.desktop" ]; then
  install -Dm644 <(sed "s|^Exec=.*|Exec=$BIN_DST|" "$here/ezoo-download.desktop") "$DESK_DST"
fi
command -v update-desktop-database >/dev/null 2>&1 && update-desktop-database "$PREFIX/share/applications" 2>/dev/null || true
command -v gtk-update-icon-cache   >/dev/null 2>&1 && gtk-update-icon-cache -qtf "$PREFIX/share/icons/hicolor" 2>/dev/null || true

say "installed ✓  $BIN_DST"
say "menu entry ✓ 'Ezoo Download' (قد تحتاج تسجيل خروج/دخول مرة واحدة لظهور القائمة)"
case ":$PATH:" in
  *":$PREFIX/bin:"*) : ;;
  *) say "تنبيه: أضف ~/.local/bin إلى PATH ليظهر البرنامج في الطرفية:" 
     say "  echo 'export PATH=\"\$HOME/.local/bin:\$PATH\"' >> ~/.bashrc" ;;
esac
command -v yt-dlp >/dev/null 2>&1 || say "تذكير: للأدوات المساعدة على مانجارو:  sudo pacman -S --needed yt-dlp ffmpeg aria2"
say "شغّله الآن:  ezoo-download   (أو من قائمة التطبيقات)"
