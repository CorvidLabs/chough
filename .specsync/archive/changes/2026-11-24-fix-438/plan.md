---
change: fix-438
artifact: plan
---

# Plan

## Approach

Make the close sequence validate-then-write:

1. Add a hidden `--dry-run` flag to `chough checkpoint --close-gate`
   (requires `--close-gate`, conflicts with nothing else): it runs every
   check the close runs — the relay checks, the native-step decision,
   rendering the record and its SESSION_LIMIT size check — but writes
   nothing and runs no native step, and reports the projected record
   size. A dry run of a gate that drives a native step validates the
   close but skips the step itself.
2. In src/gates.rs `gate record`, after the existing pre-checks, run a
   dry-run close for every named gate first (pass 1). If any refuses,
   refuse the whole record with nothing written, naming the gate and the
   refusal. Then run the real closes (pass 2) as today. The native step
   still runs exactly once, in pass 2, before any gate is closed (the
   sort already puts the driving gate first).
3. Tests: a regression test that pads a session record near the
   SESSION_LIMIT so the first named gate's close fits but the cumulative
   second exceeds the bound; `gate record --gates 1,2` must refuse with
   nothing written (today it closes gate 1 and refuses gate 2). Plus a
   test that the dry-run flag writes nothing and runs no native step.

## Out of scope

The other gate record refusals (already all-or-none), the close command's
public surface beyond the hidden flag, and any contract text.

## Steps

As above, then the gate/checkpoint tests, fmt, lint, the change's check
lane.

## Risks

The dry run renders the record; a leak-guard or redaction difference
between dry run and write could refuse a dry run that the write would
accept — the dry run reuses the same code path, so the risk is small.
The native step is skipped in the dry run, so a native-step failure
still surfaces in pass 2 before any close (the driving gate sorts
first).

## Constraints consulted

REQ-cli-061 (all or none; native step runs before any gate is closed),
REQ-cli-050, the existing close-gate checks in src/checkpoint.rs.