# Track Spec: Core Mod Management MVP

## Overview
This track implements the minimum viable product for the OpenMW Android Modloader. It focuses on the core ability to manage mods, sort load orders, and synchronize these changes with the OpenMW configuration files.

## Goals
- Establish a secure bridge between the Tauri frontend and the Android file system (via SAF).
- Parse and update `openmw.cfg` and `settings.cfg`.
- Provide a simple SvelteKit UI to list, toggle, and reorder mods.

## Core Components

### 1. Tauri File System Bridge
- **Purpose:** Provide high-level API for the frontend to interact with the Android storage.
- **Key Functions:**
    - `request_saf_permission(path: String)`: Triggers Android's Storage Access Framework dialog.
    - `list_mods(path: String)`: Scans a directory for mod folders/archives.
    - `read_config(path: String)`: Reads `openmw.cfg`.
    - `write_config(path: String, content: String)`: Safely writes to `openmw.cfg` with backup.

### 2. Config Parser (Rust/Backend)
- **Purpose:** Logic for handling OpenMW configuration files.
- **Features:**
    - Parse `data="path"` entries from `openmw.cfg`.
    - Parse `content=name.omwaddon` and `content=name.esm` entries.
    - Maintain internal representation of load order.
    - Serialize back to string format while preserving user comments (if possible) or at least ensuring valid format.

### 3. SvelteKit Frontend
- **Mod List View:** Display installed mods with checkboxes for activation.
- **Drag-and-Drop Sorting:** Allow users to reorder mods to change load order.
- **Save/Sync Button:** Trigger the write operation to the Android file system.

## Technical Constraints
- **Android Port Specifics:** Must respect the directory structure used by the OpenMW Android port.
- **Storage Access Framework:** Mandatory for accessing `/sdcard` or app-specific directories on modern Android.
- **Tauri Plugins:** May require custom Rust commands to interface with Android-specific APIs.

## Success Criteria
- User can select a mod directory.
- User can see a list of mods.
- User can reorder mods in the UI.
- Changes are reflected in `openmw.cfg` after saving.
- Application does not crash on permission denial.
