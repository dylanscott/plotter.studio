# Claude Code Instructions

## Important Documentation

- DESIGN_SPEC.md - Visual design and component specifications

## Packages

The application is written in a mix of TypeScript and Rust, and split into a few Node packages and crates:
- ui (node): The main web application package, built with Svelte 5 and bundled with rsbuild
- plotter-api (node): TypeScript types and utilities providing additional ergonomics around the WASM module
- plotter-rs (crate): Rust library implementing SVG parsing, path simplification, and plotter motion planning
- plotter-wasm (both a crate and a node package): Uses wasm-bindgen to expose plotter-rs functionality as a WASM module

## Required Patterns

### Version Control

Do not create commits on my behalf. I prefer to review and commit myself. You can (and should) stage the changes you make when completing a task.

### Before Completing Any Task

- If modifying Rust code
  - `cargo fmt --all`
  - `cargo clippy --all`. If there are fixable errors you can `cargo clippy --fix --all` but you will need to `--allow-dirty` or stage all changes and `--allow-staged`
- If modifying TypeScript/Svelte/CSS
  - `npx biome format --fix`
  - `npx biome check --fix`
