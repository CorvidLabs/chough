---
change: evolve-chough-into-a-resumable-stage-workflow
artifact: research
---

# Research

Observed on 2026-10-31 on macOS arm64 unless stated. Each item separates what was
run or read from what upstream documentation claims.

## Tool versions and writer alignment

- `PATH` resolves `~/.cargo/bin` first: `hi 0.5.0`, `fledge 1.8.0` (unreleased local
  build), `specsync 6.1.1`. Homebrew has `hi 0.8.0`, `fledge 1.7.2`, `specsync 6.1.0`.
- CI installed `human-intent 0.8.0`, Fledge binary `v1.7.2` through wrapper commit
  `b648ea2a622221d4f2ffbc7ba7aa829e6cf13a51`, and SpecSync `6.1.0`.
- Decision executed before this change: lifecycle writer and CI both use SpecSync
  6.1.1 (commit c582a10). `CorvidLabs/spec-sync@v6.1.1` contains `action.yml` with
  Linux/macOS assets and a 6.1.0 omitted-input default, so the version is explicit.
- Fledge still has no `@v1` or `v1.8.0` ref and no `action.yml` at `v1.7.2`; the
  reviewed wrapper pin stays. Local verification uses a scratch `PATH` that selects
  `hi 0.8.0`, `fledge 1.7.2` and `specsync 6.1.1` to match CI.

## SpecSync 6.1.1 mechanics rehearsed in a disposable clone

- `change adopt` only flips `enabled` and writes `.specsync/adoption-report.json`.
- The interview is acceptance_criteria, public_contract, architecture_risk and,
  because module lessons exist, constraints. Yes/yes selects context, requirements,
  plan, tasks, testing, docs, design and research.
- Affected paths outside `source_dirs` (docs/, skills/, .github/) are accepted.
- Rust `pub(crate)` items in a new src module are reported as undocumented exports;
  `pub(super)` and private items are not.
- Editing the canonical spec `files:` frontmatter directly was accepted by `change
  check`; deltas have no frontmatter block type.
- After approval, edits to tasks, testing, plan, design, context or change.md did not
  stale approval. A delta edit was refused by `change check` with a reapproval
  instruction, while `change status` still reported `approval_valid: true`.
- A disposable copy of this exact definition was approved, checked, reviewed and
  finalized with actors labelled `SIMULATED-dry-run-not-a-human`. The delta
  materialized REQ-cli-007 to REQ-cli-017 and bumped `cli.spec.md` to version 2;
  `change check` first refused while `tasks.md` had unchecked boxes; `summary.state`
  read draft, approved, verifying and archived. No simulated record exists in this
  repository.
- In one rehearsal run, a failing `change check` (exit 1, undocumented export) left
  `verification.json` with `passed: true` and status advanced to verifying. This is
  an unreproduced observation for the local backlog, not an established defect.

## APS 1.1.0 qualification (upstream main 85deceac, release v1.1.0)

All commands used an explicit scratch `--state-dir`; `~/.aps` and UserDefaults were
unchanged before and after.

| Property | Observation |
|---|---|
| Cross-process persistence | Values round-trip exactly across processes, including unicode and nested arrays. |
| Root selection | `--state-dir`, then `APS_HOME`, then `~/.aps`; no upward project discovery. A relative root from a subdirectory created a second root. |
| Schema | Int, String, Bool and object only; no array type. Unknown fields and any schema_version are accepted; no migration hook; no size limit. |
| Corruption | Corrupt value files fail with exit 65 and are left alone; a deleted value file returns the initial value with exit 0, indistinguishable from a fresh checkpoint. |
| Concurrency | 50 parallel read-modify-write updates to one key lost 49 updates, all exiting 0; blind sets were last-writer-wins. Distinct keys were correct. |
| Format | One-line JSON with unstable key order; noisy Git diffs. |
| Platforms | Releases for macOS and Linux only; values only as argv, which caps Windows below the 64 KiB record limit. |

Conclusion used by the design: defer APS. Replacing `session.json` would weaken
current guarantees, and a mirror would be a second writable checkpoint. Reconsider
when APS ships Windows binaries, stdin/file value input, compare-and-set, closed
object shapes with arrays, and refusal of an implicit global root.

## Test-definition format

- 3md v2.2.1 (`CorvidLabs/3md` c19963b; crate `threemd` 2.2.1): planes need a unique
  numeric `z`; attributes and frontmatter are single-line strings; bodies are opaque
  Markdown; the optional `3md-id` editing identity is preserved but not enforced by
  plain parsers; unknown keys are kept, never rejected.
- agent.3md: typed inputs and a `tool` command template are relevant; loaders MUST
  ignore unknown attributes; routing is ranked natural-language matching.
- Magpie (`CorvidLabs/magpie` ac93507, no releases, Bun): generic mode runs every
  `tool=` skill with `/bin/sh -c`, judges exit codes only, records guidance-only or
  unfilled skills as `ok: true, exitCode: null`, and cannot run one skill alone. Its
  README moved flat step lists to `.steps.toml`. Not installed here.
- SpecSync reads `specs/<module>/<module>.3md` as context plus lesson planes; a test
  file there would be read as lessons.
- `cargo test <filter>` that matches nothing exits 0 with "running 0 tests"; plain
  filters also match longer sibling names; `--exact` needs the full test path.

Conclusion used by the design: TOML profile `chough-testing/1` outside `specs/`, with
closed-world validation. 3md remains a possible later read-only rendering.

## Hosts and shared skills

- The CorvidLabs skills installer maps hosts to project directories: openai
  `.agents/skills`, claude `.claude/skills`, codex `.codex/skills`, cursor, gemini and
  grok `.<host>/skills`. Chough reuses this mapping instead of inventing one.
- This Claude Code session did not list `chough-workflow` although
  `.agents/skills/chough-workflow/` exists, so that install path is not discovered by
  this host. Installed agent CLIs: claude 2.1.316, codex 0.153.4, grok 1.0.70, kimi
  0.41.0, gemini 0.42.0, cursor-agent; any can host a genuine fresh-session cold start.
- Product-authoritative skills exist for spec-sync, fledge-workflows, three-md,
  trust, augur, attest and rune; Chough links to them instead of copying manuals.

## Design system and rendering

- `CorvidLabs/design-system` (private, c698ed4) publishes `assets/tokens.css` and
  asks consumers to import tokens rather than hardcode colors. A pinned copy can
  style the visual sample; Figma is not required.
- Mermaid 11 and Playwright with Chromium headless shell 1248 exist in local
  CorvidLabs checkouts for the existing `docs/diagrams/render.mjs`.

## Planning sources deferred

`docs/NEXT_CHANGE.md` proposed reusing observations after metadata-only commits. The
human did not approve it and redirected to this lifecycle. It is retained as history
and marked unapproved and deferred, not rejected; its reuse behavior is out of scope.

The untracked brief `docs/CLAUDE_HANDOFF.md` was present at session start and was no
longer in the working tree at 11:45:36, before any write by this change. Its text was
supplied in the conversation; HI wording below cites its sections.
