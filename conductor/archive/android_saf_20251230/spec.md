# Specification: Android SAF Implementation (Tauri Plugin)

## Overview
Implement the Android Storage Access Framework (SAF) integration for the OpenMW Android Modloader (OMM). This track focuses on creating a local Tauri plugin that abstracts file system operations on Android, allowing the application to access the OpenMW data directory and configuration files which are restricted by Android's scoped storage.

## Goals
- Provide a platform-agnostic API for file operations that works on both Desktop (std::fs) and Android (SAF).
- Implement a dedicated local Tauri plugin for Android-specific logic.
- Ensure persistent access to selected directories across application restarts.

## Functional Requirements

### 1. Local Tauri Plugin (`tauri-plugin-android-saf`)
- Scaffold a local plugin in `src-tauri/plugins/android-saf`.
- **Kotlin (Android):**
    - Implement a `request_directory_permission` method using `Intent.ACTION_OPEN_DOCUMENT_TREE`.
    - Implement logic to persist URI permissions using `takePersistableUriPermission`.
    - Implement file I/O helpers using `DocumentFile` and `ContentResolver` for listing, reading, and writing.
- **Rust (Plugin Interface):**
    - Define high-level commands for the frontend: `list_files`, `read_text_file`, `write_text_file`, `check_permission_status`.
    - Implement the glue code between Tauri and the native Android implementation.

### 2. File Operation Abstraction
- The plugin commands should detect the platform:
    - **On Android:** Use SAF/DocumentFile logic.
    - **On Desktop:** Fall back to standard `std::fs` operations.
- Update existing commands in `src-tauri/src/commands.rs` to either use the plugin or be replaced by plugin commands.

### 3. User Flow
- Permission request is triggered **on-demand** (e.g., when the user attempts to scan mods or read configuration if permissions aren't already held).
- Provide a `check_permission_status` command to allow the UI to show appropriate setup states.

## Non-Functional Requirements
- **Maintainability:** Use a modular plugin structure as per Tauri mobile development best practices.
- **Reliability:** Handle Android URI persistence correctly to avoid re-prompting the user unnecessarily.

## Acceptance Criteria
- [ ] Local Tauri plugin scaffolded and integrated into the main project.
- [ ] User can trigger the Android directory picker and select a folder.
- [ ] Permissions for the selected folder persist after restarting the app.
- [ ] The app can list files and read/write `openmw.cfg` on an Android device using the plugin API.
- [ ] Permission status is correctly reported to the frontend.

## Out of Scope
- Implementation of a full-featured file manager UI.
- Handling of specialized Android storage types like SD cards (focus is on primary external storage).
