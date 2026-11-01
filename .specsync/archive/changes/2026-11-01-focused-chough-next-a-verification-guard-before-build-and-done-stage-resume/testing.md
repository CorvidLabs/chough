---
change: focused-chough-next-a-verification-guard-before-build-and-done-stage-resume
artifact: testing
---

# Testing

## Requirement evidence

| Requirement | Test |
|---|---|
| REQ-cli-021 | `tests/cli.rs`: next text and `--json` forms carry the listed fields and no `native_specsync`, `observations` or `snapshot`; the cap fixture (500 referenced stale observations, 40 gates and 40 blockers of 4 KiB, a native response near the parse bound) keeps each form at most 16384 bytes with totals and `truncated`; strings cut at 512 bytes with ` [cut]`; a counting `specsync` stub records one call for `next` and one for `status` at plan, build and accept; `next` and `status` agree on `next_valid_action` across the fixtures |
| REQ-cli-021 | `tests/stages.rs`: done with an archived authenticated stub recommends reset with `--reason` and `specsync change new` and shows the native next action; the suggested reset with the placeholder filled succeeds; afterwards `next_action` is the new-cycle text; `back` leaves it unchanged; a non-authenticating archive keeps its refusal label; no native change and SDD disabled recommend the reset |
| REQ-cli-022 | `tests/stages.rs`: digests for the 7 archived approvals of this repository (fixture copies of their deltas and ledgers) match; plan to build, build to test and test to accept refuse with `refused:native-delta-changed` for an edited delta, an added module and a deleted delta, and with `refused:native-delta-unverifiable` for a missing ledger, a ledger over 4 MiB, malformed JSON, no definition approval, an approval without digests and status without affected_specs; each refused move leaves session.json and transitions.jsonl byte-identical; a CRLF-only edit, an unchanged approval and a no_spec_change approval pass; back and reset to build use the same guard |
| REQ-cli-016, REQ-cli-019 | `tests/package.rs`: the origin/main SKILL.md and references/checkpoint.md read known-outdated and update; an edited copy stays edited; no bundle file contains U+2013 or U+2014 |

Existing tests that read full-report fields through `next --json` (tests/cli.rs,
tests/bugs.rs, tests/stages.rs, tests/timing.rs, tests/evidence.rs) read them through
`status`, and keep asserting the decision fields through `next --json`.

## Cases

Reserved: TC-cli-0120 (next cap), TC-cli-0121 (done-stage resume), TC-cli-0122 (reset
next_action), TC-cli-0123 (changed delta refused), TC-cli-0124 (unverifiable delta
refused). They link REQ-cli-021 or REQ-cli-022, so they are added to
.chough/cases/cli.cases.toml and run only after the scoped `change check` materializes
those IDs.

## Real behavior

On this repository, built binary: `chough next` and `chough next --json` byte counts
and wall time against the measured 410,822 bytes and 15 seconds; the done-stage action
names the reset. In a disposable copy of examples/shipping: approve with a SIMULATED
actor, append a requirement to the delta, and `chough stage advance` from plan refuses
with `refused:native-delta-changed` while `specsync change status` still reports
approval_valid true.

Regression: full `cargo test`, `fledge lanes run verify` (including the
feature-and-bug exercise) and native structural checks on all three CI platforms.
