---
change: change-leaks
artifact: requirements
---

# Requirements

- REQ-cli-057: a description, `--rationale` and answer are leak-checked before SpecSync runs, refused as `refused:leak` or redacted under `--redact` with a count line; a control character is refused under `--redact` too (#412).
- REQ-records-006: doctor scans `.specsync/changes/` too, `--fix --redact` skips a finding there naming the way out, and `.specsync/archive/` is not scanned; the echo sweep covers `change new`'s description, `change answer`'s answer and `change ship --close-gate` (#367, #412).
