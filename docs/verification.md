# Documentation verification

On 2026-10-31, all three Mermaid sources were rendered with **Mermaid 11.17.2**
under local headless Chromium build **1248**, using Playwright and Bun **1.4.0**.
The rendered PNG previews were opened and visually inspected: labels were readable,
connectors reached the intended nodes, and no clipping or overlap was observed.

| Source | Committed render | Preview dimensions |
|---|---|---|
| [ownership.mmd](diagrams/ownership.mmd) | [ownership.svg](diagrams/ownership.svg) | 934 × 628 |
| [context.mmd](diagrams/context.mmd) | [context.svg](diagrams/context.svg) | 1144 × 624 |
| [lifecycle.mmd](diagrams/lifecycle.mmd) | [lifecycle.svg](diagrams/lifecycle.svg) | 572 × 1140 |

Temporary PNG review artifacts were saved beneath `/tmp/chough-diagram-review/`.
Those machine-local previews are not required to read the committed documentation.
The SVG files are the reviewable rendered artifacts; the Markdown pages also carry
their Mermaid source blocks for GitHub rendering and editing.

## Reproduce the render

Use locally installed Mermaid, Playwright, and Chromium. The documentation helper
does not fetch or install dependencies. Point it to the actual local modules:

```sh
MERMAID_SCRIPT=/path/to/node_modules/mermaid/dist/mermaid.min.js \
PLAYWRIGHT_MODULE=/path/to/node_modules/playwright/index.mjs \
CHROMIUM_EXECUTABLE=/path/to/chromium \
CHOUGH_DIAGRAM_SCREENSHOTS=/tmp/chough-diagram-review \
bun docs/diagrams/render.mjs
```

`CHROMIUM_EXECUTABLE` is optional when Playwright's matching browser is installed.
`CHOUGH_DIAGRAM_SCREENSHOTS` is optional. The script fails if Mermaid cannot render
a source or the resulting diagram has invalid bounds. It writes SVGs beside the
`.mmd` files and optionally saves PNGs for visual inspection. Rendering alone is
not visual verification; inspect the results after changing a diagram.

This record covers documentation rendering only. Rust tests, structural checks,
product acceptance exercises, CI, release, and delivery require their own actual
evidence. See [the CLI testing companion](../specs/cli/testing.md) for the owning
product verification record.
