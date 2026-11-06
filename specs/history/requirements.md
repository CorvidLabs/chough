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

Sessions per objective. `chough session new <key>` SHALL create `.chough/sessions/<key>.json`, a schema 2 checkpoint with its own history cycle, without replacing a file, and point at it; `session adopt <key>` SHALL move an existing checkpoint with a history cursor there byte for byte; `session use <key>` SHALL point at an open session; `session list` SHALL report each session's key, stage, status (`archived` for an archived session, otherwise its stage status) and whether it is active or archived. `session new` and `session use` SHALL move every other finished session to `.chough/sessions/archive/<key>.json` unchanged, except that a finished session with unfolded inbox entries SHALL stay open and be reported as kept open. A session is finished when it is at stage done or when its native change is archived on HEAD, at any stage, as an archive's `finalization.json` records, read without a native read (DRIVE-16). When `chough stage advance` moves the active session into done and its save succeeds, after the steps of REQ-cli-041, REQ-history-011, REQ-cli-043 and REQ-cli-049, the helper SHALL move that session to `.chough/sessions/archive/<key>.json` unchanged, in the same change, and clear the active pointer, reporting `applied.session_archive` with status `archived` and the key; a session with unfolded inbox entries SHALL stay open and the active pointer SHALL be kept, reported as `kept_open` with the count of unfolded entries; a session for which release is authorized at that move (a run consent or a standing preference, as `stage enter release` requires) SHALL stay open for the release, reported as `kept_open` with that reason, and SHALL archive itself the same way at the move from release into done; without sessions (the single checkpoint) it SHALL report `skipped`. The step SHALL NOT undo the move into done. After it, `next` and `status` SHALL refuse with "no open session" and name `chough session new <key>` until a session is opened, and a later `session new` or `session use` SHALL have no session of that change to archive (DRIVE-13). `chough inbox add --session <key>` SHALL create one entry file under `.chough/sessions/<key>.inbox/` and SHALL NOT edit any session; when an `observation` entry names an observation with no record in `.chough/evidence/`, `inbox add` SHALL still add it and SHALL warn on stderr, naming the ID, and `inbox list` SHALL mark that entry `recorded: false`; `inbox fold` SHALL add the active session's unfolded entries to its lists without duplicates and move each into `folded/`. Inbox entries SHALL carry no approval, review, consent, gate or stage field. Two branches that record work in different sessions SHALL merge without conflict. Evidence retention SHALL treat every open, archived and shelved session as a checkpoint reference source, reading a shelved one within the 16 MiB ledger bound, and SHALL refuse a preview or apply when any session cannot be read. A session that cannot be read, because it is over the checkpoint cap of REQ-cli-032 or invalid, SHALL NOT stop `session list`, `session new`, `session use` or another session's `next`: `session list` SHALL report it in its own entry with the reason and the recovery `chough session shelve <key>`. `chough session shelve <key>` SHALL move `.chough/sessions/<key>.json` unparsed and byte for byte to `.chough/sessions/shelved/<key>.json`, write `.chough/sessions/shelved/<key>.shelved.json` with its size, SHA-256, time and reason, and clear the active pointer when it named that session; `chough session shelve --legacy` SHALL do the same for `.chough/session.json` as `legacy`. Shelving SHALL never replace a file, and no reader other than retention SHALL scan `shelved/` (DRIVE-14). `chough session close <key> --reason <text>` SHALL close an open session for abandoned work without a native read: it SHALL refuse with nothing changed a blank reason, a session that is archived or does not exist, and a session with unfolded inbox entries, naming `chough session use <key>` and `chough inbox fold`; otherwise it SHALL add the decision `Closed without done: <reason>` to that session, write it to `.chough/sessions/archive/<key>.json` without replacing a file, remove its open file, and clear the active pointer when it named that session (DRIVE-16). Trace: DRIVE-3, LIFE-8, LIFE-3, LIFE-7, DRIVE-14, DRIVE-13, DRIVE-16.

