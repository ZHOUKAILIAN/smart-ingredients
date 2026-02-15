#!/usr/bin/env bash
# Build Android APK for emulator or device.
# Usage:
#   ./scripts/build-android.sh          # debug APK (default)
#   ./scripts/build-android.sh release  # release APK
#   ./scripts/build-android.sh install  # debug APK + install + launch on emulator

set -euo pipefail

# ── Android emulator uses 10.0.2.2 to reach host machine ──
export API_BASE="http://10.0.2.2:3000"
export ANDROID_HOME="${ANDROID_HOME:-$HOME/Library/Android/sdk}"
export NDK_HOME="${NDK_HOME:-$ANDROID_HOME/ndk/$(ls "$ANDROID_HOME/ndk/" | tail -1)}"
export JAVA_HOME="${JAVA_HOME:-/Applications/Android Studio.app/Contents/jbr/Contents/Home}"

cd "$(dirname "$0")/.."

MODE="${1:-debug}"

echo "🔧 API_BASE=$API_BASE"
echo "🔧 JAVA_HOME=$JAVA_HOME"
echo "🔧 NDK_HOME=$NDK_HOME"
echo ""

case "$MODE" in
  release)
    echo "📦 Building release APK..."
    cargo tauri android build --apk true
    APK_PATH="frontend/src-tauri/gen/android/app/build/outputs/apk/universal/release/app-universal-release.apk"
    ;;
  install)
    echo "📦 Building debug APK..."
    cargo tauri android build --apk true --debug
    APK_PATH="frontend/src-tauri/gen/android/app/build/outputs/apk/universal/debug/app-universal-debug.apk"
    echo ""
    echo "📲 Installing on emulator..."
    adb install -r "$APK_PATH"
    echo "🚀 Launching app..."
    adb shell am force-stop com.smart_ingredients.app
    adb shell am start -n com.smart_ingredients.app/.MainActivity
    echo "✅ Done!"
    exit 0
    ;;
  *)
    echo "📦 Building debug APK..."
    cargo tauri android build --apk true --debug
    APK_PATH="frontend/src-tauri/gen/android/app/build/outputs/apk/universal/debug/app-universal-debug.apk"
    ;;
esac

echo ""
echo "✅ APK built at: $APK_PATH"
echo ""
echo "To install on emulator:"
echo "  adb install -r $APK_PATH"
echo "  adb shell am start -n com.smart_ingredients.app/.MainActivity"
