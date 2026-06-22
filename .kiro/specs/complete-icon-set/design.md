# Design Document: Complete Icon Set

## Overview

This design covers the creation and integration of a complete icon set for the Soroban Cookbook documentation site (Docusaurus 3.x). The goal is to provide properly sized and formatted icons for all major browser, mobile, and platform contexts — browser tabs, iOS/Android homescreens, PWA installs, and Windows tiles — while keeping total asset weight under 100KB.

The existing site already has a `favicon.ico` at `documentation/static/img/favicon.ico` and an SVG logo at `documentation/static/img/logo.svg`. The new icon set will be derived from the existing `logo.svg` as the master source, generating all required raster sizes and updating configuration files.

## Architecture

The solution has three layers:

1. **Asset layer** — static files placed in `documentation/static/img/icons/` (PNG/ICO/SVG) and `documentation/static/` (manifest + browserconfig)
2. **Configuration layer** — `site.webmanifest`, `browserconfig.xml`, and updates to `docusaurus.config.ts`
3. **Generation layer** — a documented shell script (`scripts/generate-icons.sh`) that produces all assets from the SVG source using Sharp/Inkscape/ImageMagick

```
documentation/
├── static/
│   ├── img/
│   │   ├── favicon.ico              # multi-size ICO (16, 32, 48)
│   │   ├── favicon.svg              # SVG favicon (modern browsers)
│   │   └── icons/
│   │       ├── favicon-16x16.png
│   │       ├── favicon-32x32.png
│   │       ├── favicon-48x48.png
│   │       ├── apple-touch-icon.png         # 180x180
│   │       ├── apple-touch-icon-152x152.png
│   │       ├── apple-touch-icon-167x167.png
│   │       ├── apple-touch-icon-120x120.png
│   │       ├── android-chrome-192x192.png
│   │       └── android-chrome-512x512.png
│   ├── site.webmanifest
│   └── browserconfig.xml
├── docusaurus.config.ts             # updated headTags + favicon field
scripts/
└── generate-icons.sh                # asset generation script
```

## Components and Interfaces

### Asset Generation Script (`scripts/generate-icons.sh`)

A bash script that takes the master SVG and produces all required outputs. It uses `sharp-cli` (Node.js) for PNG rasterization and `icotool` (from `icoutils`) or ImageMagick for ICO assembly.

**Inputs:** `documentation/static/img/logo.svg`  
**Outputs:** All files listed in the architecture above

**Interface:**
```bash
# Usage
./scripts/generate-icons.sh [--source path/to/logo.svg] [--out documentation/static]
```

### `site.webmanifest`

Standard W3C Web App Manifest JSON file. Served from the site root (`/site.webmanifest`).

**Key fields:** `name`, `short_name`, `icons`, `theme_color`, `background_color`, `display`

### `browserconfig.xml`

Microsoft-specific XML for Windows tile configuration. Served from the site root (`/browserconfig.xml`).

**Key fields:** `square150x150logo`, `TileColor`

### Docusaurus Config Updates (`docusaurus.config.ts`)

- `favicon` field updated to `img/favicon.svg` (with ICO fallback via `<link>` tag)
- New `headTags` entries for apple-touch-icon links, manifest link, and browserconfig meta

## Data Models

### `site.webmanifest` Schema

```json
{
  "name": "Soroban Cookbook",
  "short_name": "Soroban",
  "description": "A comprehensive guide to building smart contracts on Stellar with Soroban",
  "start_url": "/",
  "display": "standalone",
  "theme_color": "#1e1e2e",
  "background_color": "#1e1e2e",
  "icons": [
    {
      "src": "/img/icons/android-chrome-192x192.png",
      "sizes": "192x192",
      "type": "image/png",
      "purpose": "any maskable"
    },
    {
      "src": "/img/icons/android-chrome-512x512.png",
      "sizes": "512x512",
      "type": "image/png",
      "purpose": "any maskable"
    }
  ]
}
```

### `browserconfig.xml` Schema

```xml
<?xml version="1.0" encoding="utf-8"?>
<browserconfig>
  <msapplication>
    <tile>
      <square150x150logo src="/img/icons/android-chrome-192x192.png"/>
      <TileColor>#1e1e2e</TileColor>
    </tile>
  </msapplication>
</browserconfig>
```

