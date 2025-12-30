# Specification: Automate Android APK Build & CI/CD

## Overview
Establish a reliable, automated pipeline to build the Android APK for the OpenMW Android Modloader (OMM). This track focuses on configuring local build scripts and setting up a GitHub Actions workflow to build, sign (optional), and artifact the APK.

## Goals
- **Local Build Reliability:** Ensure developers can build the APK locally with a standardized command.
- **Automated CI/CD:** Implement a GitHub Actions workflow that automatically builds the APK on push/pull requests.
- **Artifact Management:** Automatically upload the built APKs as artifacts for easy testing and distribution.
- **Signing Preparation:** Configure the pipeline to support APK signing using GitHub Secrets.

## Functional Requirements

### 1. Local Build Configuration
- **Prerequisites Verification:** Document necessary tools (Bun, Rust, Android SDK, NDK).
- **Build Script:** Create a convenience script or `package.json` command (e.g., `npm run build:android`) that executes the correct Tauri build command (`bun tauri android build`).

### 2. GitHub Actions Workflow
- **Workflow File:** Create `.github/workflows/android-build.yml`.
- **Environment Setup:**
    - Checkout code.
    - Install Rust (stable).
    - Install Bun.
    - Setup Java (JDK 17).
    - Setup Android SDK and NDK.
- **Build Steps:**
    - Install dependencies (`bun install`).
    - Build frontend and backend.
    - Execute `bun tauri android build`.
- **Artifact Upload:**
    - Locate the generated APK(s) (typically in `src-tauri/gen/android/app/build/outputs/apk/universal/release/` or similar).
    - Upload them using `actions/upload-artifact`.

### 3. Signing (Optional/Configurable)
- **Secrets Integration:** Configure the workflow to look for signing secrets (KEYSTORE, KEY_ALIAS, KEY_PASSWORD, STORE_PASSWORD).
- **Signing Logic:** If secrets are present, sign the APK using `apksigner` or Gradle's signing configuration.

## Non-Functional Requirements
- **Performance:** Cache dependencies (Cargo, Bun, Gradle) to speed up build times.
- **Maintainability:** Use standard actions and clear step names.

## Acceptance Criteria
- [ ] A `build:android` script exists in `package.json` and works locally.
- [ ] A GitHub Action runs on push, successfully builds the APK, and uploads it as an artifact.
- [ ] The workflow gracefully handles missing signing secrets (skips signing or produces unsigned debug builds).
