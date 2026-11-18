# records delta

## ADDED

### REQUIREMENT REQ-records-015

Migration skips macOS metadata files. `chough migrate`, its preview and `--apply` alike, SHALL skip a `.DS_Store` file and a file or folder whose name starts with `._` among a project's 0.5.x records, in `.chough/` and in the knowledge directory alike (REQ-cli-068): it SHALL neither read, carry, list nor remove such a file, so the file never stops the preview or the apply and the plan and its digest are the same as without it. A folder the migration empties of its 0.5.x records SHALL stay while such a file is in it, since macOS removes orphaned `._*` files with a folder it removes. Trace: DRIVE-14, DRIVE-27, LIFE-20.

Acceptance Criteria
- A 0.5.x project with AppleDouble `._` files and `.DS_Store` files beside its bugs, sessions, archived sessions, inbox entries, evidence, history events and knowledge notes previews with the same plan digest as without them, the apply of that digest succeeds, and every one of those files is still there after it.

## MODIFIED

(none)

## REMOVED

(none)
