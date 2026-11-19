# history delta

## ADDED

(none)

## MODIFIED

### REQUIREMENT REQ-history-012

Doctor findings and fixes. `chough doctor` (REQ-cli-055) SHALL find this debt without a native read, and its fixes SHALL use Chough's own steps:
- `unmigrated`: a 0.5.x record (REQ-records-010). The fix SHALL name `chough migrate` and SHALL NOT migrate.
- `leak`: a committed record that matches REQ-records-006, with its file, line and kind and never the text. The fix SHALL name `chough doctor --fix --redact`, which SHALL redact the matches in place and is not a decision, except a match in an ID, a front matter line that gives a record's `id`, its aliases or an ID it references (REQ-records-003): `--redact` SHALL never rewrite an ID, since a redacted ID no longer reads. That finding SHALL name the recovery instead, `chough session shelve <id>` for a session record and a reviewed hand edit for any other record, and SHALL stay `skipped` under `--fix --redact`, while the other matches of the same record are redacted.
- `stale_pointer`: the active pointer names a session that is missing or archived. The fix SHALL remove the pointer.
- `finished_session`: an open session that is finished (REQ-history-006) and has no open awaiting-human gate. The fix SHALL archive it. A session with unfolded inbox entries SHALL instead be `skipped`, naming `chough inbox fold`.
- `unfinished_archive`: a session folder whose story exists and that still holds files. The fix SHALL remove each file whose bytes are committed at HEAD (REQ-records-012) and that the story holds: the session record, and an evidence entry with its log and screenshot only when one of the story's receipts holds its ID. An evidence entry that no receipt holds, such as one a branch added to a session that main archived, SHALL be kept with its log and screenshot, and the finding SHALL be `skipped`, naming each kept entry and that the story does not hold it, so no evidence is lost (#381). macOS metadata in the folder (REQ-cli-068), committed or not, SHALL be removed with it and never block the fix (#377). The fix SHALL report the other files it keeps as `skipped`, naming the commit to make.
- `stale_gates`: an open session whose native change is archived on HEAD and that has open awaiting-human gates. This fix is a decision. With `--reason`, it SHALL withdraw those gates as `chough checkpoint --clear-awaiting-human` does, recording no human decision, SHALL write that reason, in the person's words and checked as REQ-records-006 checks any written value, as the `withdrawn` field of each withdrawn gate's `gate-closed` entry, so the archived story says why each gate was withdrawn, and SHALL then archive the session as the `finished_session` fix does. A decision fix that was applied SHALL echo the reason it wrote in its finding as `decision_reason`.
- `unfinished_session`: an open, unfinished session named with `--close <id>`. This fix is a decision. With `--reason`, it SHALL close the session as `chough session close <id> --reason` does.
- `timeline_rewritten`: a session or story whose Timeline at HEAD is not an exact prefix of its working Timeline (REQ-history-002). It has no fix; the finding names the entries that differ. An entry that differs from HEAD only where a leak was replaced by `<redacted:<kind>>`, as `chough doctor --fix --redact` writes, SHALL NOT count as rewritten, since restoring it would restore the leak: committing the redaction is the way on (#374).
- `alias_conflict`: an ID that is the `id` or alias of two records (REQ-records-003). It has no fix; the finding names both records.
- `oversize`: a committed record, evidence entry or screenshot over its bound (REQ-records-001, REQ-records-005, REQ-records-013). It has no fix; the finding names the file, its size and the bound, and the verify lane's `leaks` step fails on it as it does on `leak`, so CI refuses an oversized receipt.
- `unreadable`: a committed Chough record under `.chough/` that its reader refuses, such as front matter outside the subset of REQ-records-001, which makes `chough status` and `chough next` refuse the project. It has no fix; the finding names the file and the reader's reason with its line, and names `chough session shelve <id>` for an open session's `session.md`, except that a record whose front matter holds a Git conflict marker SHALL name resolving the conflict instead, as its `conflict_markers` finding does (REQ-records-001, #374). A record over its bound is `oversize` instead.
- `records_in_verification`: `.specsync/sdd.json` exists and its `verification_ignored_paths` lacks one of Chough's record folders (REQ-records-014). It has no fix, because Chough never writes SpecSync's policy; the finding names the entries to add.
- `merge_drivers`: in a Git work tree, a clone whose Git config has no command for `merge.chough-bug.driver` or `merge.chough-generated.driver` (a fresh clone has neither), or whose `.gitattributes` lacks one of the merge lines of REQ-cli-047. It is one finding, naming the missing drivers and lines. The fix SHALL set the missing drivers in the clone's local Git config and SHALL NOT write `.gitattributes`, a committed file a person reviews. A missing line SHALL leave the finding `skipped`, naming the lines to add and `chough init`, which appends exactly those.
- `merge_driver_outdated`: the merge drivers are set, and the `chough` they run, found through `PATH` as Git finds it, is missing or reports a version older than the running Chough, so a merge would run older code on these records. It has no fix; the finding names both versions and the install command from the README's toolchain section.

The fixes SHALL run in that order. They SHALL never replace a file, and SHALL leave the bytes of every session and story they do not fix unchanged. Trace: DRIVE-17, DRIVE-16, DRIVE-13, DRIVE-3, DRIVE-4, LIFE-26, LIFE-20.

Acceptance Criteria
- A finished session at stage intent and one at done are archived, a stale pointer is removed, and open work stays open.
- An unmigrated tree reports `unmigrated` naming `chough migrate`, and `--fix` changes nothing under `.chough/`.
- A committed home path reports `leak` without the path; `--fix --redact` replaces it with `<redacted:home-path>` and the next run finds nothing.
- A finished session folder holding an uncommitted file is `unfinished_archive`; `--fix` keeps that file, and after a commit removes the folder.
- A hand-edited earlier timeline entry reports `timeline_rewritten`, and an alias held by two bugs reports `alias_conflict`, each with no fix applied.
- Without `--reason`, a stale gate and a session named by `--close` are skipped with their files unchanged; with `--reason` the gate is withdrawn with no human decision recorded and the session is archived, and the named session is closed with the reason.
- A committed evidence entry whose front matter holds a bare `- -c` block item reports `unreadable` naming the file and the line, and `--fix` leaves it byte-identical and the finding `skipped`.
- A clone with neither merge driver set reports one `merge_drivers` finding naming `chough doctor --fix`, and `--fix` sets both. A `.gitattributes` missing a merge line is reported naming that line, and `--fix` leaves `.gitattributes` byte-identical and the finding `skipped`.
- With both merge drivers set and a `chough` first on `PATH` that reports an older version, `chough doctor` reports one `merge_driver_outdated` finding naming both versions and exits 1; with the same version, or with no driver set, it reports none.
- A committed session record whose `cases` reference holds a token, and whose Goal holds an email, gives a `leak` finding for each line; the one for the ID names `chough session shelve <id>`. `--fix --redact` redacts the email, leaves the ID line byte-identical with its finding `skipped`, and the session still reads.
- After a merge that kept main's archive of a session to which a branch added an evidence entry, `--fix` removes the folder's other committed files and keeps that entry, its finding `skipped` naming it, so the entry still resolves (#381).
- A `.DS_Store` or `._session.md` in an archived session's folder does not block `--fix`, which removes the folder (#377).
- After `--fix --redact` redacts a leak in a committed Timeline entry, the next run reports no `timeline_rewritten`; a hand edit of the same entry still does (#374).
- A committed session whose front matter holds a conflict reports `conflict_markers` naming each marker and its line, and its `unreadable` finding names resolving the conflict rather than `chough session shelve` (#374).

## REMOVED

(none)
