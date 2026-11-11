# Chough 0.6.0: sessions in their own folders with an append-only timeline, one archived story each, and no proof trimming

## ADDED

### REQUIREMENT REQ-history-015

Append-only corrections. `chough correct <entry> --reason <text>` SHALL append a `correction` entry to the timeline that holds `<entry>`, giving `corrects: ^<entry id>` and the statement, and SHALL leave every existing byte of that file unchanged, so history is never rewritten. `<entry>` SHALL be `^<entry id>` for the active session, or `<session id>#^<entry id>` for any open session or archived story, the session ID or an alias (REQ-records-003). Correcting an entry of an archived story SHALL append to that story's Timeline and change nothing else in it (REQ-records-012). It SHALL refuse, writing nothing, an unknown session or entry, a blank statement or one over 4,096 bytes, and a correction of a correction. `chough next` and `chough status` SHALL show the corrected text of a corrected decision or next action, marked as corrected with the count of corrections, for the active session and for archived stories they show; `chough next` SHALL list at most 10 corrections, newest first by recorded time and then entry ID, each cut to 512 bytes, with their total. `chough timing` SHALL treat a correction as a known kind that starts and ends no stage segment. Trace: LIFE-18, LIFE-7, FLOW-13.

Acceptance Criteria
- Correcting a stage move's reason appends one `correction` entry linked to it, and the session file is otherwise byte-identical; `next` and `status` show the corrected text marked as corrected.
- Correcting an entry of an archived story appends to its Timeline only, and every earlier byte of the story is unchanged.
- An unknown entry, a blank or 4,097-byte statement and a correction of a correction are refused with nothing written.
- With 12 corrections, `next` lists 10 newest first and says 12 in total; `timing --json` reports no unrecognised kind.

## MODIFIED

### REQUIREMENT REQ-history-002

Append-only timeline. Each session's transitions SHALL be recorded as entries of the `## Timeline` section of its `session.md` (REQ-records-004), and those entries, carried into the archived story at finish, SHALL be the session's only transition history: no event file, cycle directory, cursor or transitions log SHALL be written. Each entry SHALL be one list item, `- <time> · <kind> · <from> → <to> · <text> ^<entry id>`, where `<time>` is the UTC time to the second in ISO 8601, `<kind>` is a transition kind (`advance`, `iterate`, `back`, `reset`, `pause`, `resume`, `skip`, `enter`, `return`, `gate-opened`, `gate-closed`, `direct-native-write`, `correction`), `<text>` is the reason, gate or note, and `<entry id>` is `t-` and 6 lowercase hexadecimal digits from fresh random bits, unique in the file. Its structured fields SHALL follow as indented `- <key>: <value>` lines: `by` for the actor (REQ-records-007), `native_change` when one is recorded, and the fields of REQ-history-005, REQ-history-010, REQ-history-013, REQ-history-014 and REQ-history-015. The entry and the checkpoint change it goes with SHALL be written in the same atomic replace of `session.md`, so a refused or failed save leaves neither. Chough SHALL only append entries: it SHALL never edit, reorder or remove an existing entry line, and an entry ID SHALL never be reused in its file. A timeline entry is what other requirements call a transition, and the session file's Timeline is what they call its transition log. `chough doctor` SHALL report a session or story whose Timeline at HEAD is not an exact prefix of its Timeline in the working tree (REQ-history-012). Trace: LIFE-2, LIFE-3, LIFE-6, LIFE-7, FLOW-2, FLOW-13, DRIVE-3, DRIVE-24.

Acceptance Criteria
- Every transition kind written in a fixture session appears once, in order, as an entry with its time, kind, stages, text, unique entry ID and structured fields, and no file is written under `.chough/history/` or `.chough/transitions.jsonl`.
- A save refused after its native read leaves `session.md` byte-identical, with no entry and no checkpoint change.
- Two branches that each advance a different session merge with no conflict and keep both timelines.
- Editing an earlier entry by hand and committing nothing else makes `chough doctor` report the session's timeline as rewritten.
- Each session's timeline holds only its own entries, and a stage move in one session leaves every other session's file unchanged.

### REQUIREMENT REQ-history-003

