# Decisions

This is an index of past decisions. They distinguish human choices from
implementation choices, and they are context for later work, not authenticated
approval records. No new D-numbers are added. A decision now lives where it applies:
- human intent in [HI](../hi/);
- a change's scope, approval and lessons in its native SpecSync change and the
  module's lesson source;
- a session's decisions in its `session.md`, under Decisions and why, with a relayed
  gate decision on its Timeline.

The last column says where each decision below lives now.

| ID | Decision | Basis and consequence | Where it lives now |
|---|---|---|---|
| D-001 | Name the project Chough, pronounced chuff. | Explicit human choice; `CorvidLabs/chough`, created private and now public. | [README](../README.md) |
| D-002 | Make v1 current-agent-first. | Explicit human choice; skills guide the active agent. Rune coordination is deferred. | [AGENTS.md](../AGENTS.md) and the [workflow skill](../skills/chough-workflow/SKILL.md) |
| D-003 | Keep most behavior in skills and Markdown; use a small Rust helper. | Accepted human direction; no model client, hosted service, or custom runtime. | [AGENTS.md](../AGENTS.md) |
| D-004 | Preserve native ownership. | Accepted architecture: HI owns intent, SpecSync owns changes/contracts/lessons, Fledge owns task execution. | [AGENTS.md](../AGENTS.md) |
| D-005 | Bootstrap with native structural checks; do not invent approval (superseded by D-008 for new work). | Implementation choice within authorized first development. Native SDD is initialized with its default disabled policy; no approved/archived lifecycle is claimed. | Superseded by D-008 |
| D-006 | Store a focused session and revision/environment-bound observations. | First-slice implementation choice; durable criteria remain unchanged as achievement changes. | REQ-cli-017 in [the CLI requirements](../specs/cli/requirements.md) |
| D-007 | Expose adapters through bounded inputs and native tool output. | Implementation choice; adding an adapter requires compatibility tests and a clear ownership boundary. | [Integrations](integrations.md) |
| D-008 | Adopt the full SpecSync workflow-v2 lifecycle; meaningful files need a native change. | Human direction (2026-10-31 brief) and approval of `evolve-chough-into-a-resumable-stage-workflow`. Supersedes D-005 for new work. | `.specsync/sdd.json` |
| D-009 | Lifecycle writers and CI use SpecSync 6.1.1. | Implementation choice before the change: PATH already resolved 6.1.1; the v6.1.1 Action tag has assets and needs an explicit version. Fledge pins unchanged. Since PR #18 the first CI job, the archive gate, pins the SpecSync Action at main commit 106aaa4 because v6.1.1 ignores the archive gate inputs; other jobs stay on 6.1.1. | Superseded by D-021 |
| D-010 | Qualify APS 1.1.0 and defer it; `.chough/session.json` stays the only checkpoint. | Approved with the change. Observed lost concurrent updates without error, unknown fields accepted, an implicit global root and no Windows binary. Reconsider when those change. | Replaced in 0.6.0: the active session's `session.md` front matter is the only checkpoint (REQ-cli-007, REQ-records-004); APS stays deferred |
| D-011 | Test definitions use TOML profile `chough-testing/1`, not 3md. | Approved with the change. 3md attributes are single-line strings, its tools ignore unknown fields, and SpecSync reads module `.3md` as lessons. | The [cases template](../skills/chough-workflow/templates/cases.toml) |
| D-012 | `cases run` and `validate` execute bound commands only on explicit invocation, without a shell. | Approved with the change; replaces the bootstrap invariant that the helper never runs tests. Vacuous or environment failures are not-run. | REQ-cli-011 in [the CLI requirements](../specs/cli/requirements.md) |
| D-013 | Monitor findings are captured locally by default; propose-change only prints a command. | Approved default; whether monitors may start investigations stays unsettled. | [Integrations](integrations.md) |
| D-014 | One canonical stage list in the helper, documented in `references/stages.md`. | Implementation choice; a test keeps them in step. | The [stage reference](../skills/chough-workflow/references/stages.md) |
| D-015 | Crate-internal modules use `pub(super)`. | Implementation choice: the binary has no library target, and the contract describes commands rather than Rust symbols. | The source under `src/` |
| D-016 | Observation reuse after metadata-only commits (the deferred proposal, now in Git history) stays unapproved and deferred, not rejected. Superseded by D-018. | Human framing in PR #1; the lifecycle work came first. It is not implemented and needs its own decision. | Superseded by D-018 |
| D-017 | Evidence retention defaults (2026-10-31): protect the newest 3 automated runs and the newest failure per case and environment; only sets older than 14 days are eligible; warn above 50 MiB; `--apply` needs a run consent `retention` or the standing preference `retention auto` (default `off`); with `auto`, `cases run` and `validate run` prune after recording; removals are logged in `.chough/pruned.jsonl`. | Human choice by 0xLeif with the scope approval of `prepare-a-first-release-candidate-with-reference-aware-evidence-retention-current-guidance-and-an-installable-package` (REQ-cli-018, LIFE-8). Preview is the default; anything the checkpoint, a bug, an active native change or the most recent archived one references stays; native archives, ignore rules and Git history are untouched. | Retired in 0.6.0 with the retention ledger (REQ-history-001): a finished session's proof stays in its story |
| D-018 | Freshness compares a working content identity, not the revision (2026-11-01): committing only `.chough/` records, an empty commit, SpecSync lifecycle bookkeeping (state, approvals, verification, review, finalization, accepted-state and `lesson-bundle.md` records, and the front-matter `state:` of `change.md`) and archiving a change keep observations current; the revision is still recorded; a record without a content identity keeps the revision-bound comparison; evidence schema 2 gains an optional SHA-256 content digest (REQ-cli-017 modified); `verified` needs a linked failure and a pass observed strictly after the latest one (REQ-cli-012). Supersedes D-016 in this narrower form, with no ancestry check and no separate reuse label. | Human choice by 0xLeif with the scope approval of `keep-evidence-current-across-commits-and-specsync-lifecycle-bookkeeping-and-verify-bugs-only-by-a-pass-observed-after`, taking the recommended design answers, including BUG-0014 in scope and docs/NEXT_CHANGE.md marked addressed for content-identical commits. An observation reads current at any revision with the same content identity. | REQ-cli-004 and REQ-cli-012 in [the CLI requirements](../specs/cli/requirements.md) |
| D-019 | Pinned toolchain (2026-11-01): HI 0.8.0 (crate `human-intent`), SpecSync 6.1.1 and Fledge 1.8.0, stated in README.md, AGENTS.md and CI and kept in agreement by a package test. CI installs Fledge with the Fledge Action at its v1.8.0 GitHub release (replacing v1.7.2), HI with cargo and SpecSync with its v6.1.1 Action; local installs use one README command, `cargo install --locked human-intent@0.8.0 fledge@1.8.0 specsync@6.1.1` (Rust 1.89 or later). Chough declares `rust-version = "1.88"`, checked by the `MSRV / 1.88.0` CI job. | Human choice by 0xLeif with the scope approval of `prepare-the-0-1-0-release-pinned-toolchain-with-one-install-command-a-checked-minimum-rust-version-accurate-skill`: first "Pin 1.8.0, install via cargo", then, once Fledge v1.8.0 was tagged, the Fledge Action at v1.8.0 in CI for speed. No `fledge run tools` task: Fledge must already be installed to run it. Recorded exercises keep the versions they observed. | Superseded by D-021 for SpecSync; the install command is in the [README](../README.md#install-the-toolchain) |
| D-020 | Chough's size bounds apply only to its own records under `.chough/` and to what it displays (2026-11-04). Any gate, validation or decision reads native sources at full size: HI family exports are parsed whole to validate criteria, and SpecSync's own files (archived `finalization.json` and `change.md`, `.specsync/sdd.json`, the change records `timing` reads) are read within SpecSync's 4 MiB change-artifact bound, never the 64 KiB record bound. Display cuts stay and say they are display only. Chough never causes an agent to edit HI intent or criteria, specs or SpecSync files to fit a Chough limit; a native source over a Chough bound is a Chough bug to report (LIFE-15, REQ-cli-042, REQ-reporting-004). | Human instruction by Leif (0xLeif), the jackdaw product owner, on 2026-11-04 in issue #100, after an agent in CorvidLabs/jackdaw PR #41 rewrote his HI intent to fit the 16 KiB HI export bound: "Chough should only reduce and do all that for its own files in .chough and not the specs, not the human intent." Extends REQ-cli-034's spec-file exemption to HI and SpecSync files; every Chough record keeps its bound under REQ-cli-032. | REQ-cli-042 in [the CLI requirements](../specs/cli/requirements.md) |
| D-021 | SpecSync pin moves to 6.2.0 (2026-11-05), superseding the SpecSync 6.1.1 and 6.1.2 pins of D-009 and D-019 for current work. Every SpecSync Action step in CI, including the Archive gate, uses the v6.2.0 release commit `40dcf33a6b0f0b55dcb7f17e8e98be9d55fa2c56` with `version: '6.2.0'` passed explicitly, because the v6.2.0 tag's action.yml keeps a 6.1.2 default. README.md, AGENTS.md, the archive scripts, docs/ci.md and docs/integrations.md name 6.2.0, kept in agreement by the package test; HI 0.8.0, Fledge 1.8.0 and the Rust 1.89 install note are unchanged. | Requested by session t18 as a pin-only SpecSync change (`move-the-pinned-specsync-toolchain-to-6-2-0-every-specsync-action-step-at-the-v6-2-0-release-commit-with-an-explicit`); human scope approval and review are recorded natively, not here. Recorded exercises and lessons keep the versions they observed. | The CI workflow and the [README](../README.md#install-the-toolchain) |
| D-022 | Supported platforms are macOS and Linux; Windows through WSL 2 with Ubuntu (2026-11-05, FLOW-17). Hosted CI drops the native Windows product job; the Ubuntu jobs cover WSL 2 as Linux. | Human choice: Leif chose "Mac + Linux; Windows via WSL" in the orc tab on 2026-11-05 at 8:17 PM MST, relayed by orc, matching SpecSync, which publishes no Windows binary from 6.0. Requirements are unchanged: REQ-cli-046 keeps short attachment paths so any Git client can check a repository out. Windows-specific code stays, unbuilt and untested; WSL 1, checkouts on a Windows drive and Windows-side Git are not covered (docs/integrations.md, Platforms). | [Platforms](integrations.md#platforms); since 0.6.0 record names of at most 80 bytes (REQ-records-003) keep paths short |

## Chough 0.6.0 decisions

These are not numbered. Each one is recorded, with its source, in the context and plan
of the native change `chough-0-6-0-human-first-records-…` and in the HI criteria named
here. The words are Leif's, relayed by the coordinator on 2026-11-10 unless a date
says otherwise; the relay is a claim, not authentication.

- **One change.** The human-first redesign ships as one change, Chough 0.6.0, dogfooded
  on this repository before any other repository moves (roadmap M1 to M3).
- **Record format.** One Markdown file with YAML front matter per record, named by date
  and slug, with old IDs as aliases (DRIVE-25 to DRIVE-28).
- **Keep every finished session.** Every story stays in `.chough/archive/sessions/`,
  never trimmed: "Keep all" (KNOW-34).
- **Proof splits.** Logs go to CI and stay local; a record keeps the exact command, a
  fingerprint and the key failing lines: "Split them" (KNOW-35, LIFE-19).
- **Approval and review as the human.** A human's decision at a Chough gate drives the
  SpecSync approval or review in that human's name, from configured identity
  (FLOW-23).
- **Approval on authority given in advance.** It names the agent, `<agent>-<terminal>`,
  and quotes the human: "The agent's (Recommended)" (FLOW-24).
- **Leaked values.** Clean from 0.6.0 on, with no Git history rewrite: "No, clean from
  0.6.0 on (Recommended)" (LIFE-26).
- **A gate holds only its own stage** (#237). This was the planned default for the
  scope approval, which can still change it (REQ-cli-030).
- **Release.** No tag until the dogfood is complete and every bug it finds is fixed:
  "yes.. but we dog food and fix all bugs before tagging and releasing".

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
commits. See the [lifecycle](lifecycle.md) and [roadmap](roadmap-1.0.md)
for the agreed direction and unresolved implementation/policy choices.

Changing product intent requires updating HI through its supported flow, preserving
permanent IDs. Changing an implementation choice requires a recorded reason and
updated requirements or contracts where behavior changes.
