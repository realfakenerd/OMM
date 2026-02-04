# Specification: Android APK Signing (v2)

## Overview
Re-configure the local signing process for the Android APK. The previous configuration seems to have been lost or is missing. This is required for installation on modern Android devices (especially Android 15+).

## Goals
- **Local Signing:** Enable `tauri android build` to produce a signed APK locally.
- **Persistence:** Ensure the signing configuration is correctly integrated into `build.gradle.kts`.
- **Security:** Use `local.properties` for sensitive credentials (ignored by git).

## Functional Requirements
1. **Keystore Generation:** Generate a new `.jks` file in the Android project.
2. **Credential Storage:** Store passwords in `src-tauri/gen/android/local.properties`.
3. **Gradle Integration:** Update `src-tauri/gen/android/app/build.gradle.kts` to load these properties and apply `signingConfigs`.

## Acceptance Criteria
- [ ] `src-tauri/gen/android/app/upload-keystore.jks` exists.
- [ ] `src-tauri/gen/android/local.properties` contains the necessary credentials.
- [ ] `src-tauri/gen/android/app/build.gradle.kts` has a `signingConfigs` block.
- [ ] `bun run build:android:fast` (or `tauri android build`) produces a signed APK (not ending in `-unsigned.apk`).
