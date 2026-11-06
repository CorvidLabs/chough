# Lesson bundle: make-the-readme-docs-workflow-skill-and-diagrams-match-main-with-checks-that-fail-when-they-drift-flow-18

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Make the README, docs, workflow skill and diagrams match main, with checks that fail when they drift (FLOW-18)
- **Kind**: Documentation
- **Specs**: cli
- **Paths**: README.md, CHANGELOG.md, INTENT.md, hi/flow.md, docs/, skills/chough-workflow/, .agents/skills/chough-workflow/, src/skill.rs, tests/docs.rs, knowledge/
- **Acceptance**: tests/docs.rs fails on the base and passes after: the stage diagram draws only the helper's stages and moves (skip from design to plan, the test to accept gate naming the unlinked failure); every inline Mermaid block equals its docs/diagrams source and each SVG carries its source's labels; the README helper table names every chough command and listed subcommand and its verify list every fledge.toml verify step; every documented chough flag and relative link resolves; docs/cli.md's .gitattributes block equals the repository's; no page states a current SpecSync, Fledge or HI version other than CI's pin; the review gate is closed with --close-gate, never cleared; the 615b587b SKILL.md, checkpoint.md and lifecycle.md read as known-outdated and update. The other audit findings in context.md are corrected, lines that open pull requests change are untouched, no helper behavior or requirement text changes, and fledge lanes run verify passes locally.

## Evidence

- Verification commit: `24cb22319863f81665164beee3641209ff8ae4b6`
- Base commit: `615b587b33a1ef3a569f9557e2782a6be3df2f0f`
- Verified by: `specsync check --spec cli`

## From the change's context.md

# Context

## The request and the decision

Leif, in the orc tab on 2026-11-06: "make sure the readme and docs and mermaid diagrams are accurate and updated." Orc (hc t18) gave the work to ch-docs-accuracy as a read-only audit first, then one SpecSync change and one docs pull request. The audit found no HI criterion covering docs accuracy, so the agent proposed one. Leif chose "FLOW-18, one PR", relayed by orc: capture FLOW-18 as proposed, put the workflow skill fixes and their `KNOWN_PREVIOUS` digest rows in this same pull request (appended at the end of the list, clear of #151), write the failing checks first, leave `docs/cli.md:25` to #158 and do not touch #69's lines.

## What the audit found

The findings are in `../ch-docs-findings.md` beside this worktree, each with file:line, the claim, what is true and the command that showed it. In short, at base 615b587b:

- Accurate: tool pins, the macOS and Linux (WSL 2) platform policy, the archive-first closeout order, CI job names, all 186 documented `chough` flags, every relative link and anchor, every REQ and criterion ID, preference defaults, stage moves and exit codes in a disposable project. All eight Mermaid sources (four files and four inline blocks) parse and render offline with Mermaid 11.12.0, Playwright 1.64.1 and Chromium 1248, and the committed SVGs are rendered from their sources.
- Wrong: the skill's no-helper resume and checkpoint reference still name only `.chough/session.json` and `.chough/transitions.jsonl` (sessions and event history write elsewhere); docs/lifecycle.md says SpecSync has no verification exclusion (6.2.0 ships `verification_ignored_paths`); the closeout says "clear the gate" where the review gate is closed with `--close-gate` (FLOW-16); stages.mmd draws skip as intent to plan (the helper skips design to plan) and its test to accept gate omits the unlinked failure; ownership.mmd still says "Optional librarian" (on by default, KNOW-5); cli.md says the archive summary keeps the newest 3 runs (it keeps the newest one, KNOW-17); cli.md's `.gitattributes` block lacks four lines the repository declares; three pages state SpecSync 6.1 or 6.1.2 as current.
- Incomplete: the README helper table lacks `history`, `session`, `inbox`, `bug merge` and `skill remove-legacy`, and its verify list lacks the knowledge step; architecture.md's record tables lack sessions and `.chough/local/`; releasing.md is a 0.1.0 script; integrations.md links a private repository and has no `--host codex` row.

## Constraints

- Lines that open pull requests change stay as they are: `docs/cli.md:22` (#69), `docs/cli.md:25` (#158, whose `bug` row omits `bug merge`; reported to orc for #158), `docs/cli.md:1336` (#150), `docs/FRD.md:120` (#158) and `skills/chough-workflow/references/stages.md:16-17` with its copy (#151). #151 inserts at the top of `KNOWN_PREVIOUS`, so this change appends at the end.
- Dated records (DECISIONS, ROADMAP rows, exercise runs) keep the versions and facts of when they were written.
- No helper behavior or requirement text changes. No tag, release or publication.
- Before every push: `specsync change audit --strict` (orc, from spec-sync #900), `fledge lanes run verify`.
- If another pull request edits SKILL.md, checkpoint.md or lifecycle.md first, its main bytes are registered too (the specs/cli lesson on integrated skill predecessors).

## From the change's design.md

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

## Where these lessons go

- `specs/cli/context.md`
