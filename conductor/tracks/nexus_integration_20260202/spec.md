# Specification: Nexus Mods Integration (Explore Page)

## Overview
Integrate the "Explore" page with the Nexus Mods API to allow users to discover, search, and download Morrowind mods directly within the application. This track covers the API integration, real-time search functionality, a background download queue system, and intelligent caching to optimize performance.

## Functional Requirements

### 1. Nexus Mods API Integration
- Use the API key stored in the `.env` file to authenticate requests to the Nexus Mods API.
- Fetch featured, popular (all-time and 30-day), and recently updated mods.
- Support category-based filtering and sorting by endorsements/rating.
- **Caching Mechanism:** 
    - Cache API responses (e.g., featured/popular lists) locally (SQLite or File System) to reduce redundant requests and ensure offline availability of recently viewed data.
    - Implement cache invalidation (e.g., refresh data older than 24 hours).
    - Cache mod thumbnails and images locally after the first load.

### 2. Explore Page UI Enhancements
- Display mod cards with:
    - High-quality thumbnail/preview image.
    - Statistics: Download count, Endorsement count, and Last Updated date.
    - Brief mod description/summary.
    - "Add to Library" or "Download" action button.
- Implement a real-time search bar with debouncing (300-500ms) to query Nexus Mods.

### 3. Background Download Queue
- Implement a centralized download manager to handle concurrent downloads in the background.
- Provide a progress indicator (e.g., in the Bottom Navbar or a persistent toast).
- Create a dedicated "Downloads" view or drawer to monitor and manage the queue.
- Automatically move successfully downloaded mods to the "My Mods" or "Library" section.

## Non-Functional Requirements
- **Performance:** Debounce search inputs and utilize local cache to ensure a fluid UI experience.
- **Resilience:** Handle API rate limits and network failures gracefully with user-friendly error messages.
- **Security:** Ensure the API key is handled securely in the backend (Tauri/Rust).

## Acceptance Criteria
- [ ] Users can browse featured and popular mods from Nexus on the Explore page.
- [ ] Real-time search successfully returns relevant results from Nexus Mods.
- [ ] Data is cached locally and persists between sessions, reducing load times.
- [ ] Users can filter mods by category and sort by popularity or rating.
- [ ] Initiating a download starts a background process that updates progress in real-time.
- [ ] Downloads can be managed in a queue without blocking app navigation.
- [ ] Completed downloads are automatically recognized as local mod files.

## Out of Scope
- Nexus Mods user login/authentication (OAuth) - only public API access for now.
- Automatic installation of complex mods requiring manual user intervention (e.g., those with external installers).
