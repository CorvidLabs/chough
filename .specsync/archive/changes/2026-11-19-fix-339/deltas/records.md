# records delta

## ADDED

(none)

## MODIFIED

### REQUIREMENT REQ-records-001

Record files. Every record Chough writes, a session, an archived session story, an evidence entry, a bug, an inbox entry, a journal post and a migration receipt, SHALL be one UTF-8 Markdown file that starts with YAML front matter between two `---` lines, and that file SHALL be the record's only source of truth: no JSON or other twin SHALL be written beside it, and no index, cache or generated note SHALL be read in its place. The front matter SHALL carry the IDs and fields agents parse; the body SHALL carry the record for a human reader. Front matter SHALL use a strict subset of YAML that Chough reads and writes without a new dependency: `key: value` scalars, plain or double-quoted with `\"`, `\\` and `\n` escapes; flow lists `[a, b]`; block lists of scalars; and block lists of flat maps of scalars. Chough SHALL refuse front matter outside that subset, a duplicate key or a missing closing `---`, naming the file, the line and the rule, and SHALL write nothing in that command. A record's body SHALL NOT hold a Git conflict marker: a line that starts with `<<<<<<<` or `>>>>>>>`, or a line of `=======` between such lines (a `=======` line outside them, as a Markdown heading underline, is not one); a line inside a closed fenced code block, as CommonMark reads one, quoting an example or tool output, is not one, but a fence that never closes hides nothing after it, and no fence hides a conflict in a section Chough writes itself, a session's `## Timeline` or an archived story's `## Receipts` (#339). A real conflict Git leaves inside such a fence cannot be told from a quoted one, so no write SHALL be refused for it, and `chough doctor` SHALL only warn of each marker line there (REQ-cli-055). A lone `=======` outside a conflict may be left by a half-resolved conflict, but a heading underline may be followed directly by text, so no write SHALL be refused for it, and `chough doctor` SHALL only warn of it (REQ-cli-055). Chough SHALL refuse a write whose body would hold one as `refused:conflict-markers`, naming the file, the line and the marker, with every file unchanged, and `chough doctor` SHALL report each committed record whose body holds one as a `conflict_markers` finding naming the file, the line and the marker, which has no fix. Reading SHALL be tolerant: an unknown front matter key SHALL be kept byte for byte, in place, when Chough rewrites the front matter, and SHALL be ignored otherwise; a missing optional key SHALL take its documented default; CRLF line endings SHALL be read as LF. Chough SHALL own only the front matter and the sections it appends to, and SHALL keep every other body byte a person or agent wrote: a write SHALL replace the front matter, append list items to the end of a named section, or create a missing named section at the end of the body, and SHALL do nothing else to an existing body. Files SHALL be written with LF line endings by an atomic replace under the writer lock of REQ-cli-040, or by create-new for a new record. A record SHALL be at most 65,536 bytes, except a session record, an archived session story and a migration receipt, which SHALL be at most 1,048,576, 1,048,576 and 4,194,304 bytes; a record over its bound SHALL be refused with the bound named and every file byte-identical. When a 0.5.x record would migrate into a record that holds a marker, the preview SHALL say that the apply refuses and SHALL NOT offer the apply command, and `chough migrate --apply` SHALL refuse with nothing written, each naming the 0.5.x file to resolve, the record it would become, the line and the marker. Trace: DRIVE-25, DRIVE-26, DRIVE-28, KNOW-32, LIFE-3.

Acceptance Criteria
- Every record kind written by a fixture cycle (session, archived story, evidence, bug, inbox entry, journal post, migration receipt) starts with front matter that the reader parses, and no `.json` file is written under `.chough/` except the local caches of REQ-history-007 and `.chough/preferences.json`.
- A person's edit to a session's Goal paragraph, to a bug's Steps and to an evidence entry's body survives a following stage move, decision, evidence add and bug update byte for byte.
- An unknown front matter key added by hand is kept byte for byte after Chough rewrites that front matter, and a missing optional key reads as its default.
- Front matter with a nested map, an anchor, a duplicate key or no closing line is refused with the file, line and rule named, and nothing is written.
- A CRLF copy of a record reads the same as the LF original.
- A record one byte over its bound is refused naming the bound, with every file unchanged.
- A record write whose body holds a `<<<<<<<`, `=======` or `>>>>>>>` conflict line is refused as `refused:conflict-markers` naming the file, the line and the marker, with every file unchanged; a body with a `=======` heading underline and no conflict is written.
- With a committed record whose body holds a conflict, `chough doctor` reports a `conflict_markers` finding for each marker line naming the file, the line and the marker, exits 1, and reports none for a tracked file that is not a record.
- A conflict quoted inside a fenced code block, with backtick or tilde fences, is not a marker and the record saves; a lone `=======` with text after it is not refused either.
- A conflict after a code fence that never closes, or after a backtick line whose tag holds a backtick, is a marker and the write is refused.
- A conflict inside an archived story's `## Timeline` or fenced `## Receipts` block is a marker, inside a fence or not.
- A Git conflict left inside a code fence of a session's Goal is not refused on the next write.
- A 0.5.x inbox entry whose text holds a conflict makes the migration preview say the apply refuses, without offering the apply command, and the apply refuse, each naming that 0.5.x file first.

## REMOVED

(none)
