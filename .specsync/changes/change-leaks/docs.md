---
change: change-leaks
artifact: docs
---

# Docs

`docs/cli.md`: `chough change new` and `chough change answer` refuse a leaking description, rationale or answer, with `--redact` as the way out; `chough doctor`'s leak scan covers `.specsync/changes/` except the deltas, with `--fix --redact` skipping those findings and `.specsync/archive/` not scanned. A changelog fragment, `changelog.d/2026-11-20-change-leaks.md`.
