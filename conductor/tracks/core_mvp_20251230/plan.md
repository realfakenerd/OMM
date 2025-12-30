# Track Plan: Core Mod Management MVP

This plan outlines the steps to implement the core mod management functionality for the OpenMW Android Modloader.

## Phase 1: Project Initialization & Scaffolding [checkpoint: 35d4507]
- [x] Task: Initialize Tauri project with SvelteKit frontend c61a2a0
- [x] Task: Set up basic Rust backend structure ee2ae2b
- [x] Task: Conductor - User Manual Verification 'Phase 1: Project Initialization & Scaffolding' (Protocol in workflow.md) 35d4507

## Phase 2: Android File System Bridge (Tauri/Rust) [checkpoint: a603059]
- [x] Task: Write tests for SAF permission handling and basic file I/O commands 578ce9c
- [x] Task: Implement `request_saf_permission` Tauri command 7f1a5ec
- [x] Task: Implement basic file read/write commands for Android SAF 9ac8d98
- [x] Task: Conductor - User Manual Verification 'Phase 2: Android File System Bridge (Tauri/Rust)' (Protocol in workflow.md) a603059

## Phase 3: OpenMW Config Parser (Rust) [checkpoint: a6d13ac]
- [x] Task: Write tests for `openmw.cfg` parsing and serialization 1d3f412
- [x] Task: Implement `openmw.cfg` parser (extracting `data` and `content` lines) d41bec2
- [x] Task: Implement logic to update load order and data paths in memory 2580c79
- [x] Task: Implement backup mechanism before writing config files b3e5e44
- [x] Task: Conductor - User Manual Verification 'Phase 3: OpenMW Config Parser (Rust)' (Protocol in workflow.md) a6d13ac

## Phase 4: Frontend Mod Management UI (SvelteKit) [checkpoint: 0f4a310]
- [x] Task: Write tests for Mod List component and state management 6d34325
- [x] Task: Create basic UI for listing mods detected in data directories 823c301
- [x] Task: Implement mod toggle (active/inactive) logic in UI state e3723d9
- [x] Task: Implement drag-and-drop reordering for mod load order 60aa51a
- [x] Task: Conductor - User Manual Verification 'Phase 4: Frontend Mod Management UI (SvelteKit)' (Protocol in workflow.md) 0f4a310

## Phase 5: End-to-End Integration
- [x] Task: Write integration tests for the full flow (Load config -> Change Order -> Save) ad985a4
- [x] Task: Connect SvelteKit UI to Tauri commands for reading/writing config bff58ee
- [ ] Task: Implement error handling and user notifications for permission/IO failures
- [ ] Task: Conductor - User Manual Verification 'Phase 5: End-to-End Integration' (Protocol in workflow.md)
