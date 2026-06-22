# Implementation Plan: Complete Icon Set

## Overview

Implement a complete icon set for the Soroban Cookbook Docusaurus site by creating an asset generation script, producing all required PNG/ICO/SVG files from the existing `logo.svg` source, adding manifest and browserconfig files, updating `docusaurus.config.ts` with the required `headTags`, and verifying correctness with fast-check property-based tests.

## Tasks

- [x] 1. Create the asset generation script
  - [x] 1.1 Write `scripts/generate-icons.sh` that accepts `--source` and `--out` flags
    - Check for required tools (`sharp-cli`, `convert` or `icotool`) at startup and print install instructions if missing
    - Exit with non-zero code and descriptive message if source SVG is not found
    - _Requirements: 5.1, 5.2, 5.3_
  - [x] 1.2 Add PNG rasterization steps for all sizes in the icon size matrix
    - Produce `favicon-16x16.png`, `favicon-32x32.png`, `favicon-48x48.png` in `documentation/static/img/icons/`
    - Produce `apple-touch-icon.png` (180×180), `apple-touch-icon-167x167.png`, `apple-touch-icon-152x152.png`, `apple-touch-icon-120x120.png`
    - Produce `android-chrome-192x192.png`, `android-chrome-512x512.png`
    - _Requirements: 1.2, 2.1, 2.2_
  - [x] 1.3 Add ICO assembly step to produce `documentation/static/img/favicon.ico` embedding 16, 32, and 48px sizes
    - _Requirements: 1.1_
  - [x] 1.4 Add SVG copy/clean step to produce `documentation/static/img/favicon.svg` stripped of editor metadata (comments, Inkscape/Sodipodi namespaces, `<metadata>` elements)
    - _Requirements: 1.3, 6.4_
  - [x] 1.5 Add post-generation size check: sum all output file sizes and exit non-zero with a warning if total ≥ 100KB
    - _Requirements: 6.3_

- [-] 2. Generate and commit all icon asset files
  - [x] 2.1 Run `scripts/generate-icons.sh` to produce all files listed in the icon size matrix under `documentation/static/img/icons/` and `documentation/static/img/`
    - Verify each file exists at its expected path after the script completes
    - _Requirements: 1.1, 1.2, 2.1, 2.2, 5.5_
  - [ ]* 2.2 Write property test for Property 7: Generation script produces all expected output files
    - **Property 7: Generation script produces all expected output files**
    - **Validates: Requirements 5.5**
    - Tag: `Feature: complete-icon-set, Property 7: Generation script produces all expected output files`

- [ ] 3. Create `site.webmanifest`
  - [~] 3.1 Write `documentation/static/site.webmanifest` with the schema defined in the design
    - Include `name`, `short_name`, `description`, `start_url`, `display`, `theme_color`, `background_color`, and `icons` array
    - `theme_color` and `background_color` must be `"#1e1e2e"`
    - `icons` array must reference `android-chrome-192x192.png` and `android-chrome-512x512.png` with `"purpose": "any maskable"`
    - _Requirements: 7.1, 7.2, 7.3, 7.4, 7.5, 7.6_
  - [ ]* 3.2 Write property test for Property 3: Manifest icons array covers all required app icon sizes
    - **Property 3: Manifest icons array covers all required app icon sizes**
    - **Validates: Requirements 3.2, 7.3**
    - Tag: `Feature: complete-icon-set, Property 3: Manifest icons array covers all required app icon sizes`
  - [ ]* 3.3 Write property test for Property 4: Manifest file is valid JSON (round-trip)
    - **Property 4: Manifest file is valid JSON (round-trip)**
    - **Validates: Requirements 7.6**
    - Tag: `Feature: complete-icon-set, Property 4: Manifest file is valid JSON (round-trip)`
  - [ ]* 3.4 Write unit tests for `site.webmanifest` specific field values
    - Assert `name === "Soroban Cookbook"` (Req 7.1)
    - Assert `short_name` is non-empty and ≤12 characters (Req 7.2)
    - Assert `theme_color === "#1e1e2e"` (Req 7.4)
    - Assert `background_color` exists and is a valid CSS color (Req 7.5)
    - Assert all `icons[].src` paths exist on disk (Req 3.2)
    - _Requirements: 7.1, 7.2, 7.4, 7.5_

