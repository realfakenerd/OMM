# Tech Stack - OpenMW Android Modloader

## Core Frameworks
- **Frontend:** [SvelteKit](https://kit.svelte.dev/) (TypeScript/JavaScript) - For building a fast, reactive, and modern user interface.
- **Application Shell & Backend:** [Tauri](https://tauri.app/) (Rust) - To create the cross-platform application and provide high-performance access to the underlying system.
- **Android Storage Integration:** Local Tauri Plugin (`tauri-plugin-android-saf`) - Custom implementation using Android Storage Access Framework (SAF) for persistent file system access.
- **Android Support:** Tauri's official Android target for cross-compilation and deployment to mobile devices.

## Frontend Technologies
- **Styling:** [Tailwind CSS](https://tailwindcss.com/) - A utility-first CSS framework for rapid and consistent UI development.
- **State Management:** Svelte's built-in stores for efficient frontend state handling.
- **UI Components:** [shadcn-svelte](https://shadcn-svelte.com/) - A collection of re-usable components built with Bits UI and Tailwind CSS.

## Backend & Persistence
- **Database:** [SQLite](https://www.sqlite.org/) - A lightweight, embedded database for managing complex mod metadata, user profiles, and load order configurations.
- **Key-Value Store:** Tauri's `Store` API for simple persistence of application-wide settings and preferences.
- **Language:** Rust - Used for all backend logic, performance-critical operations (like file scanning and config parsing), and interfacing with Android APIs.

## Android-Specific Integration
- **Storage Access Framework (SAF):** Custom Rust implementation (possibly using `jni-rs`) to interact with Android's SAF, allowing the app to securely request and use permissions for OpenMW's data and configuration directories.

## CI/CD & Build System
- **Local Build Automation:** Standardized build scripts via `package.json` for Android APK generation (including optimized `aarch64` builds).
- **APK Signing:** Local signing process using `keytool` for keystore generation, Gradle configuration for release builds, and `apksigner` for verification.
- **Automated Pipeline:** GitHub Actions workflow (`android-build.yml`) for continuous integration, environment setup (Rust, Bun, Java, NDK), and artifact generation.
- **Environment:** OpenJDK 17, Android SDK (API 35+), NDK 27.

## Development Tooling
- **Svelte MCP:** A Model Context Protocol server for Svelte and SvelteKit. It should be used whenever possible for consulting documentation, best practices, and component implementation.
