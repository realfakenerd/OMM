# Tech Stack - OpenMW Android Modloader

## Core Frameworks
- **Frontend:** [SvelteKit](https://kit.svelte.dev/) (TypeScript/JavaScript) - For building a fast, reactive, and modern user interface.
- **Application Shell & Backend:** [Tauri](https://tauri.app/) (Rust) - To create the cross-platform application and provide high-performance access to the underlying system.
- **Android Support:** Tauri's official Android target for cross-compilation and deployment to mobile devices.

## Frontend Technologies
- **Styling:** [Tailwind CSS](https://tailwindcss.com/) - A utility-first CSS framework for rapid and consistent UI development.
- **State Management:** Svelte's built-in stores for efficient frontend state handling.

## Backend & Persistence
- **Database:** [SQLite](https://www.sqlite.org/) - A lightweight, embedded database for managing complex mod metadata, user profiles, and load order configurations.
- **Key-Value Store:** Tauri's `Store` API for simple persistence of application-wide settings and preferences.
- **Language:** Rust - Used for all backend logic, performance-critical operations (like file scanning and config parsing), and interfacing with Android APIs.

## Android-Specific Integration
- **Storage Access Framework (SAF):** Custom Rust implementation (possibly using `jni-rs`) to interact with Android's SAF, allowing the app to securely request and use permissions for OpenMW's data and configuration directories.
