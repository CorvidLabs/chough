---
change: fix-435
artifact: tasks
---

# Tasks

- [ ] Rewrite publish_entry (src/sessions.rs) to write whole or not at all via records::write_new_bytes, no hard link.
- [ ] Extend the ignored exFAT test in tests/no_hard_links.rs with inbox add.
- [ ] Add changelog.d/2026-11-23-fix-435.md fragment.
- [ ] Run session/inbox tests, fmt, lint, and the change check lane.