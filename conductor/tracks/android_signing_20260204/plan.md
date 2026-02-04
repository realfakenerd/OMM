# Plan: Android APK Signing (v2)

## Phase 1: Preparation
- [x] Task: Generate a new keystore file.
- [x] Task: Create `local.properties` with signing credentials.
- [x] Task: Ensure both are ignored by git (verify `.gitignore`).

## Phase 2: Configuration
- [x] Task: Update `src-tauri/gen/android/app/build.gradle.kts` to include `signingConfigs`.
- [x] Task: Link the `release` build type to the new `signingConfig`.

## Phase 3: Verification
- [x] Task: Run a release build.
- [x] Task: Verify the resulting APK is signed using `apksigner`.