### Icon Size Matrix

| File | Size | Format | Use Case |
|------|------|--------|----------|
| `favicon.ico` | 16+32+48px embedded | ICO | Browser tab fallback |
| `favicon.svg` | vector | SVG | Modern browser tab |
| `favicon-16x16.png` | 16×16 | PNG | Legacy browser tab |
| `favicon-32x32.png` | 32×32 | PNG | Standard browser tab |
| `favicon-48x48.png` | 48×48 | PNG | Windows taskbar |
| `apple-touch-icon.png` | 180×180 | PNG | iOS default touch icon |
| `apple-touch-icon-152x152.png` | 152×152 | PNG | iPad (non-retina) |
| `apple-touch-icon-167x167.png` | 167×167 | PNG | iPad Pro |
| `apple-touch-icon-120x120.png` | 120×120 | PNG | iPhone (non-retina) |
| `android-chrome-192x192.png` | 192×192 | PNG | Android homescreen / PWA |
| `android-chrome-512x512.png` | 512×512 | PNG | PWA splash / install |

### Docusaurus `headTags` Additions

```typescript
// favicon.ico fallback (browsers that don't support SVG favicons)
{ tagName: 'link', attributes: { rel: 'icon', type: 'image/x-icon', href: '/img/favicon.ico' } },
// SVG favicon
{ tagName: 'link', attributes: { rel: 'icon', type: 'image/svg+xml', href: '/img/favicon.svg' } },
// PNG favicons
{ tagName: 'link', attributes: { rel: 'icon', type: 'image/png', sizes: '32x32', href: '/img/icons/favicon-32x32.png' } },
{ tagName: 'link', attributes: { rel: 'icon', type: 'image/png', sizes: '16x16', href: '/img/icons/favicon-16x16.png' } },
// Apple touch icons
{ tagName: 'link', attributes: { rel: 'apple-touch-icon', sizes: '180x180', href: '/img/icons/apple-touch-icon.png' } },
{ tagName: 'link', attributes: { rel: 'apple-touch-icon', sizes: '167x167', href: '/img/icons/apple-touch-icon-167x167.png' } },
{ tagName: 'link', attributes: { rel: 'apple-touch-icon', sizes: '152x152', href: '/img/icons/apple-touch-icon-152x152.png' } },
{ tagName: 'link', attributes: { rel: 'apple-touch-icon', sizes: '120x120', href: '/img/icons/apple-touch-icon-120x120.png' } },
// Manifest
{ tagName: 'link', attributes: { rel: 'manifest', href: '/site.webmanifest' } },
// Windows tile
{ tagName: 'meta', attributes: { name: 'msapplication-config', content: '/browserconfig.xml' } },
```

## Correctness Properties

*A property is a characteristic or behavior that should hold true across all valid executions of a system — essentially, a formal statement about what the system should do. Properties serve as the bridge between human-readable specifications and machine-verifiable correctness guarantees.*

### Property 1: All required icon files exist with correct dimensions

*For any* icon file declared in the icon size matrix (favicon PNGs, app icons, touch icons), the file must exist on disk at its declared path and its actual pixel dimensions must exactly match the dimensions encoded in its filename and in any referencing configuration entry.

**Validates: Requirements 1.2, 2.1, 2.2**

### Property 2: All required head link tags are present in config

*For any* required icon link relationship (`rel="icon"` for each favicon size/format, `rel="apple-touch-icon"` for each touch icon size, `rel="manifest"`), the `docusaurus.config.ts` headTags array must contain a corresponding entry with the correct `rel`, `sizes`, `type`, and `href` attributes.

**Validates: Requirements 3.1, 3.3, 8.2, 8.3**

### Property 3: Manifest icons array covers all required app icon sizes

*For any* required app icon size (192×192, 512×512), the `site.webmanifest` icons array must contain an entry where `sizes` matches the required size, `src` points to an existing file, and `type` is `"image/png"`.

**Validates: Requirements 3.2, 7.3**

### Property 4: Manifest file is valid JSON (round-trip)

*For any* read of the `site.webmanifest` file, parsing it as JSON must succeed without throwing, and re-serializing the parsed object must produce a string that parses back to an equivalent object.

**Validates: Requirements 7.6**

### Property 5: Total icon asset size is under 100KB

