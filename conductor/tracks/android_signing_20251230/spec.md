# Specification: Android APK Signing

## Overview
Configure a local signing process for the Android APK to allow installation on Android 15+ devices. This involves generating a keystore, configuring Gradle for signing, and updating build scripts.

## Goals
- **Local Signing:** Enable the `build:android:fast` command to produce a signed APK.
- **Security:** Ensure sensitive information (keystore passwords) is not committed to the repository.
- **Automation:** Standardize the signing process within the existing build pipeline.

## Functional Requirements
1. **Keystore Generation:** Generate a local `.jks` file for signing.
2. **Gradle Configuration:** Update `build.gradle.kts` to use the keystore for release builds.
3. **Environment Secrets:** Use a `local.properties` or environment variables for keystore passwords.
4. **Build Script Integration:** Ensure `bun run build:android:fast` produces a signed APK.

## Acceptance Criteria
- [ ] A local keystore exists.
- [ ] `bun run build:android:fast` produces a signed APK.
- [ ] The APK can be verified using `apksigner`.
