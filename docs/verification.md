# Documentation verification

On 2026-10-31 the four Mermaid sources were rendered with **Mermaid 11.12.0** in
headless Chromium (Playwright 1.64.1, Chromium headless shell 1248) under Bun 1.4,
using `docs/diagrams/render.mjs`. The PNG previews were opened and inspected.

| Source | Committed render | Preview | Inspection |
|---|---|---|---|
| [ownership.mmd](diagrams/ownership.mmd) | [ownership.svg](diagrams/ownership.svg) | 1552 × 660 | Labels readable; every arrow reaches its node; the dotted edge marks the optional librarian |
| [stages.mmd](diagrams/stages.mmd) | [stages.svg](diagrams/stages.svg) | 1079 × 1072 | Main path top to bottom with labelled gates; back edges dotted; release only from done; side stages and the checkpoint tied to "any stage"; no clipping or overlap |
| [lifecycle.mmd](diagrams/lifecycle.mmd) | [lifecycle.svg](diagrams/lifecycle.svg) | 483 × 1356 | Human review before `review`, `ship` and archive; changes requested loop back; release dotted and conditional |
| [context.mmd](diagrams/context.mmd) | [context.svg](diagrams/context.svg) | 1180 × 720 | All inputs reach the packet; limits feed it explicitly |

Two problems were found and fixed before this record. The first `stages` render was
1552 × 179, with crowded labels near `done` and an unconnected checkpoint node, so it
was redrawn top to bottom. In `lifecycle`, a label wrapped inside `--commit` and was
reworded.

## Earlier record (PR #1)

Kept as written; it describes the three diagrams of that documentation correction,
which the four diagrams above replace.

> For the 2026-10-31 documentation correction, all three Mermaid sources were
> rendered with **Mermaid 11.17.2**
> under local headless Chromium build **1248**, using Playwright and Bun **1.4.0**.
> The rendered PNG previews were opened and visually inspected: labels were readable,
> connectors reached the intended nodes, and no clipping or overlap was observed.
> 
> | Source | Committed render | Preview dimensions |
> |---|---|---|
> | [ownership.mmd](diagrams/ownership.mmd) | [ownership.svg](diagrams/ownership.svg) | 934 × 628 |
> | [context.mmd](diagrams/context.mmd) | [context.svg](diagrams/context.svg) | 1144 × 624 |
> | [lifecycle.mmd](diagrams/lifecycle.mmd) | [lifecycle.svg](diagrams/lifecycle.svg) | 581 × 2032 |
> 
> Temporary PNG review artifacts were saved beneath `/tmp/chough-docs-review-20261031/`.
> The lifecycle render now labels conditional visual design, acceptance before
> native closeout and optional release as agreed design. The ownership label refers
> to actual project command results rather than implying a delivery integration.
> Those machine-local previews are not required to read the committed documentation.
> The SVG files are the reviewable rendered artifacts; the Markdown pages also carry
> their Mermaid source blocks for GitHub rendering and editing.

## Reproduce the render

The helper installs nothing; point it at local modules:

```sh
MERMAID_SCRIPT=/path/to/node_modules/mermaid/dist/mermaid.min.js \
PLAYWRIGHT_MODULE=/path/to/node_modules/playwright/index.mjs \
CHOUGH_DIAGRAM_SCREENSHOTS=/tmp/chough-diagram-review \
bun docs/diagrams/render.mjs
```

It fails if Mermaid cannot render a source or a diagram has invalid bounds, writes
SVGs beside the sources and optionally saves PNGs. Rendering alone is not visual
verification; inspect the images after changing a diagram.

This record covers documentation only. Product tests, structural checks, exercises,
CI and native lifecycle state have their own records; see
[the CLI testing companion](../specs/cli/testing.md).
