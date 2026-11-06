---
change: advance-through-several-stages-on-one-native-read-with-stage-advance-to-and-read-native-status-once-on-the-move-into
artifact: design
---

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
