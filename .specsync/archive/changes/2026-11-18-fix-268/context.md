---
change: fix-268
artifact: context
---

# Context

GitHub #268, from the 0.6.0 swarm report (#273): `chough checkpoint --file "src/ghp_<token>.txt"` exits 0 and commits the token-bearing path to `session.md`'s `files:` list. `chough doctor --leaks` finds it only after the commit. `chough evidence add --artifact` already refuses the same value at the write.

REQ-records-006 requires Chough to check every value it writes, front matter included, before writing a committed record. On main at be6c9f1, `guard_leaks` in `src/checkpoint.rs` checks the objective, intent scope, next action, the story lists, consents, aliases, `pr`, title, summary and pending Timeline values, but not the `files` and `design` references, which hold free-form paths. The criterion, requirement, case, observation and bug references are IDs that `validate_v2` already limits to an ID form, so they cannot hold a path, an email or a token.

This change adds the `files` and `design` references to the guard. It changes no requirement text, command or flag. Leif approved the scope on 2026-11-18, as relayed by his planning session (hc t4c).
