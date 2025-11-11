# Agent Guidelines for homedo

## Build/Test Commands

- **Rust backend**: `cargo build`, `cargo run`, `cargo test`
- **Frontend dev**: `cd static && npm run dev`
- **Frontend build**: `cd static && npm run build`
- **Type check**: `cd static && npm run check`

## Code Style

### Rust

- Edition 2024, use async/await with Tokio
- Use axum for routing
- Follow standard Rust naming: snake_case for functions/variables, PascalCase for types

### TypeScript/Svelte

- Strict mode enabled, use explicit types
- Use SvelteKit conventions: `+page.svelte`, `+layout.svelte`
- Import from `$lib` for shared code
- Use Svelte 5 runes syntax: `$props()`, `$state()`, `{@render ...}`
- Tab indentation (per existing files)
- Use `<script lang="ts">` for all Svelte components
