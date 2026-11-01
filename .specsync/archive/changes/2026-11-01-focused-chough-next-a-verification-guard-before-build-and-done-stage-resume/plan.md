---
change: focused-chough-next-a-verification-guard-before-build-and-done-stage-resume
artifact: plan
---

# Plan

## Approach

Three small increments in the CLI, each with its tests, then guidance, docs and a
bundle refresh. One agent builds. If split, ownership is: src/main.rs and the next
view tests (increment 1); src/stage.rs reset and done rules (increment 2); src/stage.rs
guard and src/records.rs bounded reader (increment 3, after increment 1 lands the
one-read refactor); docs and bundle last.

`chough timing` history is too thin to use: build has 3 samples (1 second to 4
minutes, backfilled), plan and test fewer than 3. Estimates are by relative size.

| Increment | Relative size | Main uncertainty |
|---|---|---|
| Shared assessment, one native read, compact next view with cap, text and `--json` | medium | moving existing `next --json` test reads to `status` without losing coverage |
| Done-stage new-cycle action and reset next_action replacement | small | wording |
| Approved-delta guard and 4 MiB native record reader | small to medium | SpecSync framing drift in a later release |
| Host wording, checkpoint guidance, KNOWN_PREVIOUS, host copies | small | rebase against PR #22's bundle edits |
| Cold-start record, integrations row, docs/cli.md, architecture, README, examples | small | none |

## Out of scope

Changing the host map or what `skill status` counts; a Gemini CLI or `.gemini/skills`
claim; running `specsync change check` from a gate; reading any other native record;
changing the full status report's fields; new storage, record kinds, integrations or a
model client; modifying REQ-cli-008, 016 or 019 (PR #22 does); the ROADMAP; native
fixes (spec-sync #823 is upstream).

## Steps

1. Refactor: `native_state_from(response)` and a gate function that takes a
   `NativeState`; `assess()` in main.rs; `status` built from it with unchanged
   fields; a counting `specsync` stub test shows one read for `status` and `next` at
   plan, build and accept.
2. Next view: JSON and text renderers, bounds and the drop loop; move tests that read
   full-report fields via `next --json` to `status`; cap fixture test.
3. Done rule and reset replacement, with tests from a done fixture (archived stub with
   `terminal_evidence.validity: exact`, a non-authenticating archive, no native
   change, SDD disabled).
4. Guard: bounded reader, framing, comparison, both labels; vectors from the 7
   archived approvals in this repository copied into a test fixture; refusal tests
   with byte-identical session and transitions.
5. SKILL.md and references/checkpoint.md; KNOWN_PREVIOUS; `chough skill update` for
   `.agents/` and `.claude/`.
6. docs/cli.md, docs/architecture.md, docs/integrations.md, docs/exercises/cold-start.md,
   README.md; examples use `next --json`.
7. Measure `chough next` (text and `--json`) and `chough status` sizes and times on
   this repository for the pull request description.

Post-verification steps (plain text, not tasks): after the scoped `change check`
materializes REQ-cli-021 and REQ-cli-022, add TC-cli-0120 to TC-cli-0124 to
.chough/cases/cli.cases.toml and record their runs; add a note to BUG-0002 that
Chough guards it, keeping it open; push, CI on Ubuntu, macOS and Windows; re-run
`change check`.

## Risks

- Output change: `chough next` without `--json` becomes text. Scripts piping it into
  `jq` break; the two examples and the tests in this repository are updated, and
  docs/cli.md states it as a change from earlier builds. The pull request description
  calls it out.
- Framing drift: a later SpecSync could change `specsync.approved-delta.v1`; the guard
  then refuses every approved change as `native-delta-changed`, which fails closed but
  blocks work. The refusal names the framing it used. Revisit when CI's SpecSync pin
  moves past 6.1.1, where spec-sync #823 also makes status report the edit.
- Reading SpecSync's approval ledger is a read of a native internal file, not a
  documented interface. It is read-only and copied nowhere; the timing change set the
  precedent of reading native record files read-only.
- Rebase with PR #22: both edit SKILL.md, references/checkpoint.md, KNOWN_PREVIOUS,
  README.md and docs/integrations.md. Whichever merges second rebases and adds the
  other's previous digests.
- Truncation hides detail: every list states its total and the view names `chough
  status`; nothing is dropped silently.

## Constraints consulted

specs/cli/context.md lessons (BUG-0002 lesson; plain-text gates and post-verification
steps; new requirement IDs validate only after materialization; pub(super); disposable
Git identity; Claude Code reads `.claude/skills`), Invariants (native state read never
copied, bounded inputs, unknown refuses gated moves, no recorded approval), AGENTS.md
(no new integration, model client or storage; `.chough/` not an affected path), the
research measurements of `change check`, and open PR #22.

## Open questions, each with a recommendation

1. `--json` on next. Options: text by default with `--json` for the compact JSON, or
   drop the flag and always print compact JSON. Recommended: text by default with
   `--json`, as `chough context` already does; the view is read by agents and people,
   and the honest flag costs two example edits.
2. `--json` on status, also ignored today. Recommended: keep it accepted and say in
   `--help` that status always prints JSON; removing it breaks existing callers for no
   gain.
3. Reset and `native_change`. Recommended: keep the archived id after reset; the view
   names it closed and asks for the new one, and the build gate refuses until it is
   recorded. Clearing it would hide which change the cycle followed.
4. Cap. Recommended: 16384 bytes per form (the existing context bound), 10 entries
   per list, 512 bytes per string. 8 KiB also fits this repository but leaves little
   room for long gate texts.
5. Approval without `approved_delta_digests` on a change with deltas. SpecSync treats
   absence as no claim and passes it. Recommended: refuse as unverifiable, since the
   decision asks Chough to fail closed; SpecSync 6.1.1 always records digests, so only
   approvals from older SpecSync versions are affected and none is active here.
6. Guard mechanism. Recommended: the digest comparison. Running `change check` is
   rejected (it mutates canonical specs and native records, refuses whenever tasks are
   unchecked, so always at plan to build, and runs verification commands); waiting for
   a SpecSync release leaves 0.1.0 exposed.
7. gemini and `.agents/skills`. Recommended: no change to the host map or `skill
   status`; fix the wording only.
