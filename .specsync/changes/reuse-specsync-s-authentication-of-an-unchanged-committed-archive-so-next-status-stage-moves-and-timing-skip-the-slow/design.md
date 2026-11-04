---
change: reuse-specsync-s-authentication-of-an-unchanged-committed-archive-so-next-status-stage-moves-and-timing-skip-the-slow
artifact: design
---

# Design

New module `src/native.rs` with `change_status(root, id, cap)` used by every status read that feeds a gate or report (main.rs next and status, stage.rs moves, timing.rs). Retention (`evidence.rs` validity and manifest) stays live because it exists to notice validity changes.

1. If `.specsync/changes/<id>` exists, or the ID is not plain, read live.
2. Find exactly one directory under `.specsync/archive/changes/` named `<id>` or `YYYY-MM-DD-<id>`, holding `accepted-state.json` and a `verification.json` with an acceptance manifest; otherwise read live.
3. Digest: SHA-256 over a domain tag, the resolved SpecSync binary path, size and modification time, then each input's relative path, length and bytes: every file in the archive in sorted order, the legacy baseline when present, `.specsync/sdd.json` and the native config. The key also holds `git rev-parse HEAD:<archive>` and the last commit touching the archive, because SpecSync 6.1.2 authenticates the accepted transition against reachable history (`authenticated_accepted_transition_for`). Reuse requires `git status --porcelain` to be empty for the archive and the recorded commit to be an ancestor of HEAD. A symlink, a Git failure or more than 64 MiB reads live.
4. Store: `<git common dir>/chough/archive-auth/<sha256 of id>.json`, found from `.git` without running Git. The entry holds schema_version, change_id, digest, specsync binary, authenticated_at and the response. Written atomically after a live read that proves archived plus authenticated-history.
5. Hit: return the stored response with section state `cached` and source naming the proof. stage.rs maps an archived authenticated response from a cached read to the label `native:archived-authenticated-cached`, detail naming the binary, time and digest.
6. Batched advance: `stage advance --to <stage>` reads native state once, plans every step from the current main stage to the target, checks each step's gate and acceptance prerequisite on that one read, then records the transitions; any refusal records nothing.
