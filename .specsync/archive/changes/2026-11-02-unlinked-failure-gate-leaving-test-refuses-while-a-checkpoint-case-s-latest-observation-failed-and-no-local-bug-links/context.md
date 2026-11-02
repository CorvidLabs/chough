---
change: unlinked-failure-gate-leaving-test-refuses-while-a-checkpoint-case-s-latest-observation-failed-and-no-local-bug-links
artifact: context
---

# Context

Issue #39 (CorvidLabs/chough), filed by corvid-agent from a Linux dogfood run of a
small sample project. In stage test, `chough cases run` recorded a failed
observation; no bug was written (the `bug add --title "--chars ..."` call failed in
argument parsing because the value began with `--`); `chough stage advance` then
applied test -> accept. The only gate on leaving test is `refused:blocking-bugs`, so
a failure that never became a bug is left behind, although FLOW-7 says a failing
case stays failed and LIFE-5 connects bugs to verification evidence.

Reproduced on Linux (x86_64) at main 825a380 with SpecSync 6.1.2, Fledge 1.8.0 and
hi 0.8.0 in a disposable copy of `examples/shipping` (SDD off): a failed,
unlinked TC-shipping-0002 observation, then `stage advance` from test exits 0 to
accept, and `chough next` says "Continue test" with no warning. Recorded as
BUG-leaving-test-ignores-a-referenced-case-s-failed-unlinked-observation-ea76be10351c67bc019ac939 (in-scope product defect) linking
OBS-product-test-9e2d612534e636aa16dbc628 and the log
`.chough/artifacts/bug39-repro.log`.

Proposed answers to the issue's open questions (posted on #39 as proposals; they
become binding only through native scope approval of this definition):

- Yes, leaving test reads the latest observation of each case referenced in the
  checkpoint and refuses when it failed and no local bug links it. Refuse rather
  than warn, because the missed warning is the reported problem.
- A linked bug of any status satisfies the gate; open blocking bugs keep their own
  refusal. Stale but not failed observations do not trigger it; they stay under
  "Observations needing attention".
- `chough next` and `chough status` name the same refusal at test from the same
  rule; no new list is added to the bounded next view.

Ruled out: warning only (the failure is the one the next agent misses); gating on
every failed observation in the repository (old, unreferenced failures would block
unrelated work); counting stale observations (a separate decision).

Not in this change: the `bug add` value-starting-with-`--` parse (clap behavior;
`--title=--chars ...` works). Approvals: corvid-agent does not record approval for
a human; this definition waits for 0xLeif's native scope approval.

Overlap: PR #37 (0xLeif) edits `src/stage.rs` in `assess`, `log_transition`,
`log_gate`, `recent_transitions` and `move_forward`'s logging call; this change adds
a prerequisite check beside the existing blocking-bug check and one branch in
`assess`. PR #41 and issue #42 own no file this change edits. REQ-cli-025 to 029
are reserved elsewhere, so this change modifies REQ-cli-008 only.