*For any* complete set of generated icon files (all PNGs, ICO, SVG), the sum of their file sizes in bytes must be strictly less than 102,400 bytes (100KB).

**Validates: Requirements 6.3**

### Property 6: SVG favicon contains no editor metadata

*For any* read of the `favicon.svg` file, the file content must not contain XML comments (`<!-- -->`), Inkscape namespace declarations (`xmlns:inkscape`), Sodipodi namespace declarations (`xmlns:sodipodi`), or `<metadata>` elements.

**Validates: Requirements 6.4**

### Property 7: Generation script produces all expected output files

*For any* run of `generate-icons.sh` against a valid source SVG, every file listed in the icon size matrix must exist at its expected output path after the script completes successfully.

**Validates: Requirements 5.5**

## Error Handling

- **Missing source SVG**: The generation script exits with a non-zero code and a descriptive error message if `logo.svg` is not found at the expected path.
- **Tool not installed**: The script checks for required tools (`sharp-cli`, `convert`/`icotool`) at startup and prints installation instructions if missing.
- **Malformed manifest**: The manifest is validated as JSON during the build verification step; a syntax error will cause the browser to silently ignore it, so the CI check catches this before deployment.
- **Oversized assets**: The script reports individual and total file sizes after generation; if the total exceeds 100KB the script prints a warning and exits with a non-zero code.
- **Docusaurus build failure**: If `headTags` entries reference files that don't exist in `static/`, Docusaurus will still build (it doesn't validate static asset references), but the browser will receive 404s. The generation script must be run before `docusaurus build`.

## Testing Strategy

### Unit Tests

Unit tests verify specific examples, edge cases, and integration points. They should be focused and not duplicate what property tests cover.

Specific examples to verify:
- `site.webmanifest` `name` field equals `"Soroban Cookbook"` (Requirement 7.1)
- `site.webmanifest` `short_name` field is non-empty and ≤12 characters (Requirement 7.2)
- `site.webmanifest` `theme_color` equals `"#1e1e2e"` (Requirement 7.4)
- `site.webmanifest` `background_color` field exists and is a valid CSS color (Requirement 7.5)
- `browserconfig.xml` is well-formed XML with a `TileColor` element (Requirement 3.4)
- `docusaurus.config.ts` `favicon` field is set to `"img/favicon.svg"` (Requirement 8.1)
- `docusaurus.config.ts` headTags contains a `rel="manifest"` entry (Requirement 8.3)
- `docusaurus.config.ts` headTags contains a `name="theme-color"` meta entry (Requirement 8.4)
- All icon files referenced in manifest `icons[].src` exist on disk (Requirement 3.2)

### Property-Based Tests

Property-based testing validates universal properties across all icon assets and configuration values. The chosen library is **fast-check** (TypeScript), which integrates naturally with the existing TypeScript/Node.js toolchain.

Each property test runs a minimum of 100 iterations.

**Tag format:** `Feature: complete-icon-set, Property {number}: {property_text}`

| Property | Test Description | Tag |
|----------|-----------------|-----|
| Property 1 | For any required icon file, it exists and has correct dimensions | `Feature: complete-icon-set, Property 1: All required icon files exist with correct dimensions` |
| Property 2 | For any required link relationship, headTags contains a matching entry | `Feature: complete-icon-set, Property 2: All required head link tags are present in config` |
| Property 3 | For any required app icon size, manifest icons array has a matching entry | `Feature: complete-icon-set, Property 3: Manifest icons array covers all required app icon sizes` |
| Property 4 | Manifest JSON round-trips without data loss | `Feature: complete-icon-set, Property 4: Manifest file is valid JSON (round-trip)` |
| Property 5 | Sum of all icon file sizes < 100KB | `Feature: complete-icon-set, Property 5: Total icon asset size is under 100KB` |
| Property 6 | SVG favicon contains no editor metadata | `Feature: complete-icon-set, Property 6: SVG favicon contains no editor metadata` |
| Property 7 | Generation script produces all expected output files | `Feature: complete-icon-set, Property 7: Generation script produces all expected output files` |

Each correctness property is implemented by a single property-based test. Properties 1–6 can be run without executing the generation script (they test the committed static assets). Property 7 requires running the script in a CI environment with the required tools installed.

