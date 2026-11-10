---
change: know-29-roll-up-copies-never-moves-tracked-session-records-165
artifact: testing
---

# Testing

| Requirement | Coverage |
|---|---|
| REQ-history-011 | Updated roll-up tests in tests/reduce.rs: rolled sessions stay in `.chough/sessions/archive/` byte-identical and the vault holds copies; anchor-holder and librarian-off behavior unchanged |

tests/reduce.rs (43), tests/sessions.rs (30) and tests/evidence.rs (36) pass with the change.
