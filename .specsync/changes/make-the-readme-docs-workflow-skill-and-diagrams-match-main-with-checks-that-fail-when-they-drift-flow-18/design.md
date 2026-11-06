---
change: make-the-readme-docs-workflow-skill-and-diagrams-match-main-with-checks-that-fail-when-they-drift-flow-18
artifact: design
---

# Design

No product visuals or interactions change. The diagrams are documentation: each is edited in its `docs/diagrams/*.mmd` source, copied unchanged into any page that inlines it, and re-rendered with `docs/diagrams/render.mjs` using the toolchain docs/verification.md names; the PNG previews are inspected.

`tests/docs.rs` holds the FLOW-18 checks. Each reads the repository's own files with CRLF read as LF and asks the helper for its lists, so nothing is copied by hand:

- FR-docs-1: `chough stage show` gives the stage names and kinds in order; stages.mmd is parsed into nodes and edges; a skip edge must go from a stage to the helper's next main stage, a back edge to an earlier main stage, any other edge between stages must be an advance, a move into build, test, accept or done must name its gate, and the test to accept label must name the blocking bug and the unlinked failure while `src/stage.rs` has both refusals.
- FR-docs-2: every ```mermaid block in a page equals a `docs/diagrams/*.mmd` body (front matter aside), and each SVG's visible text holds every node and edge label of its source.
- FR-docs-3: the README helper table names every command `chough --help` lists, and a cell that lists subcommands lists them all; the README verify list names every step of `fledge.toml`'s verify lane.
- FR-docs-4: every flag given to a `chough` command in a page is in that command's `--help`; every relative link and anchor resolves; the cli.md `.gitattributes` block equals the repository's merge lines.
- FR-docs-5: a present-tense clause that says which SpecSync, Fledge or HI version is used, pinned or followed names the version CI pins; past-tense clauses, the decision log and the roadmap are history.
- FR-docs-6: no page says "clear the gate" for the review gate, and the closeout pages name `--close-gate`; the three skill files main shipped at 615b587b read as known-outdated and update, and an edited copy stays.

The flag and link checks pass on the base; they guard the audit's result.
