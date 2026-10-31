# Release candidate demonstrations (2026-10-31)

Run against `0xleif/feat/first-release-candidate` at `b07f849` on macOS arm64
(SpecSync 6.1.1), each in disposable copies and each re-run by an independent
verifier. Approvals and consents inside the copies were SYNTHETIC.

| Demonstration | Result |
|---|---|
| Lifecycle on this change | BUG-0012 failed first (`d761ff2`: 0 of 8 retention tests), fixed and verified; BUG-0013 (long native change IDs) found, fixed in `3b0b2f7` and verified; TC-cli-0050 to 0066 passed; the checkpoint reached accept through `native:verifying` gates. Human review and archive follow. |
| Retention | 99 observation sets after repeated runs; preview changed no file; 6 eligible (10,644 bytes) after synthetic 30-day aging, 93 protected with reasons (checkpoint, bugs, newest 3, validation boundaries, native archive manifest); `--apply` refused until a run consent, then removed exactly 6 sets with 6 ledger lines; pruned references labelled `pruned`, a hand-deleted one `unavailable`, none counted as passed; validation record byte-identical; all archives still `authenticated-history`; with SpecSync unavailable every set stayed protected. |
| Fresh-context resumption | A new agent with no conversation resumed from files alone and reported the objective, stage, native state and next action, the pending human gate, open and verified bugs and evidence labels correctly. |
| Install | `cargo package` holds 42 files (source, canonical skill, Cargo files, README, LICENSE); the packaged crate builds and installs; `chough init` in a new project creates only the skill and an empty checkpoint. |

Found along the way: BUG-0014 (bug verification does not check that the pass came
after the failure), recorded for a later change. In a repository without commits
`chough next` shows the raw `git rev-parse` error as the snapshot.
