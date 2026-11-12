---
change: prepare-the-chough-0-6-0-release-cargo-0-6-0-and-the-dated-changelog-with-the-dogfood-results-and-the-fixes-the-dogfood
artifact: plan
---

# Plan

## Approach

Drive this change with the 0.6.0 build only, as the dogfood cycle the 0.6.0 plan
names: approval and review run as Leif through Chough's gates (#236 for real), Chough
records written after `change check --commit` leave verification current (the #12
path), and the move into done keeps every earlier story byte-identical (#165).

## Out of scope

Tagging, releasing or publishing 0.6.0, and merging #238: each waits for Leif's go.
One contract change, D2's REQ-cli-061 delta, which Leif chose to fix inside 0.6.0; any other the dogfood finds goes back to Leif as a send-back.

## Steps

1. D1: a red test (a migrated project whose local pointer names a 0.5.x key), then
   the fix in `src/sessions.rs` and `src/doctor.rs`; a bug record red then green.
2. Rename the test helper `session_id` in `tests/migrate_evidence.rs` to
   `session_folder`.
3. KNOW gaps: link KNOW-3, KNOW-6 and KNOW-24 from the active cases that test them,
   run the cases of KNOW-3, -4, -6, -15 and -24 with `chough cases run`, and recompute
   the gap list from `hi/knowledge.md` and the records until it is empty.
4. #148: run its regression test against the code before #157 (red), link it, run it
   on this tree (green), and mark the bug verified.
5. The four flaky-test bugs: reproduce each under load, fix and verify what has a
   cause, and leave open with its diagnostics, named to Leif, what never reproduces.
6. D2 (REQ-cli-061 delta): a red test (a second change on one branch counts the
   earlier change's files), then the base selection in `src/gates.rs`; #190's test
   stays green. docs/cli.md says the same, and its "planned after 0.6.0" note goes.
7. The migrate preview warning: a red test (a preview with an open native change gives
   no warning), then the warning in `src/migrate.rs`; the bug record red then green.
8. Cargo 0.6.0 (Cargo.toml and Cargo.lock), and the CHANGELOG's dated [0.6.0]
   section with the dogfood results in its upgrade notes.
9. `change check --commit`, Leif's review as him through `chough change ship
   --close-gate`, and the move into done with its two-step closeout.

## Risks

- A flaky test may not reproduce: it stays open, named to Leif, and blocks the tag,
  not this change.
- #148's red needs the test applied to the code before #157; if that code cannot run
  the test, the record says so and the bug goes to Leif.

## Constraints consulted

AGENTS.md (SpecSync owns the lifecycle, Chough only its records); the cli lessons (a
delta is frozen at approval; `change check` refuses unticked tasks, so human steps
are plain text); the 0.6.0 plan's dogfood plan and ticket ledger.
