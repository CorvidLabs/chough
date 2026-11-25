---
id: 2026-11-25-pr-448-449-records-repair
title: PR 448/449 records repair: mechanical re-approval annotations and PR 449 re-keyed as fix-441
type: journal
tags: [chough/journal, chough/intent]
date: 2026-11-25
time: 2026-11-25T06:45:14Z
kind: intent
links:
  - knowledge/journal/2026-11-25-leif-rules-on-pr-448-449-blockers.md
  - knowledge/journal/2026-11-25-leif-confirms-the-six-fix-provenance-and-authorizes-merge.md
actor:
  - agent: pi
    terminal: t14z
---
# PR 448/449 records repair: mechanical re-approval annotations and PR 449 re-keyed as fix-441

Implements Leif's 2026-11-25 rulings on the PR 448/449 blockers (see the linked
ruling journal). Done in the `pr448-followup` checkout and a throwaway clone;
nothing was pushed. Performed as a directed records repair, not a native
change, because Leif's ruling directed the exact annotations and the task
forbade opening a second lifecycle.

## Part A: the three wordless definition re-approvals marked mechanical

Additively annotated (only a `note` and a `relayed_by` entry added; no other
field changed) in two archive records:

- `.specsync/archive/changes/2026-11-24-fix-436/approvals.json`, definition
  entry 1795529693
- `.specsync/archive/changes/2026-11-24-fix-439/approvals.json`, definition
  entries 1795494331 and 1795530080

Each `relayed_by` carries Leif's verbatim selection and citation:

- words: `Mark as mechanical (Recommended)`
- where: `pi chat t14z interview question 2, 2026-11-25T05:56:31Z, "Ratify with
  your words, or authorize marking them mechanical?"`

Each `note` states the per-entry explanation and the relay disclaimer (the
original definition approval stands; recorded as mechanical per Leif's
ruling; relayed by the t14z orchestrator; not authenticated), matched to
what each send-back actually changed:

- fix-436 1795529693: re-scoping only — `affected_paths` narrowed from the
  `src` and `tests` directories to the six files the branch carries
  (send-back 1795529639: "Finalize refuses the scope as wider than the
  work ... the fix content is unchanged").
- fix-439 1795494331: re-scoping only — the REQ-cli-039 spec delta the
  second review required (send-back 1795494235), `no_spec_change` now
  false, `affected_paths` unchanged.
- fix-439 1795530080: re-scoping only — `affected_paths` re-declared as
  the exact files the branch carries instead of the `src` and `tests`
  directories (send-back 1795530031: "Finalize refuses the scope as
  misdeclared").

These entries are mechanical relayed annotations, not new authenticated
approvals.

## Part C checks (this checkout, after the annotations)

- `chough doctor`: no findings, exit 0.
- `specsync doctor`: no problems found, exit 0.
- `cargo test --locked --bin chough
  product_digests_match_every_archived_approval`: 1 passed, exit 0 (the
  archive's delta digests still match every archived approval).
- `specsync change status` for fix-435, fix-436, fix-437, fix-438, fix-439,
  fix-440: all `State: archived`, all readable by `specsync` and `chough
  change status`, exit 0.
- `chough change ready` has no live change to run against here (all six are
  archived; `.specsync/changes/` is empty); the doctor and digest-guard checks
  above are the applicable readiness checks for archived records.

## Part B: PR 449 re-keyed as fix-441 and merged with PR 448

In a throwaway clone (`<redacted:path>pr449-rekey`), branch
`fix/441-grouped-gates-rekeyed` (from `fix/438-atomic-grouped-gates` tip
c469d409), two commits:

- 62785722: re-key the change `fix-438` as `fix-441` — folder
  `.specsync/changes/fix-441/`, session folder and session key
  `2026-11-25-fix-441`, changelog fragment
  `changelog.d/2026-11-25-fix-441.md`, id fields and self-references updated.
- a055ab9c: merge `pr448-followup` @ 743af0d2, keeping both fixes.

Conflict resolution (src/checkpoint.rs, src/gates.rs; tests/gates.rs merged
cleanly): #449's grouped-decision atomic publish is kept — one in-process
preparation pass, one checkpoint write — and #448's gate-record all-or-none is
folded into it. Every named gate's relay (values, open check, identity,
decision size) is checked before preparation, and preparation completes all
gates' native-step eligibility, decision size, cumulative session-record bound
and Timeline cap before the native step runs and before the single publish, so
a refusal on any named gate saves nothing. #448's per-gate dry-run rehearsal
and subprocess closes (dry_relay_refusal, close_command,
apply_gate_close_next_action) are subsumed by that pass and removed; the
single-gate `checkpoint --close-gate --dry-run` rehearsal stays.
`finish_relay` takes the display name so grouped and single-gate closes render
REQ-records-008 display-name decisions identically, and identity refusals
surface in the grouped reply.

Verification in the throwaway: `cargo build` exit 0; `cargo clippy
--all-targets` exit 0; `cargo test --locked --test gates` 32 passed, `--test
checkpoint` 5 passed, plus humans 23, relay 7, native_check 10,
direct_writes 12, advance_to 5, change 37, history 8, next 42, sessions 52 and
131 binary unit tests, all exit 0.

`chough change ready fix-441` on the merged branch: one finding while PR 448
is not yet in main — PR 448's own changelog fragment
`changelog.d/2026-11-23-fix-438.md` (a different, archived fix-438) counts as
an undeclared changed file until it is in the default branch; with the
throwaway's origin/main pointed at PR 448's head (the post-merge state) the
same command reports "Nothing is missing", exit 0.

Native consequence of the re-key, left for a human gate: the recorded
definition approval no longer validates (specsync doctor: "scope approval
digest does not match its approved projection"; `specsync change show`:
approval_valid false), and `chough change check fix-441` refuses with the same
digest error until a fresh fix-441 definition approval exists. Leif must
approve the re-keyed change; then check, review and ship can run.
