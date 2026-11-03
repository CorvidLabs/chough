---
change: read-native-specsync-spec-files-under-specs-at-any-size-chough-s-65536-byte-structured-record-bound-no-longer-applies
artifact: docs
---

# Docs

- docs/cli.md "Records and limits": native spec files under `specs/` are read at any
  size; size is a composability hint, never a gate. It names the `next` Lessons
  search and `library snapshot` 4 MiB limits as known exceptions (REQ-cli-034); the
  `next` lessons row and the snapshot row keep their 4 MiB notes until the later change.
- skills/chough-workflow/references/build.md (and `.agents/skills` copy): "Keep it
  composable".
- CHANGELOG.md Unreleased entry.
