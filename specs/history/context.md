# History context

## Decisions

2026-11-03: Split from the cli module so each requirements companion stays within
Chough's 65,536-byte structured record bound, on the parent agent's instruction at
9:46 AM MT (split rather than raise the limit, keep wording, weaken nothing).
SpecSync 6.1.2 requires `REQ-<module>-<number>` IDs in a module delta and keeps
removed IDs as permanent tombstones, so the moved requirements were renamed:

| Old ID | New ID | Subject |
|---|---|---|
| REQ-cli-018 | REQ-history-001 | Reference-aware retention |
| REQ-cli-025 | REQ-history-002 | Immutable transition history |
| REQ-cli-026 | REQ-history-003 | Bounded history lifecycle |
| REQ-cli-027 | REQ-history-004 | Conservative evidence cleanup |

The bodies are byte-identical to main 7bd1b62; tests/spec_split.rs pins their
SHA-256 digests. Archived native changes, observations, bug records and decisions
that cite the old IDs are historical records and are not rewritten. Earlier
decisions about retention and history are in the cli context and docs/DECISIONS.md
(D-017 and later).

## Working files

- src/history.rs: transition events, cursor, adoption and history prune.
- tests/history.rs and tests/evidence.rs: history and retention regressions.
- Evidence retention itself is implemented in src/evidence.rs, owned by cli.

## Lessons

No module lessons recorded yet; earlier retention and history lessons are in the
Lessons section of specs/cli/context.md.
