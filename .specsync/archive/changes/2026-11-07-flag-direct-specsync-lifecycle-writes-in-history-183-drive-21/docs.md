---
change: flag-direct-specsync-lifecycle-writes-in-history-183-drive-21
artifact: docs
---

# Docs

- `docs/cli.md`: a "Direct lifecycle writes" section (REQ-cli-060, REQ-history-013), and the `direct_writes` field in the status table.
- `skills/chough-workflow/references/lifecycle.md` (both copies): direct `specsync` lifecycle calls still work and are recorded as `direct-native-write` events.
- `CHANGELOG.md`: one Unreleased entry citing #183 and DRIVE-21. It says that Chough 0.4.0 and earlier builds refuse a history holding the event, and names the first release with it (planned 0.5.0) as the oldest reader.
