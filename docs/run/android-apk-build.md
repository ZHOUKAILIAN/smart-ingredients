# Android APK Build Guide (Tauri 2)

This project uses Tauri 2. To build an APK you need Android SDK/NDK + JDK.
Do not paste API keys or passwords into chat or git.

## 1) Install dependencies

- Android Studio (SDK + NDK)
- JDK 17
- Rust (with Android targets)
- Tauri CLI

```bash
# Rust targets
rustup target add aarch64-linux-android armv7-linux-androideabi i686-linux-android x86_64-linux-android

# Tauri CLI (v2)
cargo install tauri-cli --version "^2"
```

Set env vars (adjust paths):

```bash
export ANDROID_HOME="$HOME/Library/Android/sdk"
export NDK_HOME="$ANDROID_HOME/ndk/<your-ndk-version>"
export JAVA_HOME="/Library/Java/JavaVirtualMachines/jdk-17.jdk/Contents/Home"
export PATH="$ANDROID_HOME/platform-tools:$ANDROID_HOME/emulator:$PATH"
```

## 2) Configure backend API address

The frontend currently uses a hardcoded API base:

`frontend/src/services/mod.rs`

Update it to your server before building:

```rust
const API_BASE: &str = "http://xxxxxx:3000";
```

If you keep HTTP (not HTTPS), Android 9+ needs cleartext enabled:

```bash
cargo tauri android init
```

Then edit:

`frontend/src-tauri/gen/android/app/src/main/AndroidManifest.xml`

Add `android:usesCleartextTraffic="true"` inside `<application ...>`.

## 3) Initialize Android project (first time only)

Run from repo root:

```bash
cargo tauri android init
```

This generates `frontend/src-tauri/gen/android`.

## 4) Build APK

Debug APK:

```bash
cargo tauri android build --apk
```

APK path (debug):

`frontend/src-tauri/gen/android/app/build/outputs/apk/debug/app-debug.apk`

Release APK:

```bash
cargo tauri android build --apk --release
```

APK path (release):

`frontend/src-tauri/gen/android/app/build/outputs/apk/release/app-release.apk`

## 5) Install on device

```bash
adb install -r frontend/src-tauri/gen/android/app/build/outputs/apk/debug/app-debug.apk
```

## Troubleshooting

- If build fails, confirm `ANDROID_HOME`, `NDK_HOME`, `JAVA_HOME` are correct.
- For HTTP requests blocked, ensure `usesCleartextTraffic="true"` is set.
- If you change API_BASE, rebuild the APK.

