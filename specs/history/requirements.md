# History requirements

These requirements moved from the cli module on 2026-11-03 with their wording
unchanged; the old IDs are SpecSync tombstones. Mapping: REQ-cli-018 is
REQ-history-001, REQ-cli-025 is REQ-history-002, REQ-cli-026 is REQ-history-003 and
REQ-cli-027 is REQ-history-004. Observations live in `.chough/evidence/`.

### REQ-history-001

Reference-aware retention. The helper SHALL preview by default which observation sets (a record and the artifacts only it references) are eligible for removal and which are protected and why, and SHALL protect sets referenced by the checkpoint, a bug, an active or the most recent archived native change, non-routine records, the newest runs and newest failure per case and environment, validation failures and boundary passes, and recent sets. It SHALL remove eligible sets only under a run consent or a standing retention preference set by a human, SHALL record each removal in an append-only ledger, and SHALL label referenced observations current, historical, pruned or unavailable, never treating pruned or unavailable ones as current or passed. It SHALL NOT change native archives, ignore rules or Git history. Trace: LIFE-8, LIFE-2, LIFE-7, FLOW-7, FLOW-13.

Acceptance Criteria
- The preview changes no file and lists eligible and protected sets with reasons and reclaimed bytes.
- Apply without consent or the standing preference is refused.
- A pruned observation referenced anywhere is reported pruned, and a missing unlisted one unavailable; neither is current or passed.

### REQ-history-002

Immutable transition history. The helper SHALL offer read-only `history adopt` and explicit `history adopt --apply`, preserving the byte-identical legacy transitions log and schema-1 migration backup. Adopted and newly initialized projects SHALL write immutable versioned event files with independent random 96-bit cycle/event identities, parent links, native-change context and the existing transition payload. The single schema-2 checkpoint, or in sessions mode each session (REQ-history-006), SHALL hold an optional history cursor as part of its atomic current-cycle merge group. Event publication SHALL precede atomic checkpoint/cursor publication; failures SHALL preserve prior checkpoint state and mark unselected events explicitly. No second mutable checkpoint or inferred native lifecycle state SHALL be introduced; in sessions mode only the active session is written. Trace: LIFE-2, LIFE-3, LIFE-6, LIFE-7, FLOW-2, FLOW-13, DRIVE-3.

Acceptance Criteria
- Adoption preview changes no file; explicit apply and repeat preserve original log and migration bytes, validate a digest/length/count anchor and do not duplicate events. Unexpected anchor changes refuse new writes.
- Legacy checkpoints without a cursor and explicit schema-1 migration remain supported. Old readers of adopted checkpoints fail visibly; they never silently discard the cursor.
- Independent real Git branches writing at the same controlled timestamp retain both immutable event paths and hashes. Divergent cursor edits refuse as a current-cycle group, while compatible merges preserve the selected lineage.
- New event/cycle IDs use one random candidate with atomic no-replace refusal and no retry or fallback. Failed random, event or checkpoint writes preserve existing bytes; orphan events cannot count as completed transitions.
- Parent links and the selected checkpoint cursor determine lineage. Missing parents, conflicting identities and unrelated branch reset events cannot manufacture order, change current guidance or bypass native gates.
- Each session's history cursor names its own cycle, and a stage move in one session leaves every other session's cycle unchanged.

### REQ-history-003

Bounded history lifecycle. The helper SHALL bound each event at 65,536 bytes, recent traversal at 128 events and 512 KiB, each new cycle at 4,096 events and 16 MiB, and each timing request at 4,096 events and 16 MiB of transition content. Maintenance enumeration SHALL be limited to 16,384 source files and 64 MiB of metadata, with incomplete scans refusing apply. Ordinary appends at cycle capacity SHALL refuse before checkpoint mutation; a reasoned reset MAY start a new cycle while preserving the previous one. Recent guidance SHALL follow the selected cursor without a directory-wide event scan. Timing SHALL keep independent lineages separate and report omitted, malformed, missing and pruned history as partial or unknown, never zero or verified. Trace: LIFE-3, LIFE-7, LIFE-8, LIFE-10, FLOW-13.

Acceptance Criteria
- Instrumented synthetic fixtures prove each exact byte/event bound, bounded routine reads and byte-identical state on refusal; output retains existing focused-view limits.
- `history prune` previews exact candidate cycle files, digests, bytes and protection reasons without writing. It protects active, newest-three, younger-than-14-day and unresolved fork/orphan cycles and every checkpoint, bug, case, native-manifest and frozen-evidence reference. Unreadable protection sources prevent deletion.
- Only an explicit `history prune --apply` with existing human retention authorization may remove eligible complete closed cycles; evidence auto-retention never starts history pruning. Legacy anchors/logs, migration backups and native archives remain untouched.
- An immutable receipt bounded at 1 MiB names planned paths/digests and authorization before deletion. Present files remain retained, absent receipted files are pruned and absent unlisted files are unavailable. Interruption is explicit partial cleanup, never completed timing evidence.
- Protected-only data yields a truthful no-op. The report explains that retained proof may grow beyond the routine retention target; no global disk-size guarantee is inferred.
- A real Git fixture integrates supported retention with another branch's new events/evidence, retaining all surviving references and artifact hashes.

### REQ-history-004

