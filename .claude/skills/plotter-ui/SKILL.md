---
name: plotter-ui-skill
description: Guidelines for implementing the pen plotter web application UI
---

# Pen Plotter UI Development Skill

This skill contains project-specific patterns and constraints for the pen plotter web application.

## Core Principles

- Always reference DESIGN_SPEC.md for visual design decisions
- Use modern Svelte 5 patterns (e.g. `$state` rune over `svelte/store` when possible). The Claude Svelte plugin (which includes a `svelte-code-writer` skill) should be installed, as well as the Svelte MCP server
- Use relevant types from `plotter-api`

## Svelte Patterns

### TypeScript

All Svelte components should use `<script lang="ts">` and annotate the types of component props.

### Testing

Refer to [TESTING.md](TESTING.md) for a guide

## Blueprint Theme

### Colors

Always use CSS variables defined in DESIGN_SPEC.md:
```css
color: var(--accent-cyan);
background: var(--bg-primary);
border: 1px solid var(--border-subtle);
```

### Typography

All text should be monospace:
```css
font-family: 'IBM Plex Mono', monospace;
```

### Spacing

Use 8px base unit for spacing:
- Small: 8px
- Medium: 16px
- Large: 24px

## Accessibility

- All icon buttons need aria-label
- Collapsible sections need aria-expanded
- Form inputs need associated labels
- Keyboard navigation for tree view
