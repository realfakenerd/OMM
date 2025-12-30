# Track Plan: Core Mod Management MVP

This plan outlines the steps to implement the core mod management functionality for the OpenMW Android Modloader.

## Phase 1: Project Initialization & Scaffolding
- [x] Task: Initialize Tauri project with SvelteKit frontend c61a2a0
- [x] Task: Set up basic Rust backend structure ee2ae2b
- [ ] Task: Conductor - User Manual Verification 'Phase 1: Project Initialization & Scaffolding' (Protocol in workflow.md)

## Phase 2: Android File System Bridge (Tauri/Rust)
- [ ] Task: Write tests for SAF permission handling and basic file I/O commands
- [ ] Task: Implement `request_saf_permission` Tauri command
- [ ] Task: Implement basic file read/write commands for Android SAF
- [ ] Task: Conductor - User Manual Verification 'Phase 2: Android File System Bridge (Tauri/Rust)' (Protocol in workflow.md)

## Phase 3: OpenMW Config Parser (Rust)
- [ ] Task: Write tests for `openmw.cfg` parsing and serialization
- [ ] Task: Implement `openmw.cfg` parser (extracting `data` and `content` lines)
- [ ] Task: Implement logic to update load order and data paths in memory
- [ ] Task: Implement backup mechanism before writing config files
- [ ] Task: Conductor - User Manual Verification 'Phase 3: OpenMW Config Parser (Rust)' (Protocol in workflow.md)

## Phase 4: Frontend Mod Management UI (SvelteKit)
- [ ] Task: Write tests for Mod List component and state management
- [ ] Task: Create basic UI for listing mods detected in data directories
- [ ] Task: Implement mod toggle (active/inactive) logic in UI state
- [ ] Task: Implement drag-and-drop reordering for mod load order
- [ ] Task: Conductor - User Manual Verification 'Phase 4: Frontend Mod Management UI (SvelteKit)' (Protocol in workflow.md)

## Phase 5: End-to-End Integration
- [ ] Task: Write integration tests for the full flow (Load config -> Change Order -> Save)
- [ ] Task: Connect SvelteKit UI to Tauri commands for reading/writing config
- [ ] Task: Implement error handling and user notifications for permission/IO failures
- [ ] Task: Conductor - User Manual Verification 'Phase 5: End-to-End Integration' (Protocol in workflow.md)
