---
change: read-native-specsync-spec-files-under-specs-at-any-size-chough-s-65536-byte-structured-record-bound-no-longer-applies
artifact: docs
---

# Docs

- docs/cli.md "Records and limits": native spec files under `specs/` are read at any
  size; size is a composability hint, never a gate. The `next` lessons table drops
  the `larger than 4194304 bytes, not searched` note.
- skills/chough-workflow/references/build.md (and `.agents/skills` copy): "Keep it
  composable".
- CHANGELOG.md Unreleased entry.
