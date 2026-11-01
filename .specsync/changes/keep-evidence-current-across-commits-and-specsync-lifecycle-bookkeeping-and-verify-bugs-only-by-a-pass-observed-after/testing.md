---
change: keep-evidence-current-across-commits-and-specsync-lifecycle-bookkeeping-and-verify-bugs-only-by-a-pass-observed-after
artifact: testing
---

# Testing

## Requirement evidence

| Requirement | Test |
|---|---|
| REQ-cli-004 | `tests/cli.rs`, in a temporary Git repository with a local identity. A new observation stays current after committing only `.chough/`, after an empty commit, after committing an edit that was already present, and after committing a deletion that was already present. It goes stale after a tracked edit, a new untracked file, an exec-bit change, and edits to `specs/`, `.specsync/config.toml`, `.specsync/changes/<id>/deltas/*.md`, `requirements.md` and `testing.md`. It stays current after writes to each bookkeeping file and after a `state:` change in `change.md`, and goes stale after any other `change.md` edit. Moving a stub change directory to `.specsync/archive/changes/<date>-<id>/` keeps it current, and an artifact edit after that move stales it. |
| REQ-cli-004, REQ-cli-017 | A legacy record without `content` stays stale after an empty commit. A record with a malformed `content` is refused. `revision` is still recorded and shown. |
| REQ-cli-012 | `tests/bugs.rs`: pass then fail gives `reproduced` and refuses `verified`. Fail then a later pass gives `reproduced-and-later-passed` and allows `verified`. A pass in the same second gives `reproduced`. A regression-baseline ancestry check still uses the revision. |
| REQ-cli-016 | The existing skill status test is extended: an unedited 2756dc2 accept.md or lifecycle.md is known-outdated and is updated. |

## Cases and bugs

New `.chough/cases/cli.cases.toml` entries cover BUG-0014, BUG-0015 and BUG-0016, and each has a failing observation before the fix and a passing one after it. Their requirement links validate only after REQ-cli-004 is materialized, so the scoped check runs first.

## End to end

In a disposable copy of examples/shipping, built with examples/feature-bug-loop.sh, with a local Git identity and `GIT_CONFIG_GLOBAL=/dev/null`, follow the documented order: check, cases run, gate, commit with `.chough/`, check --commit, review (SIMULATED label, not a human decision), finalize. `chough next` reports the observations as `current-observation` after every step, and SpecSync needs no further check. This is recorded as an agent-assisted observation with its limits.

## Regression

The full `cargo test`, `fledge lanes run verify` and `fledge lanes run pre-push`, and hosted CI on three platforms.
