---
change: refuse-a-native-change-id-that-specsync-does-not-know-issue-63
artifact: requirements
---

# Requirements

MODIFIED REQ-cli-007 (see `deltas/cli.md`). The canonical checkpoint requirement now
says that, with SpecSync configured, `checkpoint --native-change <id>` reads
`specsync change status <id>` before applying any update and refuses as
`refused:native-change-unknown` when SpecSync does not report the ID, or when the read
fails or times out, leaving `.chough/session.json` byte-identical. A reported ID saves
as before, and without SpecSync only the format check applies. REQ-cli-005 is
unchanged: it covers observation criterion IDs, requirement IDs and contract paths,
not the native change ID.
