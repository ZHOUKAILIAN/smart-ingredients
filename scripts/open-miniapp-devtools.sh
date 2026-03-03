#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/.." && pwd)"
MINIAPP_DIR="$ROOT_DIR/miniapp"
PROXY_DIR="$HOME/WeChatProjects/smart-ingredients-miniapp-proxy"
APP_PATH="/Applications/wechatwebdevtools.app"

if [[ ! -d "$MINIAPP_DIR" ]]; then
  echo "miniapp directory not found: $MINIAPP_DIR" >&2
  exit 1
fi

mkdir -p "$PROXY_DIR"

# Keep the proxy directory small so DevTools does not scan the monorepo root.
# Use real files/dirs (not symlinks) to avoid DevTools path resolution issues.
ITEMS=(
  "src"
  "config"
  "dist"
  "package.json"
  "package-lock.json"
  "babel.config.js"
  "project.config.json"
  "project.private.config.json"
)

for item in "${ITEMS[@]}"; do
  src="$MINIAPP_DIR/$item"
  dst="$PROXY_DIR/$item"
  rm -rf "$dst"
  if [[ -d "$src" ]]; then
    mkdir -p "$dst"
    rsync -a --delete "$src/" "$dst/"
  else
    cp -f "$src" "$dst"
  fi
done

pkill -f wechatwebdevtools || true
sleep 1

if [[ -d "$APP_PATH" ]]; then
  open -a "$APP_PATH" "$PROXY_DIR"
else
  open -a "wechatwebdevtools" "$PROXY_DIR"
fi

echo "Opened WeChat DevTools with proxy project: $PROXY_DIR"
echo "Source project: $MINIAPP_DIR"
