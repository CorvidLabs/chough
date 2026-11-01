# Decisions

These entries distinguish human choices from implementation choices. They are
context for later work, not authenticated approval records.

| ID | Decision | Basis and consequence |
|---|---|---|
| D-001 | Name the project Chough, pronounced chuff. | Explicit human choice; `CorvidLabs/chough`, created private and now public. |
| D-002 | Make v1 current-agent-first. | Explicit human choice; skills guide the active agent. Rune coordination is deferred. |
| D-003 | Keep most behavior in skills and Markdown; use a small Rust helper. | Accepted human direction; no model client, hosted service, or custom runtime. |
| D-004 | Preserve native ownership. | Accepted architecture: HI owns intent, SpecSync owns changes/contracts/lessons, Fledge owns task execution. |
| D-005 | Bootstrap with native structural checks; do not invent approval (superseded by D-008 for new work). | Implementation choice within authorized first development. Native SDD is initialized with its default disabled policy; no approved/archived lifecycle is claimed. |
| D-006 | Store a focused session and revision/environment-bound observations. | First-slice implementation choice; durable criteria remain unchanged as achievement changes. |
| D-007 | Expose adapters through bounded inputs and native tool output. | Implementation choice; adding an adapter requires compatibility tests and a clear ownership boundary. |
| D-008 | Adopt the full SpecSync workflow-v2 lifecycle; meaningful files need a native change. | Human direction (2026-10-31 brief) and approval of `evolve-chough-into-a-resumable-stage-workflow`. Supersedes D-005 for new work. |
| D-009 | Lifecycle writers and CI use SpecSync 6.1.1. | Implementation choice before the change: PATH already resolved 6.1.1; the v6.1.1 Action tag has assets and needs an explicit version. Fledge pins unchanged. Since PR #18 the first CI job, the archive gate, pins the SpecSync Action at main commit 106aaa4 because v6.1.1 ignores the archive gate inputs; other jobs stay on 6.1.1. |
| D-010 | Qualify APS 1.1.0 and defer it; `.chough/session.json` stays the only checkpoint. | Approved with the change. Observed lost concurrent updates without error, unknown fields accepted, an implicit global root and no Windows binary. Reconsider when those change. |
| D-011 | Test definitions use TOML profile `chough-testing/1`, not 3md. | Approved with the change. 3md attributes are single-line strings, its tools ignore unknown fields, and SpecSync reads module `.3md` as lessons. |
| D-012 | `cases run` and `validate` execute bound commands only on explicit invocation, without a shell. | Approved with the change; replaces the bootstrap invariant that the helper never runs tests. Vacuous or environment failures are not-run. |
| D-013 | Monitor findings are captured locally by default; propose-change only prints a command. | Approved default; whether monitors may start investigations stays unsettled. |
| D-014 | One canonical stage list in the helper, documented in `references/stages.md`. | Implementation choice; a test keeps them in step. |
| D-015 | Crate-internal modules use `pub(super)`. | Implementation choice: the binary has no library target, and the contract describes commands rather than Rust symbols. |
| D-016 | Observation reuse after metadata-only commits (`docs/NEXT_CHANGE.md`) stays unapproved and deferred, not rejected. Superseded by D-018. | Human framing in PR #1; the lifecycle work came first. It is not implemented and needs its own decision. |
| D-017 | Evidence retention defaults (2026-10-31): protect the newest 3 automated runs and the newest failure per case and environment; only sets older than 14 days are eligible; warn above 50 MiB; `--apply` needs a run consent `retention` or the standing preference `retention auto` (default `off`); with `auto`, `cases run` and `validate run` prune after recording; removals are logged in `.chough/pruned.jsonl`. | Human choice by 0xLeif with the scope approval of `prepare-a-first-release-candidate-with-reference-aware-evidence-retention-current-guidance-and-an-installable-package` (REQ-cli-018, LIFE-8). Preview is the default; anything the checkpoint, a bug, an active native change or the most recent archived one references stays; native archives, ignore rules and Git history are untouched. |
| D-018 | Freshness compares a working content identity, not the revision (2026-11-01): committing only `.chough/` records, an empty commit, SpecSync lifecycle bookkeeping (state, approvals, verification, review, finalization, accepted-state and `lesson-bundle.md` records, and the front-matter `state:` of `change.md`) and archiving a change keep observations current; the revision is still recorded; a record without a content identity keeps the revision-bound comparison; evidence schema 2 gains an optional SHA-256 content digest (REQ-cli-017 modified); `verified` needs a linked failure and a pass observed strictly after the latest one (REQ-cli-012). Supersedes D-016 in this narrower form, with no ancestry check and no separate reuse label. | Human choice by 0xLeif with the scope approval of `keep-evidence-current-across-commits-and-specsync-lifecycle-bookkeeping-and-verify-bugs-only-by-a-pass-observed-after`, taking the recommended design answers, including BUG-0014 in scope and docs/NEXT_CHANGE.md marked addressed for content-identical commits. An observation reads current at any revision with the same content identity. |
| D-019 | Pinned toolchain (2026-11-01): HI 0.8.0 (crate `human-intent`), SpecSync 6.1.1 and Fledge 1.8.0, stated in README.md, AGENTS.md and CI and kept in agreement by a package test. CI installs Fledge with the Fledge Action at its v1.8.0 GitHub release (replacing v1.7.2), HI with cargo and SpecSync with its v6.1.1 Action; local installs use one README command, `cargo install --locked human-intent@0.8.0 fledge@1.8.0 specsync@6.1.1` (Rust 1.89 or later). Chough declares `rust-version = "1.88"`, checked by the `MSRV / 1.88.0` CI job. | Human choice by 0xLeif with the scope approval of `prepare-the-0-1-0-release-pinned-toolchain-with-one-install-command-a-checked-minimum-rust-version-accurate-skill`: first "Pin 1.8.0, install via cargo", then, once Fledge v1.8.0 was tagged, the Fledge Action at v1.8.0 in CI for speed. No `fledge run tools` task: Fledge must already be installed to run it. Recorded exercises keep the versions they observed. |

## Agreed direction after the bootstrap interview

These are human product decisions recorded in PR #1 to capture in the next native
scope definition. They were captured as FLOW-1 to FLOW-13 and REQ-cli-007 to
REQ-cli-017 and approved in `evolve-chough-into-a-resumable-stage-workflow`; D-008 to
D-016 record the choices made there. The list is kept as written:

- Natural stage iteration with meaningful whole-stage intent/design decisions and
  one coherent initial technical scope approval; real human implementation review
  at acceptance before routine native closeout.
- Conditional one-source local HTML/CSS design with companion notes and actual
  rendered review; focused functional, copy/localization and design testing by
  relevance; local-first bug evidence/backlog.
- Optional validation/monitoring by explicit project preference, optional release,
  and an optional project-local librarian callable from any stage.
- Mandatory checkpointing/cold resume even with the librarian off. APS storage and
  a 3md testing profile are candidates, not adopted mandatory dependencies.

The metadata-only observation-reuse proposal was deferred in favor of lifecycle work,
not rejected; D-018 later approved it in a narrower form for content-identical
commits. See the [lifecycle](lifecycle.md) and [roadmap](ROADMAP.md)
for the agreed direction and unresolved implementation/policy choices.

Changing product intent requires updating HI through its supported flow, preserving
permanent IDs. Changing an implementation choice requires a recorded reason and
updated requirements or contracts where behavior changes.
