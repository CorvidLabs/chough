---
change: host-matrix-adapters-for-claude-code-and-codex-each-run-once-for-real-on-macos-and-recorded
artifact: tasks
---

# Tasks

- [ ] Red: add the usage-limit case to selftest.sh and TC-cli-host-matrix-unavailable-turn, and record the failing run.
- [ ] Teach run.sh exit 75, add integrations/claude-code/adapter.sh and integrations/codex/adapter.sh, and the prompt line.
- [ ] Run Claude Code once for real and record TC-cli-host-matrix-claude-code.
- [ ] Run Codex once for real and record TC-cli-host-matrix-codex.
- [ ] Update tools/host-matrix/README.md, docs/integrations.md and CHANGELOG.md.

After verification (plain text, not tasks): change audit, the full suite at the final
head, the review gate, ship, push, retarget to main after #254 merges, and hosted CI.
