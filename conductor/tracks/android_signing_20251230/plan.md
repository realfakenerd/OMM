# Plan: Android APK Signing

## Phase 1: Keystore Generation
- [x] Task: Generate a local Android keystore.
- [x] Task: Create `local.properties` (if missing) and add keystore credentials.
- [x] Task: Verify the keystore is ignored by git.

## Phase 2: Gradle & Build Integration
- [~] Task: Configure `build.gradle.kts` to use signing credentials from `local.properties`.
- [ ] Task: Update `build:android:fast` to ensure it produces a signed release APK.
- [ ] Task: Verify the build produces a signed APK locally.
- [ ] Task: Conductor - User Manual Verification 'Phase 2: Gradle & Build Integration' (Protocol in workflow.md)
