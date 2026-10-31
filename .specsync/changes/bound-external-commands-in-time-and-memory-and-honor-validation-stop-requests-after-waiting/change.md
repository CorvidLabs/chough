---
id: bound-external-commands-in-time-and-memory-and-honor-validation-stop-requests-after-waiting
state: implementing
type: bug_fix
base_commit: 31d5105343a46728551648375bf9a33e213a3ce3
---

# Bound external commands in time and memory and honor validation stop requests after waiting

## Intent

Bound external commands in time and memory and honor validation stop requests after waiting

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- Every external command Chough runs (case bindings, native status and other native reads) is read through one bounded runner: each stream retains at most its cap while reading and keeps draining beyond it, both streams are read concurrently, and the run ends by its deadline. On a deadline, or when output is still held open after the command exits, the command and its descendants are stopped (Unix: its process group; Windows: its process tree), and any reader still blocked is abandoned rather than awaited (BUG-0010, BUG-0011, REQ-cli-011, REQ-cli-006).
- A case run with a 1 s deadline whose descendant holds the pipes returns within a few seconds as failed with a timeout limitation; a command that exits while a descendant holds the pipes returns by its deadline with a limitation naming the stopped descendants; output beyond 1 MiB per stream is counted, not retained, and the artifact notes the discarded byte count (BUG-0010).
- Native reads keep the 16 KiB display bound and its truncation note, parse complete stdout up to 4 MiB, retain at most 16 KiB of stderr, and report a response over the bound, a failure, an undecodable or unavailable command, or a deadline (default 120 s, CHOUGH_NATIVE_TIMEOUT_SECONDS 1 to 3600) explicitly; gated moves refuse each of them as native-unknown, and status and next show the state and next action (BUG-0011, REQ-cli-003).
- A repeated validation run checks for a stop request and a paused checkpoint immediately before every attempt, including after its interval, and waits in slices so a stop during a long interval ends the run promptly with no further attempt (BUG-0009, REQ-cli-011).
- Existing behavior these rules do not name is unchanged; the full suite, the feature-and-bug exercise and native structural checks pass on Ubuntu, macOS and Windows CI; regression tests cover descendants, deadlines and output bounds per platform and the stop-during-interval case, and BUG-0009 to BUG-0011 are verified by passing observations.

## No-spec Rationale

Not applicable
