---
change: make-the-readme-docs-workflow-skill-and-diagrams-match-main-with-checks-that-fail-when-they-drift-flow-18
artifact: context
---

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