Acceptance Criteria
- Two branches that each advance a different session merge in Git with no conflict and keep both sessions' history.
- Switching away from a done session moves it to the archive with its bytes unchanged.
- Folding an inbox twice adds each entry once, and adding an entry never changes a session file.
- A done session with unfolded inbox entries stays open on a switch.
- An observation referenced only by an inactive or archived session is protected from `evidence prune --apply`, and an unreadable session file refuses it with nothing removed.
- `session list` reports an archived session with status `archived`.
- `inbox add --kind observation` with an ID that has no evidence record adds the entry, warns on stderr naming the ID, and `inbox list` marks it `recorded: false`; a recorded observation gets no warning and `recorded: true`.
- With one session over the cap and another unreadable, `session list` reports each with `chough session shelve <key>`, and `next` on the active session and `session new` still work.
- `chough session shelve <key>` moves an oversized session byte for byte into `shelved/`, after which `session new` and `next` work; `chough session shelve --legacy` does the same for an oversized `.chough/session.json`.
- The move into done of the active session moves it to `.chough/sessions/archive/` and clears the active pointer, reported as `archived`; `next` then refuses naming `chough session new <key>`, and a following `session new` archives nothing.
- At the move into done, a session with an unfolded inbox entry stays open with its pointer, reported as `kept_open` with the count; without sessions the step reports `skipped`.
- With release authorized before the move into done, the session stays open (`kept_open`), `stage enter release` works, and the move from release into done archives it and clears the pointer.
- A session whose native change is archived on HEAD, at a stage short of done, is moved to the archive with its bytes unchanged by `session new` and by `session use`, and stays open while its inbox has unfolded entries, with no native read.
- `chough session close <key> --reason <text>` moves the session to the archive with the reason in its decisions and no native read, and clears the active pointer when it named that session; a blank reason, an archived or missing session and a session with unfolded inbox entries refuse with nothing changed.

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

### REQ-history-009

Inbox entries and their sessions. `chough inbox add` SHALL hold the project's checkpoint writer lock from before it checks that its session is open until it exits, so a concurrent `session use` or `session new` either sees the entry and keeps a done session open, or archives it first and the add refuses with nothing written; an entry SHALL never be left beside an archived session. `inbox add` SHALL publish its entry whole, by linking a complete temporary file into place, so an add stopped while writing leaves no partial entry; the temporary file it leaves is ignored by Git and removed by the next checkpoint save. Evidence retention SHALL treat the observation IDs named by unfolded inbox entries of open and archived sessions as references, labelled `inbox of session <key>`, and SHALL refuse when such an entry cannot be read. Trace: DRIVE-3, LIFE-8.

Acceptance Criteria
- While another process holds the lock and archives the session, `inbox add` waits, then refuses with "no open session" and writes no entry.
- An old observation named only by an unfolded inbox entry is protected as `inbox of session <key>` and survives `evidence prune --apply`.
- An `inbox add` killed while writing its entry leaves no partial entry: `inbox list`, `status`, `next` and `evidence prune` still work, and the next checkpoint save removes the temporary file it left.
- When an unfolded inbox entry cannot be read, `evidence prune --apply` refuses and removes nothing.

### REQ-history-010

Relayed decision on gate records. A `gate-closed` transition MAY carry `relayed_by` with exactly the keys `decided_by`, `words`, `where` and `recorded_by`, each a non-blank string of at most 4096 bytes (REQ-cli-050). The field SHALL be omitted when absent, so every other transition keeps its exact bytes, and SHALL be carried unchanged by the legacy transitions log and by immutable history events. A `relayed_by` with a missing, blank, over-long or unknown key, or on a transition of any other kind, SHALL be refused as an invalid transition payload. The record is a relayed claim: it does not authenticate who decided. Trace: FLOW-16, FLOW-13.

Acceptance Criteria
- A transition without `relayed_by` serializes byte-identically to before.
- A relayed gate-closed transition written with history adopted, and one written to the legacy transitions log, are read back with the same four values.
- An event whose `relayed_by` lacks a key, has a blank or over-long value or an unknown key, or sits on a `gate-opened` or stage transition, is refused as an invalid transition payload.

### REQ-history-011

