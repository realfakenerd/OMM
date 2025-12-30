# Track Spec: Add shadcn to the project to improve visuals

## Overview
This track involves integrating `shadcn-svelte` into the OpenMW Android Modloader project. The goal is to establish a consistent, professional, and accessible UI component library to enhance the user experience and speed up frontend development.

## Goals
- Integrate `shadcn-svelte` into the SvelteKit frontend.
- Configure the project to support shadcn's theming and utility classes (Tailwind CSS).
- Install a set of base components for immediate use in the UI.

## Functional Requirements
- **shadcn-svelte Initialization:** Run the CLI to set up shadcn-svelte in the project.
- **Component Installation:** Install the following components:
    - Button
    - Card
    - Input
    - Form
    - Dialog
- **Theming:** Set up the "Default" style with a "Slate" base color.
- **Tailwind Integration:** Ensure `tailwind.config.ts` and global CSS are correctly updated to work with shadcn.

## Non-Functional Requirements
- **Consistency:** All new UI elements should leverage shadcn components for visual coherence.
- **Touch-Friendliness:** Ensure components are appropriately sized for mobile use, adhering to the project's product guidelines.

## Technical Constraints
- **Tooling:** Use the Svelte MCP server (`list-sections`, `get-documentation`, `svelte-autofixer`) for guidance on best practices and component implementation during this track.

## Acceptance Criteria
- shadcn-svelte is successfully initialized without errors.
- The specified components (Button, Card, Input, Form, Dialog) are available in the project.
- A demo page or a simple integration into an existing page proves the components are styled and functional.
- The project's build and lint checks pass.

## Out of Scope
- Redesigning the entire existing UI using shadcn (this will be done in subsequent tracks).
- Implementing complex custom themes beyond the initial shadcn setup.
