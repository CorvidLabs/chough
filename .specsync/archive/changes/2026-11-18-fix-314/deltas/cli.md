# cli delta

## ADDED

### REQUIREMENT REQ-cli-068

macOS metadata files. In `.chough/`, every reader SHALL skip a file named `.DS_Store` and a file or folder whose name starts with `._`, the metadata macOS writes beside files: Finder writes `.DS_Store` when someone opens a folder, and macOS writes an AppleDouble `._<name>` file beside every file on exFAT and other non-Apple volumes. Such a file SHALL never be read, refused or reported as a record, an evidence entry, a case definition or a session, so it never stops a command, and `chough doctor` SHALL give it no `unreadable` finding; `chough migrate` skips it among 0.5.x records as REQ-records-015 says. The leak scan of REQ-records-006 SHALL still cover such a file when Git tracks it, reading its bytes as text with each invalid UTF-8 sequence replaced, since an AppleDouble file is binary. `chough doctor --fix --redact` SHALL leave such a file that is not valid UTF-8 byte-identical, with its leak finding `skipped` and naming the file for a person to remove, since rewriting decoded text would corrupt it. A file with any other name keeps the rules of REQ-records-001, so a malformed record is still refused, naming the file. Trace: DRIVE-14, DRIVE-25, LIFE-3, LIFE-26.

Acceptance Criteria
- With an AppleDouble `._<name>.md` file in `.chough/bugs/`, in a session's `evidence/` and in `.chough/archive/sessions/`, `chough next` and `chough status` exit 0, as they do without it.
- With a `.DS_Store` file and an AppleDouble `._<name>.cases.toml` file in `.chough/cases/`, `chough cases check` passes.
- A committed `._<name>.md` file in `.chough/bugs/` gives no `unreadable` finding from `chough doctor`, while a committed malformed record with an ordinary name still gives one.
- A committed binary AppleDouble `._<name>.md` file whose bytes hold an email gives a `leak` finding from `chough doctor --leaks`, and `chough doctor --fix --redact` leaves it byte-identical with that finding `skipped`.

## MODIFIED

(none)

## REMOVED

(none)
