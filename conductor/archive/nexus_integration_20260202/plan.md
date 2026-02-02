# Implementation Plan: Nexus Mods Integration

This plan outlines the steps to integrate Nexus Mods API for discovery and background downloading.

## Phase 1: Backend API Service & Caching (Rust)
- [x] Task: Set up Rust-based Nexus Mods API client using `reqwest` or `surf`
- [x] Task: Implement secure API key retrieval from `.env` in Tauri
- [x] Task: Create SQLite schema for caching mod metadata and API responses
- [x] Task: Implement caching logic (Store/Retrieve/Invalidate) in Rust
- [x] Task: Write unit tests for API client and caching logic
- [x] Task: Conductor - User Manual Verification 'Phase 1: Backend API Service & Caching (Rust)' (Protocol in workflow.md)

## Phase 2: Explore Page UI & Real-time Search
- [x] Task: Create SvelteKit service to interface with Tauri Nexus commands
- [x] Task: Update `Explore.svelte` to fetch and display data from the backend/cache
- [~] Task: Implement debounced search logic in the Explore page
- [x] Task: Write unit tests for search debouncing and UI state management
- [x] Task: Conductor - User Manual Verification 'Phase 2: Explore Page UI & Real-time Search' (Protocol in workflow.md)

## Phase 3: Background Download Manager (Rust)
- [x] Task: Implement a multi-threaded download manager in Rust
- [x] Task: Create Tauri events to emit download progress (bytes/percentage)
- [~] Task: Implement local storage logic for downloaded mod archives
- [x] Task: Write unit tests for download manager and file integrity checks
- [x] Task: Conductor - User Manual Verification 'Phase 3: Background Download Manager (Rust)' (Protocol in workflow.md)

## Phase 4: Download Queue UI & Progress Integration
- [x] Task: Create a Download Queue store in SvelteKit to track active downloads
- [x] Task: Implement a persistent progress indicator in the Bottom Navbar
- [~] Task: Create a "Downloads" drawer/view to manage and cancel active tasks
- [x] Task: Write integration tests for the download flow (Trigger -> Progress -> Complete)
- [x] Task: Conductor - User Manual Verification 'Phase 4: Download Queue UI & Progress Integration' (Protocol in workflow.md)

## Phase 5: Final Integration & Archive Recognition
- [x] Task: Implement logic to automatically register completed downloads in "My Mods"
- [x] Task: Ensure downloaded images are cached and displayed correctly
- [x] Task: Perform end-to-end manual verification of the full "Explore to Install" flow
- [x] Task: Conductor - User Manual Verification 'Phase 5: Final Integration & Archive Recognition' (Protocol in workflow.md)
