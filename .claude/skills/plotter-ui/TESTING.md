# UI Testing

## Basics

The `ui` package is configured to run tests with the `rstest` package, which exposes a Jest-compatible API (`expect`, `test`, etc, but imported from `@rstest/core`). Tests can be run in the `ui` folder with `pnpm test`.

Tests should be located within the `test` folder and be named `*.test.ts`.

### Unit Testing

Pure functions and code which does not depend on `.svelte` or `.svelte.ts` files should be unit tested straightforwardly.

### Component Testing

Additionally, the test environment is configured to use `js-dom` which can be used to test Svelte components. Rather than interacting with `js-dom` directly, tests should use the higher-level APIs exposed by `@testing-library/svelte` and simulate user-interaction with `@testing-library/user-event`. The test environment has been extended with custom matchers from `@testing-library/jest-dom` to make assertions about DOM elements.

## Reference Files

- **examples/** - Examples showing testing patterns
  - `Counter.svelte` - A simple button click counter component
  - `Counter.test.ts` - An example component test for `Counter.svelte` using `@testing-library/svelte`
