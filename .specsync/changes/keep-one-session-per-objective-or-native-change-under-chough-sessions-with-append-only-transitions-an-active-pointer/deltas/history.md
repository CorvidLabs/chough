## ADDED

### REQUIREMENT REQ-history-006

Sessions per objective. `chough session new <key>` SHALL create `.chough/sessions/<key>.json`, a schema 2 checkpoint with its own history cycle, without replacing a file, and point at it; `session adopt <key>` SHALL move an existing checkpoint with a history cursor there byte for byte; `session use <key>` SHALL point at an open session; `session list` SHALL report each session's key, stage, status and whether it is active or archived. `session new` and `session use` SHALL move every other session at stage done to `.chough/sessions/archive/<key>.json` unchanged, except that a done session with unfolded inbox entries SHALL stay open and be reported as kept open. `chough inbox add --session <key>` SHALL create one entry file under `.chough/sessions/<key>.inbox/` and SHALL NOT edit any session; `inbox fold` SHALL add the active session's unfolded entries to its lists without duplicates and move each into `folded/`. Inbox entries SHALL carry no approval, review, consent, gate or stage field. Two branches that record work in different sessions SHALL merge without conflict. Evidence retention SHALL treat every open and archived session as a checkpoint reference source, and SHALL refuse a preview or apply when any session cannot be read. Trace: DRIVE-3, LIFE-8.

Acceptance Criteria
- Two branches that each advance a different session merge in Git with no conflict and keep both sessions' history.
- Switching away from a done session moves it to the archive with its bytes unchanged.
- Folding an inbox twice adds each entry once, and adding an entry never changes a session file.
- A done session with unfolded inbox entries stays open on a switch.
- An observation referenced only by an inactive or archived session is protected from `evidence prune --apply`, and an unreadable session file refuses it with nothing removed.

## MODIFIED

### REQUIREMENT REQ-history-002

Immutable transition history. The helper SHALL offer read-only `history adopt` and explicit `history adopt --apply`, preserving the byte-identical legacy transitions log and schema-1 migration backup. Adopted and newly initialized projects SHALL write immutable versioned event files with independent random 96-bit cycle/event identities, parent links, native-change context and the existing transition payload. The single schema-2 checkpoint, or in sessions mode each session (REQ-history-006), SHALL hold an optional history cursor as part of its atomic current-cycle merge group. Event publication SHALL precede atomic checkpoint/cursor publication; failures SHALL preserve prior checkpoint state and mark unselected events explicitly. No second mutable checkpoint or inferred native lifecycle state SHALL be introduced; in sessions mode only the active session is written. Trace: LIFE-2, LIFE-3, LIFE-6, LIFE-7, FLOW-2, FLOW-13, DRIVE-3.

Acceptance Criteria
- Adoption preview changes no file; explicit apply and repeat preserve original log and migration bytes, validate a digest/length/count anchor and do not duplicate events. Unexpected anchor changes refuse new writes.
- Legacy checkpoints without a cursor and explicit schema-1 migration remain supported. Old readers of adopted checkpoints fail visibly; they never silently discard the cursor.
- Independent real Git branches writing at the same controlled timestamp retain both immutable event paths and hashes. Divergent cursor edits refuse as a current-cycle group, while compatible merges preserve the selected lineage.
- New event/cycle IDs use one random candidate with atomic no-replace refusal and no retry or fallback. Failed random, event or checkpoint writes preserve existing bytes; orphan events cannot count as completed transitions.
- Parent links and the selected checkpoint cursor determine lineage. Missing parents, conflicting identities and unrelated branch reset events cannot manufacture order, change current guidance or bypass native gates.
- Each session's history cursor names its own cycle, and a stage move in one session leaves every other session's cycle unchanged.
