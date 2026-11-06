---
id: make-the-readme-docs-workflow-skill-and-diagrams-match-main-with-checks-that-fail-when-they-drift-flow-18
state: draft
type: documentation
base_commit: 615b587b33a1ef3a569f9557e2782a6be3df2f0f
---

# Make the README, docs, workflow skill and diagrams match main, with checks that fail when they drift (FLOW-18)

## Intent

Make the README, docs, workflow skill and diagrams match main, with checks that fail when they drift (FLOW-18)

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- tests/docs.rs fails on the base and passes after: the stage diagram draws only the helper's stages and moves (skip from design to plan, the test to accept gate naming the unlinked failure); every inline Mermaid block equals its docs/diagrams source and each SVG carries its source's labels; the README helper table names every chough command and listed subcommand and its verify list every fledge.toml verify step; every documented chough flag and relative link resolves; docs/cli.md's .gitattributes block equals the repository's; no page states a current SpecSync, Fledge or HI version other than CI's pin; the review gate is closed with --close-gate, never cleared; the 615b587b SKILL.md, checkpoint.md and lifecycle.md read as known-outdated and update. The other audit findings in context.md are corrected, lines that open pull requests change are untouched, no helper behavior or requirement text changes, and fledge lanes run verify passes locally.

## No-spec Rationale

No requirement text changes: the README, docs, workflow skill, diagrams and their SVGs are corrected to what main already does, the earlier skill files' digests are registered so installed copies update (REQ-cli-019's existing rule), and tests/docs.rs checks the docs against the helper, fledge.toml, .gitattributes and the CI pins. No helper behavior changes.