Bounded timeline. Each timeline entry, with its structured fields, SHALL be at most 65,536 bytes, each session's timeline at most 4,096 entries, and a session file within the bound of REQ-records-001. An append that would pass a bound SHALL be refused before the save, naming the bound and the recovery, `chough session close <id> --reason <text>` and then `chough session new <key>`, with the session file byte-identical. Recent guidance in `chough next` SHALL read at most the last 128 entries of the active timeline. `chough timing` SHALL read at most 4,096 entries and 16 MiB of timeline content in one request, across the open sessions and archived stories it reads, and SHALL report what it did not read as partial, never as zero or verified. Trace: LIFE-3, LIFE-7, LIFE-10, FLOW-13.

Acceptance Criteria
- An entry of 65,537 bytes and the 4,097th entry of a timeline are refused naming their bound and the recovery, with the session file unchanged.
- `chough next` on a session with 4,096 entries reads only its last 128 entries and answers within the focused-view bounds.
- A timing request over more than 16 MiB of timelines reports partial, naming what it did not read.

### REQUIREMENT REQ-history-005

Backfill on timeline entries. A backfilled `advance` entry (REQ-cli-036) SHALL carry `recorded_at`, the UTC time it was written, `commit`, the full commit SHA, and `quote`, the words it rests on, as structured fields, and its `<time>` SHALL be the original time given. Every entry without a backfill SHALL carry none of the three. Trace: DRIVE-5, LIFE-7.

Acceptance Criteria
- A backfilled entry reads back with its original time, write time, full commit SHA and quote, and `timing` lists it as backfilled.
- An entry without a backfill has none of the three fields.

### REQUIREMENT REQ-history-006

