# Initial Concept
i want to make a modloader and organizer for the android port of openmw, want to make it using tauri and sveltekit

## Vision
To provide a robust, efficient, and user-friendly mod management experience for the Android port of OpenMW, empowering both casual players and advanced modders to easily customize their game.

## Target Users
- **Casual Players:** Users seeking a straightforward way to install and manage a few mods without diving into configuration files.
- **Advanced Modders:** Power users who require precise control over load orders, complex mod setups, and conflict resolution.

## Core Goals
- **Simplify Modding:** Streamline the process of installing, activating, and deactivating mods on Android.
- **Load Order Management:** Provide a clear interface for organizing and sorting the mod load order.
- **Conflict & Dependency Resolution:** Automatically identify and help resolve issues between overlapping or missing mods.

## Key Features
- **File Browser Integration:** Built-in file explorer to locate and import mod archives or folders.
- **Mod Profiles:** Create and switch between multiple mod configurations for different playthroughs or testing.
- **Modding Platform Integration:** Connect with services like Nexus Mods for direct mod discovery and potentially downloads.
- **Minimalist UI:** A high-density, functional interface built with Tauri and SvelteKit, focused on performance and utility.

## Technical Approach (Android)
- **Storage Access Framework (SAF):** Utilize Android's SAF to securely access OpenMW configuration and data directories with user permission.
- **Configuration Management:** Safely and directly modify `openmw.cfg` and `settings.cfg` while maintaining automatic backups of original files to prevent data loss.