Archived session roll-up. When `chough stage advance` moves into done and its save succeeds, with the librarian on, after the trim of REQ-cli-041 and before the knowledge refresh of REQ-cli-043, the helper SHALL keep in `.chough/sessions/archive/` only the five most recent archived sessions, apart from the session that adopted the legacy history. Recency SHALL be the finalization timestamp of the session's native change, newest first, then the key, the later key first. A session without a native change, or whose change has no readable finalization, SHALL count as the oldest. The archived session whose history cursor holds the legacy anchor (the session that adopted the legacy history) SHALL never be rolled into the vault: it SHALL stay in `.chough/sessions/archive/` in addition to the five and SHALL NOT count toward them, so history never depends on the knowledge directory. Each other archived session SHALL move byte for byte to `<library-dir>/attachments/<folder of its key>/sessions/<key>.json`, with the folder formed as in REQ-cli-046, and SHALL never replace a file. An attachment that already holds the same bytes SHALL be kept, and the source removed. One that holds different bytes SHALL leave the session in place and be reported. It SHALL report `applied.session_rollup` with status `rolled` and the moved keys, `unchanged`, `skipped` (librarian off), or `refused` with its reason, and SHALL NOT undo the move into done. A rolled session is no longer a retention reference source: its cycle is closed, and its native archive keeps what the change touched. `session list` SHALL NOT list it. Nothing SHALL be deleted from Git history. Trace: KNOW-29, DRIVE-13, KNOW-13.

Acceptance Criteria
- With seven archived sessions, five tied to archives finalized 60 to 20 days ago, one tied with the oldest, and one without a native change, the move into done keeps the five newest in `.chough/sessions/archive/` and moves the other two byte for byte into `knowledge/attachments/<key>/sessions/<key>.json`, reported as `rolled`.
- With the librarian off, no session moves and the step reports `skipped`.
- With seven archived sessions of which only the one that would roll first holds the legacy anchor, the move into done keeps that session and the five newest of the other six in `.chough/sessions/archive/`, rolls only the remaining one, and the anchor holder has no copy in the vault.

### REQ-history-012

Doctor findings and fixes. `chough doctor` (REQ-cli-055) SHALL find this debt without a native read, and its fixes SHALL use Chough's own steps:
- `legacy_checkpoint`: `.chough/session.json` exists. The fix SHALL adopt history first when the checkpoint has no history cursor, and then adopt it as a session (REQ-history-006) under its native change ID or under `--adopt-as <key>`. With neither key, the finding SHALL be `skipped`, with a fix that names `--adopt-as <key>`.
- `stale_pointer`: the active pointer names a session that is missing or archived. The fix SHALL remove the pointer.
- `finished_session`: an open session that is finished (REQ-history-006) and has no open awaiting-human gate. The fix SHALL archive it with its bytes unchanged. A session with unfolded inbox entries SHALL instead be reported as `skipped`, with a fix that names `chough inbox fold`.
- `stale_gates`: an open session whose native change is archived on HEAD and that has open awaiting-human gates. This fix is a decision. With `--reason`, it SHALL withdraw those gates as `chough checkpoint --clear-awaiting-human` does, recording no human decision, and SHALL then archive the session as the `finished_session` fix does.
- `unfinished_session`: an open, unfinished session named with `--close <key>`. This fix is a decision. With `--reason`, it SHALL close the session as `chough session close <key> --reason` does.

The fixes SHALL run in that order. They SHALL never replace a file, and SHALL leave the bytes of every session they do not fix unchanged. Trace: DRIVE-17, DRIVE-16, DRIVE-13, DRIVE-3.

Acceptance Criteria
- A finished session at stage intent and one at done are archived byte for byte, a stale pointer is removed, and open work stays open.
- A legacy checkpoint with a native change and no history cursor is adopted under that change's ID, with a cursor. One with no native change is skipped, naming `--adopt-as <key>`, and `--adopt-as <key>` adopts it.
- A finished session with unfolded inbox entries stays open and is reported as skipped, naming `chough inbox fold`.
- Without `--reason`, a stale gate and a session named by `--close` are skipped with their files unchanged. With `--reason`, the gate is withdrawn with no human decision recorded and the session is archived, and the named session is closed with the reason.

