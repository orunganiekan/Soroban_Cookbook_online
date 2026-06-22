# Requirements Document

## Introduction

Add a "Quick Start" section to the Soroban Cookbook homepage that gives developers an immediate, hands-on feel for Soroban. The section displays a compact, syntax-highlighted code snippet (a minimal Soroban contract), a copy-to-clipboard button with success feedback, and a call-to-action link pointing to the full setup/onboarding docs. The section must be responsive across all viewport sizes and respect the site's light/dark color mode.

## Glossary

- **QuickStart_Section**: The homepage UI section containing the code snippet, copy button, and CTA link.
- **Code_Block**: The syntax-highlighted, read-only code display element within the QuickStart_Section.
- **Copy_Button**: The interactive button that copies the Code_Block content to the system clipboard.
- **CTA_Link**: The call-to-action anchor element that navigates users to the full onboarding documentation.
- **Clipboard_API**: The browser's `navigator.clipboard` interface used to write text to the system clipboard.
- **Prism**: The syntax-highlighting library already configured in the Docusaurus site (`prism-react-renderer`).
- **Theme**: The active Docusaurus color mode — either `light` or `dark`.
- **Viewport**: The visible browser window area, categorized as mobile (< 768 px), tablet (768–1024 px), or desktop (> 1024 px).

---

## Requirements

### Requirement 1: Display the Quick Start Section on the Homepage

**User Story:** As a developer visiting the Soroban Cookbook homepage, I want to see a compact quick-start code example, so that I can immediately understand what a Soroban contract looks like without navigating away.

#### Acceptance Criteria

1. THE QuickStart_Section SHALL be rendered on the homepage below the hero/features area and above the footer.
2. THE QuickStart_Section SHALL include a visible heading (e.g., "Quick Start") that identifies the section's purpose.
3. THE QuickStart_Section SHALL contain exactly one Code_Block displaying a minimal, valid Soroban contract snippet.
4. THE Code_Block SHALL be read-only and SHALL NOT allow user text editing.

---

### Requirement 2: Syntax Highlighting

**User Story:** As a developer, I want the code snippet to be syntax-highlighted, so that I can read it quickly and understand its structure at a glance.

#### Acceptance Criteria

1. THE Code_Block SHALL render the snippet with syntax highlighting using the Prism library already configured in the Docusaurus site.
2. WHEN the active Theme is `light`, THE Code_Block SHALL apply the Prism `github` theme token colors.
3. WHEN the active Theme is `dark`, THE Code_Block SHALL apply the Prism `vsDark` theme token colors.
4. THE Code_Block SHALL highlight Rust language tokens (keywords, types, strings, comments) as distinct visual elements.

---

### Requirement 3: Copy to Clipboard

**User Story:** As a developer, I want to copy the code snippet with a single click, so that I can paste it into my project without manually selecting text.

#### Acceptance Criteria

1. THE QuickStart_Section SHALL contain a Copy_Button visually associated with the Code_Block.
2. WHEN a user activates the Copy_Button, THE Copy_Button SHALL write the full Code_Block content to the system clipboard via the Clipboard_API.
3. WHEN the clipboard write succeeds, THE Copy_Button SHALL display a success label (e.g., "Copied!") for a duration of 2000 ms, then revert to its default label (e.g., "Copy").
4. IF the Clipboard_API is unavailable or the write operation fails, THEN THE Copy_Button SHALL display an error label (e.g., "Failed") for a duration of 2000 ms, then revert to its default label.
5. WHILE the Copy_Button is displaying the success or error label, THE Copy_Button SHALL be disabled to prevent duplicate activations.
6. THE Copy_Button SHALL have an accessible name (via `aria-label`) that reflects its current state (e.g., "Copy code", "Code copied", "Copy failed").

---

### Requirement 4: Call-to-Action Link

**User Story:** As a developer, I want a clear link to the full setup documentation, so that I can continue onboarding after seeing the quick-start snippet.

#### Acceptance Criteria

1. THE QuickStart_Section SHALL contain exactly one CTA_Link.
2. THE CTA_Link SHALL navigate to the Soroban Cookbook getting-started setup page (`/docs/getting-started/setup`).
3. THE CTA_Link SHALL open in the same browser tab (no `target="_blank"`).
4. THE CTA_Link SHALL have descriptive visible text (e.g., "Read the full setup guide →") that communicates its destination.
5. THE CTA_Link SHALL be keyboard-focusable and SHALL have a visible focus indicator.

---

### Requirement 5: Responsive Layout

**User Story:** As a developer on any device, I want the Quick Start section to be readable and usable, so that I can access it on mobile, tablet, or desktop.

#### Acceptance Criteria

1. THE QuickStart_Section SHALL render without horizontal overflow at viewport widths from 320 px to 1440 px.
2. WHEN the Viewport is mobile (< 768 px), THE Code_Block SHALL be horizontally scrollable if the snippet content exceeds the container width, rather than wrapping or truncating tokens.
3. WHEN the Viewport is tablet or desktop (≥ 768 px), THE QuickStart_Section SHALL display the Code_Block and CTA_Link in a layout that uses available horizontal space effectively.
4. THE Copy_Button SHALL remain accessible and tappable at all Viewport sizes with a minimum touch target of 44 × 44 px.

---

### Requirement 6: Theme Awareness

**User Story:** As a developer using light or dark mode, I want the Quick Start section to match the site's active theme, so that it feels visually consistent with the rest of the page.

#### Acceptance Criteria

1. WHEN the active Theme changes, THE QuickStart_Section SHALL update its background, text, and border colors to match the new Theme without requiring a page reload.
2. THE QuickStart_Section SHALL use Docusaurus CSS custom properties (e.g., `--ifm-background-color`, `--ifm-color-primary`) for all color values rather than hardcoded hex values.
3. THE Copy_Button SHALL apply theme-consistent hover and focus styles derived from the active Theme's CSS custom properties.
4. THE Code_Block background SHALL be visually distinct from the surrounding QuickStart_Section background in both light and dark Themes.
