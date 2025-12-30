# Plan: Android APK Signing

## Phase 1: Keystore Generation
- [x] Task: Generate a local Android keystore.
- [x] Task: Create `local.properties` (if missing) and add keystore credentials.
- [x] Task: Verify the keystore is ignored by git.

## Phase 2: Gradle & Build Integration
- [x] Task: Configure `build.gradle.kts` to use signing credentials from `local.properties`.
- [x] Task: Update `build:android:fast` to ensure it produces a signed release APK.
- [x] Task: Verify the build produces a signed APK locally.
- [x] Task: Conductor - User Manual Verification 'Phase 2: Gradle & Build Integration' (Protocol in workflow.md)
