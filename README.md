# Tauri + Leptos

This template should help get you started developing with Tauri and Leptos.

## Recommended IDE Setup

[VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer).

## Android Build

### Environment Setup
Before building for Android, ensure you have the necessary environment variables set:

```bash
export NDK_HOME=/opt/android-sdk/ndk/25.2.9519653
export PATH=$PATH:$NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64/bin
```

### Build Commands

#### Generate Debug APK (Signed with debug key)
This version is ready for testing on devices.
```bash
cargo tauri android build --apk --debug
```
**Path:** `src-tauri/gen/android/app/build/outputs/apk/universal/debug/app-universal-debug.apk`

#### Generate Release APK (Unsigned)
This version must be signed manually before installation.
```bash
cargo tauri android build --apk
```
**Path:** `src-tauri/gen/android/app/build/outputs/apk/universal/release/app-universal-release-unsigned.apk`