Sessions in their own folders. `chough session new <key>` SHALL create `.chough/sessions/<id>/session.md` by create-new, where `<id>` is the date and the slug of `<key>` (REQ-records-003) and `<key>` is kept as an alias, with its own timeline, and SHALL point at it; `session use <id>` SHALL point at an open session by ID, alias or the bare slug when exactly one open session has it; `session list` SHALL report each session's ID, aliases, stage, status (`archived` for an archived story, otherwise its stage status) and whether it is active, open or archived. A session is finished when it is at stage done or its native change is archived on HEAD, at any stage, as the archive's `finalization.json` records, read without a native read (DRIVE-16). A session SHALL close only itself, at its own move into done or with `chough session close --reason`: `session new` and `session use` SHALL NOT archive, close or change any other session, even a finished one (#207, #214). `session list`, `chough next` and `chough doctor` SHALL list each finished open session with its fix, `chough doctor --fix` (REQ-history-012), which keeps one with unfolded inbox entries or an open awaiting-human gate open and names the recovery: `chough inbox fold`, or `chough checkpoint --close-gate`, since closing a gate is a decision (#181). When `chough stage advance` moves the active session into done and its save succeeds, after the knowledge refresh of REQ-cli-043, the helper SHALL archive that session in the same command and clear the active pointer, reporting `applied.session_archive` with status `archived` and the ID; a session with unfolded inbox entries SHALL stay open with its pointer, reported as `kept_open` with the count; a session for which release is authorized at that move (a run consent or a standing preference, as `stage enter release` requires) SHALL stay open for the release, reported as `kept_open` with that reason, and SHALL archive itself the same way at the move from release into done. The step SHALL NOT undo the move into done. To archive a session, Chough SHALL write `.chough/archive/sessions/<id>.md` by create-new: the session's front matter with `stage: done` (or the stage it closed at), `closed` (the UTC date) and `outcome` (`done` or `closed-without-done`), and `receipts`, one map per evidence entry the session holds with its `id`, `aliases`, `claim`, `result`, `command`, `revision`, `environment`, `content`, `fingerprint`, `case`, `case_digest` and `screenshot` when present; then the story as written, with one What was verified line per receipt (its claim, its command and the first 12 digits of its fingerprint, ending `^<evidence id>`) for each entry not already listed, the Timeline and a closing line `Closed <date>: <outcome>.` Chough SHALL read the story back and compare it before it removes anything, and SHALL then remove the session folder, `session.md`, `evidence/` and `agent/` included, and move its screenshots beside the story (REQ-records-013), each file only under REQ-records-012; files it keeps SHALL be reported, the folder SHALL count as archived while its story exists, and `chough doctor --fix` SHALL finish the removal once they are committed. Archiving a session SHALL NOT read, move, rewrite or remove any other session's folder or story. After it, `next` and `status` SHALL refuse with "no open session" and name `chough session new <key>` until a session is opened (DRIVE-13). Evidence and bug links into an archived session SHALL resolve to its story's receipts. Two branches that record work in different sessions SHALL merge without conflict. A session that cannot be read, because it is over the bound of REQ-records-001 or invalid, SHALL NOT stop `session list`, `session new`, `session use` or another session's `next`; `session list` SHALL report it with the reason and the recovery `chough session shelve <id>`, which SHALL move the folder unparsed and byte for byte to `.chough/archive/shelved/<id>/` with a `shelved.md` giving its size, SHA-256 per file, time and reason, and clear the active pointer when it named that session. Shelving SHALL never replace a file, and no reader SHALL scan `.chough/archive/shelved/` (DRIVE-14). `chough session close <id> --reason <text>` SHALL close an open session for abandoned work without a native read: it SHALL refuse with nothing changed a blank reason, a session that is archived or does not exist, and a session with unfolded inbox entries, naming `chough session use <id>` and `chough inbox fold`; otherwise it SHALL add the decision `Closed without done: <reason>` and archive the session as above with outcome `closed-without-done`, clearing the active pointer when it named that session (DRIVE-16). Trace: DRIVE-3, DRIVE-24, DRIVE-25, KNOW-30, KNOW-34, LIFE-3, LIFE-7, DRIVE-14, DRIVE-13, DRIVE-16.

Acceptance Criteria
- Two branches that each advance a different session merge in Git with no conflict and keep both sessions' timelines.
- The move into done of the active session writes `.chough/archive/sessions/<id>.md` with every receipt and the full timeline, removes the committed session folder, clears the pointer, reports `archived`, and `next` then refuses naming `chough session new <key>`.
- Regression for issue 165: with seven earlier archived stories present, among them 0.5.x-era stories, and `knowledge/` neither tracked nor ignored, advancing accept to done leaves every earlier story byte-identical in place, `git status` shows no deleted file, and the only new committed record is the new story; the same path in the 0.5.x layout is recorded failing first on the 0.5.0 code and linked to a local bug.
- With an uncommitted file in `agent/`, the move into done writes the story, keeps that file and its folder, reports it, and `next` does not treat the folder as open; after a commit, `doctor --fix` removes the folder.
- At the move into done, a session with unfolded inbox entries stays open (`kept_open` with the count).
- Regression for #207: with a finished session whose native change is archived and that still waits on a publish gate, `session new` and `session use` leave it byte-identical and open, and `next` and `doctor` list it as finished with its fix.
- With release authorized before the move into done, the session stays open (`kept_open`), `stage enter release` works, and the move from release into done archives it.
- A session whose native change is archived on HEAD, at a stage short of done, is listed as finished with no native read, and only `chough doctor --fix` archives it.
- `session list` reports open, active and archived sessions; `session use` accepts the ID, the alias and a unique bare slug.
- With one session over its bound and another unreadable, `session list` reports each with `chough session shelve <id>`, and `next` on the active session and `session new` still work; shelving moves the folder byte for byte under `.chough/archive/shelved/`.
- `chough session close <id> --reason <text>` archives the session with outcome `closed-without-done` and the reason in its decisions; a blank reason, an archived or missing session and a session with unfolded inbox entries refuse with nothing changed.

### REQUIREMENT REQ-history-007

Local state directory. `.chough/local/` SHALL hold only Git-ignored state of one worktree, with a `.gitignore` of `*`: the active-session pointer, the writer lock (REQ-cli-040), gate lists (REQ-cli-061), direct-write baselines (REQ-cli-060), `external-artifacts.toml` and `logs/` (REQ-records-005), an optional `identity.toml` (REQ-records-008) and the doctor's scan cache when one is kept. Every machine-specific fact Chough keeps SHALL be there and nowhere else. Trace: DRIVE-3, DRIVE-11, LIFE-26.

Acceptance Criteria
- After a fixture cycle with an external artifact, a case run and a gate list, every file under `.chough/local/` is ignored by Git, and no committed record names a file in it except by a neutral ID.
- `git status` after `chough init` shows no file under `.chough/local/`.

### REQUIREMENT REQ-history-009

Inbox entries in the session's agent folder. `chough inbox add --session <id>` SHALL create one entry as a Markdown record (REQ-records-001) under `.chough/sessions/<id>/agent/inbox/`, giving `kind`, `value`, `from` and `recorded` when it names an observation, and SHALL NOT edit any session file. It SHALL hold the project's writer lock from before it checks that the session is open until it exits, so a concurrent `session use` or `session new` either sees the entry and keeps a done session open, or archives it first and the add refuses with nothing written; an entry SHALL never be left beside an archived session. It SHALL publish the entry whole, by linking a complete temporary file into place, and the next save SHALL remove a temporary file an interrupted add left. When an `observation` entry names no evidence entry, `inbox add` SHALL still add it, warn on standard error naming the ID, and `inbox list` SHALL mark it `recorded: false`. `inbox fold` SHALL add the active session's unfolded entries to its sections without duplicates and move each into `agent/inbox/folded/`. Inbox entries SHALL carry no approval, review, consent, gate or stage field. Trace: DRIVE-3, DRIVE-24, LIFE-8.

Acceptance Criteria
- Folding an inbox twice adds each entry once, and adding an entry never changes a session file.
- While another process holds the lock and archives the session, `inbox add` waits, then refuses with "no open session" and writes no entry.
- An `inbox add` killed while writing leaves no partial entry, and the next save removes its temporary file.
- `inbox add --kind observation` with an unrecorded ID adds the entry, warns naming the ID, and `inbox list` marks it `recorded: false`.

### REQUIREMENT REQ-history-010

Relayed decision on gate entries. A `gate-closed` entry MAY carry the structured fields `decided_by`, `words`, `where` and `recorded_by`, each a non-blank value of at most 4,096 bytes (REQ-cli-050), and, when the gate drove a native step (REQ-records-008), `native` naming the SpecSync command run and `actor`, the SpecSync actor it ran as. The four relay fields SHALL appear together or not at all, SHALL appear only on a `gate-closed` entry, and an entry breaking either rule SHALL be refused as an invalid entry. The record is a relayed claim: it does not authenticate who decided. Trace: FLOW-16, FLOW-13, FLOW-23.

Acceptance Criteria
- A relayed `gate-closed` entry reads back with the four values byte for byte, and one that drove an approval also reads back its `native` and `actor`.
- An entry with three of the four, a blank or over-long value, or relay fields on a `gate-opened` or stage entry, is refused as invalid.

### REQUIREMENT REQ-history-012

Doctor findings and fixes. `chough doctor` (REQ-cli-055) SHALL find this debt without a native read, and its fixes SHALL use Chough's own steps:
- `unmigrated`: a 0.5.x record (REQ-records-010). The fix SHALL name `chough migrate` and SHALL NOT migrate.
- `leak`: a committed record that matches REQ-records-006, with its file, line and kind and never the text. The fix SHALL name `chough doctor --fix --redact`, which SHALL redact the matches in place and is not a decision.
- `stale_pointer`: the active pointer names a session that is missing or archived. The fix SHALL remove the pointer.
- `finished_session`: an open session that is finished (REQ-history-006) and has no open awaiting-human gate. The fix SHALL archive it. A session with unfolded inbox entries SHALL instead be `skipped`, naming `chough inbox fold`.
- `unfinished_archive`: a session folder whose story exists and that still holds files. The fix SHALL remove each file whose bytes are committed at HEAD (REQ-records-012) and report the rest as `skipped`, naming the commit to make.
- `stale_gates`: an open session whose native change is archived on HEAD and that has open awaiting-human gates. This fix is a decision. With `--reason`, it SHALL withdraw those gates as `chough checkpoint --clear-awaiting-human` does, recording no human decision, and SHALL then archive the session as the `finished_session` fix does.
- `unfinished_session`: an open, unfinished session named with `--close <id>`. This fix is a decision. With `--reason`, it SHALL close the session as `chough session close <id> --reason` does.
- `timeline_rewritten`: a session or story whose Timeline at HEAD is not an exact prefix of its working Timeline (REQ-history-002). It has no fix; the finding names the entries that differ.
- `alias_conflict`: an ID that is the `id` or alias of two records (REQ-records-003). It has no fix; the finding names both records.
- `records_in_verification`: `.specsync/sdd.json` exists and its `verification_ignored_paths` lacks one of Chough's record folders (REQ-records-014). It has no fix, because Chough never writes SpecSync's policy; the finding names the entries to add.

The fixes SHALL run in that order. They SHALL never replace a file, and SHALL leave the bytes of every session and story they do not fix unchanged. Trace: DRIVE-17, DRIVE-16, DRIVE-13, DRIVE-3, DRIVE-4, LIFE-26, LIFE-20.

Acceptance Criteria
- A finished session at stage intent and one at done are archived, a stale pointer is removed, and open work stays open.
- An unmigrated tree reports `unmigrated` naming `chough migrate`, and `--fix` changes nothing under `.chough/`.
- A committed home path reports `leak` without the path; `--fix --redact` replaces it with `<redacted:home-path>` and the next run finds nothing.
- A finished session folder holding an uncommitted file is `unfinished_archive`; `--fix` keeps that file, and after a commit removes the folder.
- A hand-edited earlier timeline entry reports `timeline_rewritten`, and an alias held by two bugs reports `alias_conflict`, each with no fix applied.
- Without `--reason`, a stale gate and a session named by `--close` are skipped with their files unchanged; with `--reason` the gate is withdrawn with no human decision recorded and the session is archived, and the named session is closed with the reason.

### REQUIREMENT REQ-history-013

Direct-write entries. A `direct-native-write` entry SHALL have `<from>` and `<to>` equal to the current stage and the structured fields `change`, the native change; one `file` line per changed file, giving its path under `.specsync/changes/<id>/` and `added`, `changed` or `removed`; and `before_state` and `after_state`, the change's state from `state.json`, or `none` when it is absent. No other entry SHALL carry these fields. Readers that count unknown kinds, such as `chough timing`, SHALL treat it as a known kind that starts or ends no stage segment. Trace: DRIVE-21.

Acceptance Criteria
- A recorded `direct-native-write` entry reads back with its change, files and states, and the same fields on any other kind are refused as invalid.
- `chough timing` reads a timeline holding a `direct-native-write` entry as complete, with no unrecognised kind and no partial mark.

### REQUIREMENT REQ-history-014

Summary digests on relays. A relayed `gate-closed` entry (REQ-history-010) MAY carry `summary_digest`, 64 lowercase hex digits, and `list`, 12 lowercase hex digits, both or neither; one of them alone, or a malformed value, SHALL be refused as an invalid entry. Trace: FLOW-19, FLOW-20.

Acceptance Criteria
- A relay with both fields reads back; a relay with one of them, or a malformed value, is refused as invalid.

## REMOVED

### REQUIREMENT REQ-history-001

Retired in Chough 0.6.0: committed observation sets no longer pile up for retention to prune. Evidence lives in its session and folds into the archived story at finish (REQ-history-006), raw logs are never committed (REQ-records-005), and files leave the tree only under REQ-records-012. `chough evidence prune` and the retention ledger are removed; LIFE-8's removed-not-passed rule is kept by REQ-records-009.

### REQUIREMENT REQ-history-004

Retired in Chough 0.6.0 with `chough evidence prune` and the retention ledger (see REQ-history-001); REQ-records-012 keeps the rule that only committed bytes ever leave the tree.

### REQUIREMENT REQ-history-008

Retired in Chough 0.6.0: there are no separate event files to orphan. A timeline entry and its checkpoint change are one atomic replace of `session.md` (REQ-history-002), and `session use` and `session new` still wait for a running writer (REQ-cli-040, REQ-history-009).

### REQUIREMENT REQ-history-011

Retired in Chough 0.6.0 and superseded by KNOW-30 and KNOW-34: archived session stories are never moved, copied or trimmed into the vault, and every one stays in `.chough/archive/sessions/` (REQ-history-006, REQ-records-012). The roll-up caused the data loss of issue 165, whose regression REQ-history-006 carries.
