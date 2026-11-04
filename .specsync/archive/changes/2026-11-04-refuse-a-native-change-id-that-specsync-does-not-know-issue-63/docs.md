---
change: refuse-a-native-change-id-that-specsync-does-not-know-issue-63
artifact: docs
---

# Docs

- docs/cli.md, `checkpoint` row: with SpecSync configured, `--native-change <id>`
  refuses as `refused:native-change-unknown` and saves nothing from the call unless
  `specsync change status <id> --json` succeeds with JSON. Without a SpecSync
  configuration only the ID format is checked.
- CHANGELOG.md: one Unreleased entry citing #63.
