# Lesson bundle: advance-through-several-stages-on-one-native-read-with-stage-advance-to-and-read-native-status-once-on-the-move-into

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Advance through several stages on one native read with stage advance --to, and read native status once on the move into done (DRIVE-9, DRIVE-2)
- **Kind**: Feature
- **Specs**: cli
- **Paths**: src/stage.rs, src/library.rs, src/evidence.rs, tests/advance_to.rs, docs/cli.md, docs/FRD.md, CHANGELOG.md
- **Acceptance**: Under a per-change session, 'chough stage advance --to accept' from plan on an approved change runs specsync change status once, records three advance events with their gate labels in applied.steps and ends at accept, and a single-checkpoint project appends the same three transitions. From intent, '--to plan' makes no native read. A refused step (blocking bug, draft state, a target not ahead, a side or unknown target, an open human gate) refuses the whole batch with at most one read, nothing recorded and the session file and history byte-identical. On an archived change with an acceptance manifest and the librarian on, 'stage advance' from accept to done and '--to done' from test each run specsync change status exactly once and report librarian_trim; before this change, accept to done ran it twice. A plain 'stage advance' is unchanged. The new-behavior tests fail against main first and pass after, and fledge lanes run verify passes.

## Evidence

- Verification commit: `516dcfc2ce72ab70f1138914d2d182ebb1560227`
- Base commit: `260ecc949ea01214f4bec43b30ea4226ce8fae59`
- Verified by: `specsync check --spec cli`

## From the change's context.md

# Context

## Intent and decisions

DRIVE-9 and DRIVE-2 are in `hi/drive.md`, quoted from Leif's t811u brief and confirmed by him on 2026-11-03.
- DRIVE-9: "Several `stage advance` steps in a row share one native status read."
- DRIVE-2: each command makes "at most one native status read".

Leif's decisions, relayed by the orc (t18):
- 2026-11-05 21:51 MST, "Split" for PR #69:
  - a small new PR from main carries `stage advance --to` (DRIVE-9), as one SpecSync change with failing-first tests under per-change sessions;
  - the archive cache goes to kimi-perf32's #32 change;
  - #69 closes only after both replacements are open.
- 2026-11-05 21:52 MST, Q2 "Yes": the librarian at done reuses the gate's native read, so advance into done is one read (DRIVE-2).

This change takes REQ-cli-052. REQ-cli-050 (ch-d7) and REQ-cli-051 (KNOW-29) are claimed, and #32 took REQ-cli-053.

## What led here

PR #69 (corvid-agent, base 8298e5a) combined two things: a persistent cache of SpecSync's authenticated-archive verdict, and `stage advance --to`. It went stale and conflicting. On main 73a7da56 with SpecSync 6.2.0, measured on fresh scratch clones of Chough (122 archives) and Raven (148 archives), with Raven runs interleaved A/B:
- Test to done on Raven:
  - main took two commands, 55 to 66 s and three native reads;
  - #69's batch with its cache emptied took one command, 33 s and two reads.
- Main's move into done reads twice:
  - once for the gate;
  - once again for the same change, when the trim reads the latest archive's acceptance manifest.
- Load average was 42 to 117 on 20 cores, so the absolute times are inflated. The read counts are exact.

## Prior art and what is ruled out

- #69's `advance_to` and its two batch tests are the starting point. They are re-asserted here under per-change sessions and event history.
- Out of this change:
  - any cache that outlives a command (#32 owns the next/status cache);
  - any change to `next`, `status` or `timing`;
  - a cached gate for stage moves.
  Every native read stays live; a read is shared only within one command.
- Main is affected by chough#141: KNOW-29's roll-up hid the legacy history anchor, so new sessions cannot write events. Implementation waits for #141 to merge, then merges main.

## From the change's design.md

# Design

## Approach

- **Command:** `StageCommand::Advance` gains `--to <stage>`. Without it, the command runs today's `move_forward`, unchanged.
- **Shared checks:** the checks for leaving test (blocking bugs, unlinked failure) and the destination gate move into one helper. `move_forward` and the batch both use it, so a step in a batch is checked exactly as a single advance is.
- **The batch (`advance_to`):**
  - It applies `ensure_active`, `ensure_main` and `ensure_no_open_human_gate` first, and refuses a target that is not a main stage after the current one.
  - It walks `ORDER` from the current stage to the target. The native state is read once, on the first step whose destination has a prerequisite, and every later step's gate uses `gate_for_state` with that state.
  - Every step is checked before the first transition is written. A refusal names the step, as `advance <from> -> <to> refused (<label>): <detail>; nothing was recorded`.
  - It then appends one `advance` transition per step and saves once.
- **A failed append mid-batch:** in event history, the events this command published are removed with `history::discard_published`, and the command fails with nothing saved. In a legacy single log, lines already appended stay, as with any append failure today.
- **One read into done:**
  - `move_forward` and `advance_to` keep the `specsync change status` response they read for the session's change.
  - `run` passes it to `library::trim_at_done`, then to the post-archive summary, then to `evidence::native_manifests`.
  - There, `manifest(root, id)` uses that response when `id` is the session's change, and reads live for any other ID.
  - The validity probes around removals (`evidence::validity`) are not given the response and stay live.
  - The response is a value inside one process and is never written.

## Risks

- **A stale manifest:** the reused response is from the same command, a moment earlier, before any write of this command touches `.specsync/`. Chough never writes `.specsync/`, so the manifest cannot change in between except through an outside writer. An outside writer could change it between two live reads too.
- **Retention safety:** removals still need the live validity probes and the open-change refusal, so reuse cannot widen what is removed.
- **Merge drift:** `stage.rs` and `evidence.rs` are busy files, so this branch merges origin/main before its PR.

## Constraints consulted

- REQ-cli-008, REQ-cli-030, REQ-cli-041, REQ-cli-033.
- REQ-history-008: no orphan events.
- DRIVE-9, DRIVE-2.
- #32 (REQ-cli-053) owns any persistent cache.

## From the change's testing.md

# Testing

## tests/advance_to.rs (new, unix)

- `a_batched_advance_reads_native_status_once_in_a_session`: under `session new`, plan to accept on one read, three events, steps reported.
- `a_batched_advance_in_a_single_checkpoint_project`: the same, in `.chough/transitions.jsonl`.
- `a_batch_with_no_native_prerequisite_reads_nothing`: intent to plan, zero reads, two events.
- `a_refused_step_refuses_the_whole_batch_with_nothing_recorded`: a blocking bug, draft state, a target that is not ahead, a side or unknown target, and an open human gate. Each leaves the session file and history byte-identical, with at most one read.
- `advance_into_done_reads_native_status_once`: from accept, and with `--to done` from test, on an archived change with an acceptance manifest. Each makes exactly one read and reports `librarian_trim`.
- `a_plain_advance_is_unchanged`: one transition, no `steps`.

All but the last fail first against main.

## Requirement evidence

| Requirement | Evidence |
|---|---|
| REQ-cli-052 | tests/advance_to.rs, all six tests |

## Lanes

`fledge lanes run verify` passes.

## Where these lessons go

- `specs/cli/context.md`