Conservative evidence cleanup. Evidence retention SHALL keep the existing eligibility thresholds, protection semantics and ledger entry shape, rebuild protection at apply and revalidate candidate identity and safe paths immediately before removal. Changed, missing, shared, newly referenced or unsafe files SHALL be retained or refused with explicit outcomes. Interrupted cleanup SHALL distinguish present content, recorded removal and unavailable history. It SHALL preserve the active checkpoint, migration backup, historical retained proof, native archives and every artifact still referenced by another observation. Trace: LIFE-2, LIFE-7, LIFE-8, FLOW-7, FLOW-13.

Acceptance Criteria
- Adding a checkpoint, bug, case or native protection reference after preview protects the relevant observation/cycle when apply runs.
- Shared artifacts remain present and byte-identical; missing/changed candidates, malformed sources, path traversal and symlinks cannot broaden deletion or be reported as a successful complete cleanup.
- Fault injection around publication/removal yields an actionable partial outcome and leaves active state and protected bytes intact. No existing observation, artifact fingerprint or evidence-ledger schema is rewritten.

### REQ-history-005

Backfill on transition records. A transition MAY carry `backfill` with `recorded_at_unix_seconds`, the full `commit` SHA and the `quote` it rests on, meaning its `at_unix_seconds` is the original time given when it was recorded later (REQ-cli-036). The field SHALL be omitted when absent, so every other transition keeps its exact bytes, and SHALL be carried unchanged by the legacy transitions log and by immutable history events. Trace: DRIVE-5, LIFE-7.

Acceptance Criteria
- A transition without a backfill serializes byte-identically to before.
- A backfilled transition written with history adopted is read back from its event with the same backfill.

### REQ-history-006

Sessions per objective. `chough session new <key>` SHALL create `.chough/sessions/<key>.json`, a schema 2 checkpoint with its own history cycle, without replacing a file, and point at it; `session adopt <key>` SHALL move an existing checkpoint with a history cursor there byte for byte; `session use <key>` SHALL point at an open session; `session list` SHALL report each session's key, stage, status (`archived` for an archived session, otherwise its stage status) and whether it is active or archived. `session new` and `session use` SHALL move every other session at stage done to `.chough/sessions/archive/<key>.json` unchanged, except that a done session with unfolded inbox entries SHALL stay open and be reported as kept open. `chough inbox add --session <key>` SHALL create one entry file under `.chough/sessions/<key>.inbox/` and SHALL NOT edit any session; when an `observation` entry names an observation with no record in `.chough/evidence/`, `inbox add` SHALL still add it and SHALL warn on stderr, naming the ID, and `inbox list` SHALL mark that entry `recorded: false`; `inbox fold` SHALL add the active session's unfolded entries to its lists without duplicates and move each into `folded/`. Inbox entries SHALL carry no approval, review, consent, gate or stage field. Two branches that record work in different sessions SHALL merge without conflict. Evidence retention SHALL treat every open and archived session as a checkpoint reference source, and SHALL refuse a preview or apply when any session cannot be read. Trace: DRIVE-3, LIFE-8, LIFE-3, LIFE-7.

Acceptance Criteria
- Two branches that each advance a different session merge in Git with no conflict and keep both sessions' history.
- Switching away from a done session moves it to the archive with its bytes unchanged.
- Folding an inbox twice adds each entry once, and adding an entry never changes a session file.
- A done session with unfolded inbox entries stays open on a switch.
- An observation referenced only by an inactive or archived session is protected from `evidence prune --apply`, and an unreadable session file refuses it with nothing removed.
- `session list` reports an archived session with status `archived`.
- `inbox add --kind observation` with an ID that has no evidence record adds the entry, warns on stderr naming the ID, and `inbox list` marks it `recorded: false`; a recorded observation gets no warning and `recorded: true`.

### REQ-history-007

Local state directory. `.chough/local/` SHALL hold only Git-ignored local state of one worktree, with a `.gitignore` of `*`: the active-session pointer in sessions mode and the checkpoint writer lock (REQ-cli-040). A project without `.chough/sessions/` SHALL gain no tracked file and no session file from sessions support; its first checkpoint writer creates only `.chough/local/` with that `.gitignore` and the lock. Trace: DRIVE-3, DRIVE-11.

Acceptance Criteria
- In a project without sessions, `init` and `stage advance` create no `.chough/sessions/`, and `.chough/local/` holds only `.gitignore` and `checkpoint.lock`, both ignored by Git.
- In sessions mode, `.chough/local/` holds only `.gitignore`, `checkpoint.lock` and `active-session`.

### REQ-history-008

No orphan event from a vanished checkpoint. When a command's save is refused because the checkpoint it resolved was archived, removed or adopted into sessions mode while it ran, the helper SHALL first remove the history events that command published, and any cycle directory it reserved that is then empty, when their bytes are unchanged and they were published after the command last found its checkpoint in place. A save finds its checkpoint in place just before it writes, so the helper leaves no orphan event and never removes an event a save may have written, wherever that checkpoint is moved afterwards. Any other failed save SHALL keep REQ-history-002's unselected orphan. Chough's own `session use` and `session new` SHALL wait for a running checkpoint writer rather than archive or switch its session under it. Trace: DRIVE-3, LIFE-7.

Acceptance Criteria
- A `stage advance` whose session file is moved away during its native read refuses with nothing saved and leaves no new event file.
- `session use` started during a `stage advance` waits for it, and the advance records exactly its own event.
- A session file moved away after a successful save keeps the event that save points to.
- A session file moved after a successful save to a path outside `.chough/sessions/` keeps the event that save points to.

