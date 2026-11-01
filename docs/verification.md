# Documentation verification

On 2026-10-31 the four Mermaid sources were rendered with **Mermaid 11.12.0** in
headless Chromium (Playwright 1.64.1, Chromium headless shell 1248) under Bun 1.4,
using `docs/diagrams/render.mjs`. The PNG previews were opened and inspected. On
2026-11-01 the same tooling re-rendered them after the stage gates, the closeout gate
step and the native status edge were corrected; `ownership.svg` came out
byte-identical, and the table shows the new preview sizes.

| Source | Committed render | Preview | Inspection |
|---|---|---|---|
| [ownership.mmd](diagrams/ownership.mmd) | [ownership.svg](diagrams/ownership.svg) | 1552 × 660 | Labels readable; every arrow reaches its node; the dotted edge marks the optional librarian |
| [stages.mmd](diagrams/stages.mmd) | [stages.svg](diagrams/stages.svg) | 1073 × 1204 | Main path top to bottom with labelled gates; back edges dotted; release only from done; side stages and the checkpoint tied to "any stage"; every edge label sits on its own edge with an opaque background, and no edge crosses another edge's label |
| [lifecycle.mmd](diagrams/lifecycle.mmd) | [lifecycle.svg](diagrams/lifecycle.svg) | 483 × 1496 | Human review before `review`, `ship` and archive; changes requested loop back; release dotted and conditional |
| [context.mmd](diagrams/context.mmd) | [context.svg](diagrams/context.svg) | 1180 × 636 | Inputs reach the packet; limits feed it explicitly; native status joins at chough next |

Two problems were found and fixed before this record. The first `stages` render was
1552 × 179, with crowded labels near `done` and an unconnected checkpoint node, so it
was redrawn top to bottom. In `lifecycle`, a label wrapped inside `--commit` and was
reworded.

A later review found that the 1079 × 1142 `stages` render drew the build to test
edge through its own wrapped gate label: with SVG text labels, Mermaid 11.12.0
left-aligns a wrapped edge label beside the edge instead of centring it on the edge.
`stages.mmd` now sets `htmlLabels: true` in its front matter, so its labels are HTML
inside `foreignObject` and are centred, with an opaque background. The other three
diagrams keep the SVG text labels set in `render.mjs`, and their SVGs were
byte-identical after the re-render on 2026-11-01. A viewer that does not render
`foreignObject` shows the `stages` boxes and edges without their text; browsers and
GitHub do render it.

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
