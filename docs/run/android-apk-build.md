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

Set the API base at build time using `API_BASE` (do not hardcode it in code or commit it):

```bash
export API_BASE="http://<your-server-host>:3000"
```

The Android `network_security_config.xml` is generated during build from `API_BASE`,
so HTTP hosts are allowed automatically. Keep the server out of git by not committing
the generated `frontend/src-tauri/gen` files.

## 3) Initialize Android project (first time only)

Run from repo root:

```bash
cargo tauri android init
```

This generates `frontend/src-tauri/gen/android`.

## 4) Generate release keystore (first time only)

Release builds need a signing key. Generate it after `android init`:

```bash
./scripts/generate-android-keystore.sh
```

This writes:
- `frontend/src-tauri/gen/android/release-key.jks`
- `frontend/src-tauri/gen/android/keystore.properties`

Both are already ignored by git.

## 5) Build APK

Release APK (universal, "big package"):

```bash
API_BASE="http://<your-server-host>:3000" cargo tauri android build --apk true
```

APK path (release, universal):

`frontend/src-tauri/gen/android/app/build/outputs/apk/universal/release/app-universal-release.apk`

## 6) Install on device

```bash
adb install -r frontend/src-tauri/gen/android/app/build/outputs/apk/universal/release/app-universal-release.apk
```

## Troubleshooting

- If build fails, confirm `ANDROID_HOME`, `NDK_HOME`, `JAVA_HOME` are correct.
- For HTTP requests blocked, ensure `usesCleartextTraffic="true"` is set.
- If you change API_BASE, rebuild the APK.
