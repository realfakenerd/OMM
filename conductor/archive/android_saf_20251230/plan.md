# Plan: Android SAF Implementation (Tauri Plugin)

## Phase 1: Plugin Scaffolding & Setup
- [x] Task: Scaffold the new plugin using `bun run tauri plugin new --android saf`.
- [x] Task: Register the new plugin in `src-tauri/Cargo.toml` and `src-tauri/tauri.conf.json` (or `capabilities`).
- [x] Task: Register the plugin in `src-tauri/src/lib.rs` builder.
- [x] Task: Verify the plugin builds and links correctly with the main Android app.
- [x] Task: Conductor - User Manual Verification 'Phase 1: Plugin Scaffolding & Setup' (Protocol in workflow.md)

## Phase 2: Android Native Implementation (Kotlin)
- [x] Task: Implement `requestPermission` method in Kotlin (launching `ACTION_OPEN_DOCUMENT_TREE`).
- [x] Task: Implement `checkPermission` method in Kotlin (verifying persistent URI permissions).
- [x] Task: Implement `persistPermission` logic (calling `takePersistableUriPermission`).
- [x] Task: Implement `readFile`, `writeFile`, and `listDir` methods using `DocumentFile` and `ContentResolver`.
- [ ] Task: Conductor - User Manual Verification 'Phase 2: Android Native Implementation (Kotlin)' (Protocol in workflow.md)

## Phase 3: Rust Interface & Abstraction
- [x] Task: Define the Rust API for the plugin (commands matching the Kotlin implementation).
- [x] Task: Implement the cross-platform logic in Rust:
    - If `target_os = "android"`, call the JNI/Kotlin methods.
    - If `target_os != "android"`, implement standard `std::fs` fallbacks (or separate logic).
- [x] Task: Update the frontend (Svelte) to use the new plugin APIs instead of the old placeholder commands.
- [x] Task: Conductor - User Manual Verification 'Phase 3: Rust Interface & Abstraction' (Protocol in workflow.md)

## Phase 4: Integration & Verification
- [x] Task: Update the UI to check permission status on load (using the new API) and show a "Grant Permission" button if needed.
- [x] Task: Connect the "Grant Permission" button to the plugin's request command.
- [x] Task: Verify the full flow on an Android Emulator or Device (Select Folder -> Persist -> Read Config).
- [x] Task: Conductor - User Manual Verification 'Phase 4: Integration & Verification' (Protocol in workflow.md)
- [ ] Task: Connect the "Grant Permission" button to the plugin's request command.
- [ ] Task: Verify the full flow on an Android Emulator or Device (Select Folder -> Persist -> Read Config).
- [ ] Task: Conductor - User Manual Verification 'Phase 4: Integration & Verification' (Protocol in workflow.md)
