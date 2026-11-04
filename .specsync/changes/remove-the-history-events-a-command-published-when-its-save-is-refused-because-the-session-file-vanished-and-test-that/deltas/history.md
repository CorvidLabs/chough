## MODIFIED

### REQUIREMENT REQ-history-002

Immutable transition history. The helper SHALL offer read-only `history adopt` and explicit `history adopt --apply`, preserving the byte-identical legacy transitions log and schema-1 migration backup. Adopted and newly initialized projects SHALL write immutable versioned event files with independent random 96-bit cycle/event identities, parent links, native-change context and the existing transition payload. The single schema-2 checkpoint, or in sessions mode each session (REQ-history-006), SHALL hold an optional history cursor as part of its atomic current-cycle merge group. Event publication SHALL precede atomic checkpoint/cursor publication; failures SHALL preserve prior checkpoint state and mark unselected events explicitly. When a command's save is refused because the checkpoint it resolved was archived, removed or adopted into sessions mode while it ran, the helper SHALL first remove the history events that command published, when their bytes are unchanged, so it leaves no orphan. No second mutable checkpoint or inferred native lifecycle state SHALL be introduced; in sessions mode only the active session is written. Trace: LIFE-2, LIFE-3, LIFE-6, LIFE-7, FLOW-2, FLOW-13, DRIVE-3.

Acceptance Criteria
- Adoption preview changes no file; explicit apply and repeat preserve original log and migration bytes, validate a digest/length/count anchor and do not duplicate events. Unexpected anchor changes refuse new writes.
- Legacy checkpoints without a cursor and explicit schema-1 migration remain supported. Old readers of adopted checkpoints fail visibly; they never silently discard the cursor.
- Independent real Git branches writing at the same controlled timestamp retain both immutable event paths and hashes. Divergent cursor edits refuse as a current-cycle group, while compatible merges preserve the selected lineage.
- New event/cycle IDs use one random candidate with atomic no-replace refusal and no retry or fallback. Failed random, event or checkpoint writes preserve existing bytes; orphan events cannot count as completed transitions.
- Parent links and the selected checkpoint cursor determine lineage. Missing parents, conflicting identities and unrelated branch reset events cannot manufacture order, change current guidance or bypass native gates.
- Each session's history cursor names its own cycle, and a stage move in one session leaves every other session's cycle unchanged.
- A `stage advance` whose session file is moved away during its native read refuses with nothing saved and leaves no new event file, and `session use` started during a `stage advance` waits for it.
