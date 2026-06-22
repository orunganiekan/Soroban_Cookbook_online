# Implementation Plan: Homepage QuickStart Section

## Overview

Implement the `QuickStartSection` component with a `CopyButton` sub-component, integrate it into the homepage, and add a CSS module with theme-aware styles. Tests use Jest + React Testing Library for unit tests and `fast-check` for property-based tests.

## Tasks

- [x] 1. Create the QuickStartSection component files
  - Create `documentation/src/components/QuickStartSection/index.tsx` with the `SNIPPET` constant, `CopyButton` internal sub-component, and `QuickStartSection` default export
  - `CopyButton` must manage `CopyState` (`idle | success | error`) via `useState`, use `useRef` for the timeout ID, and clean up on unmount via `useEffect`
  - `QuickStartSection` must use `useColorMode` to select `prismThemes.github` (light) or `prismThemes.vsDark` (dark) and pass it to `Highlight` from `prism-react-renderer`
  - Render heading, `Highlight`-based code block (read-only, no `contentEditable`), `CopyButton`, and a `Link` CTA to `/docs/getting-started/setup`
  - _Requirements: 1.1, 1.2, 1.3, 1.4, 2.1, 2.2, 2.3, 2.4, 3.1, 4.1, 4.2, 4.3, 4.4_

  - [ ]* 1.1 Write unit tests for QuickStartSection structure
    - Assert section renders heading, exactly one code block, one copy button, one CTA link
    - Assert CTA `href` is `/docs/getting-started/setup` with no `target="_blank"`
    - Assert code block has no `contentEditable` attribute
    - Assert Prism token `<span>` elements are present in rendered output
    - _Requirements: 1.1, 1.2, 1.3, 1.4, 2.4, 4.2, 4.3_

- [x] 2. Implement CopyButton clipboard logic and accessible labels
  - Wire `navigator.clipboard.writeText` call inside a `try/catch` in the click handler
  - Map `CopyState` to visible label (`Copy` / `Copied!` / `Failed`) and `aria-label` (`Copy code` / `Code copied` / `Copy failed`)
  - Disable the button while state is `success` or `error`
  - Reset state to `idle` after 2000 ms; clear timeout on unmount
  - _Requirements: 3.2, 3.3, 3.4, 3.5, 3.6_

  - [ ]* 2.1 Write property test for copy button writes full snippet (Property 2)
    - **Property 2: Copy button writes full snippet to clipboard**
    - **Validates: Requirements 3.2**

  - [ ]* 2.2 Write property test for feedback label and revert (Property 3)
    - **Property 3: Copy feedback label and revert after 2000 ms**
    - **Validates: Requirements 3.3, 3.4**

  - [ ]* 2.3 Write property test for button disabled during feedback (Property 4)
    - **Property 4: Button disabled during feedback window**
    - **Validates: Requirements 3.5**

  - [ ]* 2.4 Write property test for aria-label reflects copy state (Property 5)
    - **Property 5: aria-label reflects copy state**
    - **Validates: Requirements 3.6**

- [x] 3. Create the CSS module with theme-aware styles
  - Create `documentation/src/components/QuickStartSection/styles.module.css`
  - Use only Docusaurus CSS custom properties (`--ifm-*`) for all color values — no hardcoded hex colors
  - Section container: background `var(--ifm-background-color)`, border `var(--ifm-border-color)`
  - Code block wrapper: background `var(--ifm-background-surface-color)`, `overflow-x: auto` for mobile scrolling
  - Copy button: minimum `44px × 44px` touch target, hover/focus styles via `--ifm-color-primary`
  - CTA link: visible focus indicator
  - _Requirements: 5.1, 5.2, 5.3, 5.4, 6.1, 6.2, 6.3, 6.4_

  - [ ]* 3.1 Write unit tests for CSS module constraints
    - Assert CSS module source contains no hardcoded hex color values (regex check)
    - Assert code block container has `overflow-x: auto` computed style
    - Assert copy button computed min-width and min-height are ≥ 44 px
    - _Requirements: 5.2, 5.4, 6.2, 6.3_

  - [ ]* 3.2 Write property test for no horizontal overflow (Property 6)
    - **Property 6: No horizontal overflow across viewport widths 320–1440 px**
    - **Validates: Requirements 5.1**

- [x] 4. Checkpoint — Ensure all tests pass
  - Ensure all tests pass, ask the user if questions arise.

- [~] 5. Add theme-awareness property tests
  - [ ]* 5.1 Write property test for theme selects correct Prism theme (Property 1)
    - **Property 1: Theme selects correct Prism theme**
    - Mock `useColorMode` with `fc.constantFrom('light', 'dark')`, assert `Highlight` receives `prismThemes.github` for light and `prismThemes.vsDark` for dark
    - **Validates: Requirements 2.2, 2.3**

  - [ ]* 5.2 Write property test for theme change updates colors reactively (Property 7)
    - **Property 7: Theme change updates section colors reactively**
    - Toggle `data-theme` attribute on `document.documentElement`, assert CSS custom properties resolve to theme-appropriate values without reload
    - **Validates: Requirements 6.1**

  - [ ]* 5.3 Write property test for code block background distinct from section background (Property 8)
    - **Property 8: Code block background is visually distinct from section background**
    - For each theme, assert computed background of code block element differs from computed background of section container
    - **Validates: Requirements 6.4**

- [x] 6. Integrate QuickStartSection into the homepage
  - In `documentation/src/pages/index.tsx`, import `QuickStartSection` and render it between the `PatternPreview`/`Stats` block and `<NewsletterSignup />`
  - Ensure it renders outside the loading skeleton (always visible, not gated by `isLoading`)
  - _Requirements: 1.1_

- [x] 7. Final checkpoint — Ensure all tests pass
  - Ensure all tests pass, ask the user if questions arise.

## Notes

- Tasks marked with `*` are optional and can be skipped for a faster MVP
- Property tests use `fast-check` (`fc`) with `numRuns: 100`; fake timers (`jest.useFakeTimers`) are required for Properties 3 and 4
- All 8 correctness properties from the design document are covered by sub-tasks 2.1–2.4 and 5.1–5.3 plus 3.2
- `navigator.clipboard` must be mocked in the test environment (jsdom does not implement it)