- [ ] 4. Create `browserconfig.xml`
  - [~] 4.1 Write `documentation/static/browserconfig.xml` with the schema defined in the design
    - Reference `android-chrome-192x192.png` as the `square150x150logo`
    - Set `TileColor` to `#1e1e2e`
    - _Requirements: 3.4_
  - [ ]* 4.2 Write unit test asserting `browserconfig.xml` is well-formed XML with a `TileColor` element
    - _Requirements: 3.4_

- [~] 5. Checkpoint — Ensure all asset files and manifest/config files are in place
  - Ensure all tests pass, ask the user if questions arise.

- [ ] 6. Update `docusaurus.config.ts` with icon `headTags`
  - [~] 6.1 Update the `favicon` field to `"img/favicon.svg"`
    - _Requirements: 8.1_
  - [~] 6.2 Add `headTags` entries for favicon ICO fallback, SVG favicon, and PNG favicons (16×16, 32×32)
    - _Requirements: 3.1, 8.2_
  - [~] 6.3 Add `headTags` entries for all four apple-touch-icon sizes (180×180, 167×167, 152×152, 120×120)
    - _Requirements: 3.3, 8.2_
  - [~] 6.4 Add `headTags` entry for `rel="manifest"` pointing to `/site.webmanifest`
    - _Requirements: 3.2, 8.3_
  - [~] 6.5 Add `headTags` entry for `name="msapplication-config"` pointing to `/browserconfig.xml`
    - _Requirements: 3.4_
  - [ ]* 6.6 Write property test for Property 2: All required head link tags are present in config
    - **Property 2: All required head link tags are present in config**
    - **Validates: Requirements 3.1, 3.3, 8.2, 8.3**
    - Tag: `Feature: complete-icon-set, Property 2: All required head link tags are present in config`
  - [ ]* 6.7 Write unit tests for `docusaurus.config.ts` specific entries
    - Assert `favicon` field equals `"img/favicon.svg"` (Req 8.1)
    - Assert headTags contains a `rel="manifest"` entry (Req 8.3)
    - Assert headTags contains a `name="theme-color"` meta entry (Req 8.4)
    - _Requirements: 8.1, 8.3, 8.4_

- [ ] 7. Set up fast-check test infrastructure and write remaining property tests
  - [~] 7.1 Install `fast-check` as a dev dependency in `documentation/` and configure a test runner (Vitest or Jest) if not already present
    - _Requirements: (testing infrastructure)_
  - [~] 7.2 Write property test for Property 1: All required icon files exist with correct dimensions
    - **Property 1: All required icon files exist with correct dimensions**
    - **Validates: Requirements 1.2, 2.1, 2.2**
    - Tag: `Feature: complete-icon-set, Property 1: All required icon files exist with correct dimensions`
  - [~] 7.3 Write property test for Property 5: Total icon asset size is under 100KB
    - **Property 5: Total icon asset size is under 100KB**
    - **Validates: Requirements 6.3**
    - Tag: `Feature: complete-icon-set, Property 5: Total icon asset size is under 100KB`
  - [~] 7.4 Write property test for Property 6: SVG favicon contains no editor metadata
    - **Property 6: SVG favicon contains no editor metadata**
    - **Validates: Requirements 6.4**
    - Tag: `Feature: complete-icon-set, Property 6: SVG favicon contains no editor metadata`

- [ ] 8. Document the asset generation process
  - [~] 8.1 Add a `## Icon Generation` section to `documentation/README.md` (or create `scripts/README.md`) documenting:
    - Source file location (`documentation/static/img/logo.svg`)
    - Required tools and install commands
    - The `generate-icons.sh` command with its flags
    - Optimization steps applied
    - How to update icons when the brand changes
    - _Requirements: 5.1, 5.2, 5.3, 5.4, 5.5_

- [~] 9. Final checkpoint — Ensure all tests pass
  - Ensure all tests pass, ask the user if questions arise.

## Notes

- Tasks marked with `*` are optional and can be skipped for faster MVP
- Each task references specific requirements for traceability
- Property tests use **fast-check** with a minimum of 100 iterations each
- Properties 1–6 test committed static assets and can run in CI without the generation script
- Property 7 requires the generation script and its tool dependencies to be installed
- The `generate-icons.sh` script must be run before `docusaurus build` to ensure all referenced static assets exist
