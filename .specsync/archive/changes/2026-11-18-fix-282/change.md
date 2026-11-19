---
id: fix-282
state: archived
type: bug_fix
base_commit: 66efea4dd777077ef9bbd412cf0fac8f7ea93b41
---

# Fix 282

## Intent

fix 282

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- chough checkpoint --pr 150 and --pr with a pull request URL each set the session's pr field, which another process reads back; an empty value clears it; a value with a line break, or one holding a local path or a token, is refused with the session file byte-identical, or redacted under --redact, as the pr field's existing guard does. Setting it adds no Timeline entry, as --objective and --title do not.

## No-spec Rationale

Not applicable
