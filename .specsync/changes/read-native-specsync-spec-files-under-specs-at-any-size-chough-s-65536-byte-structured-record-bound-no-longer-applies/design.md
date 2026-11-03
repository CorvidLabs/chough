---
change: read-native-specsync-spec-files-under-specs-at-any-size-chough-s-65536-byte-structured-record-bound-no-longer-applies
artifact: design
---

# Design

No visual or interaction design. One reader boundary: `records::read_spec_file` for
native spec files under `specs/`, `records::read_record` and its bounds for
everything Chough writes.
