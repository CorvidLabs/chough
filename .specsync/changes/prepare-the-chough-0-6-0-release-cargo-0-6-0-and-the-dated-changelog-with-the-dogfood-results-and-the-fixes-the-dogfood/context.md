---
change: prepare-the-chough-0-6-0-release-cargo-0-6-0-and-the-dated-changelog-with-the-dogfood-results-and-the-fixes-the-dogfood
artifact: context
---

# Context

Chough 0.6.0 (human-first records) was accepted by Leif and archived on SpecSync 6.3.0
(2026-11-12), and this repository was then migrated to 0.6.0 records with `chough
migrate` (receipt `.chough/archive/migrations/2026-11-12-from-0-5.md`). The archived
change's dogfood plan makes the release preparation the dogfood cycle: one small
native change driven intent to done with the 0.6.0 build only, with any fix the
dogfood finds. Hosted CI is green on the migrated tree (977bd246).

The dogfood so far found:

- D1: an in-place migration keeps the machine-local active-session pointer, which
  0.5.x wrote as the session's key. That key is now the session's alias, but the
  pointer is read as an ID only, so `next` and `status` fail until `session use`
  or `doctor --fix`. Clones never showed it, since a clone drops `.chough/local/`.
- CodeQL's rust/cleartext-logging rule flags a test helper named `session_id` (it
  returns a migrated session's dated folder name, not a secret) where its value
  reaches an assertion message in `tests/common/mod.rs`.
- The KNOW gap list (#216), computed from `hi/knowledge.md` and the recorded
  evidence: KNOW-3, KNOW-6 and KNOW-24 have no case linking them, and KNOW-4 and
  KNOW-15 have cases but no passing receipt in the records.
- #148's local bug record: GitHub #148 is closed and #157 merged on 2026-11-06, but
  the record has no linked failure, so Chough will not mark it verified.

Re-scoped (2026-11-12T15:14Z): asked whether deferred bugs block the 0.6.0 tag, Leif
chose "Block the tag" ("Fix both inside 0.6.0 before tagging. Skip 0.6.1."), so two
more dogfood findings join this change:

- D2: for a second native change on one branch, the gate summary's What changed
  counts from the merge base with main, so it includes the earlier change's work.
  REQ-cli-061 specifies that, so the fix is a contract change (t38c's wording).
- The jackdaw dogfood (t38f): `chough migrate`'s preview exits 0 with a digest while
  native changes are open, and only `--apply` says it will refuse.

Four flaky-test bugs stay open from 0.6.0 (the macOS writer lock timeout, the timing
native-budget regression on MSRV, the timing trace flake, the verify lane's test
step under load). The release gate allows no tag while any Chough bug is open, and a
flake that will not reproduce goes to Leif, never closed to clear the gate.

Re-scoped again (2026-11-12, 15:35Z to 15:41Z, relayed by t370). Leif saw this
repository's first journal post: its title is the raw three-line objective, cut
mid-word in the index, and its body is a stale first bullet ("Plan step 3 … 561
passed"). He chose "yes put the journal fixes in 0.6.0": short human titles, real
summaries, a backfill so each of the 68 earlier archived stories has a post, and 0.6.0's
own post written again under the new rules. Since Chough cannot write a good title or
summary by itself, he chose "Yes, add both (Recommended)": `chough checkpoint
--title/--summary`, which the agent sets before the move into done, and `chough
journal add --record <story>` for the backfill, with a derived title and body as the
fallback. He chose "yes add lessons in 0.6.0": a few real cross-project lessons from
this release in `knowledge/lessons/`. For the jackdaw finding (t38f: `chough doctor`
reports the deliberate `/tmp` `XDG_DATA_HOME` settings in case definitions as leaks and
offers `--redact`, which would break the cases) he chose "Yes (Recommended)": a
temporary-root path in a case definition is not a leak.

Re-scoped a third time (2026-11-12T16:26Z, relayed by t370). A fresh-eyes tester
(t38j, a cold full cycle on a scratch project plus a 0.5.0 migration) rated 0.6.0 at 7
of 10 and found twelve issues. Leif wants 0.6.0 at 90% or better with no open bug at the
tag, and chose "Yes, fix all (Recommended)" for every one: bugs and gaps (the cases
check crash on a module's first change, no new-project path in the docs, `chough
context` reading AGENTS.md files above the project), wrong or misleading messages
(`gate record`'s output and approval note, the stage-move authority line, the empty
entry a plain `specsync change check` leaves, `chough status` cutting its next action
and the next action going stale after a gate closes), and small requirement changes
(`next` at plan naming the scope gate instead of SpecSync's approve, the scope-gate
summary showing the delta and plan, and record polish: labelled Timeline fields, a bug's
Fix line, a labelled trigger, receipts in time order and `none` in empty receipt
sections). The CHANGELOG and docs corrections the report also found were already in
scope.

On 2026-11-12T16:38Z Leif also chose "Yes, fix it (Recommended)" for a finding from
jackdaw's real migration (t38f): `chough migrate --apply` can write a `session.md` that
ends with a blank line, which `git diff --check` rejects.

On 2026-11-12T17:42Z Leif chose "Yes, fix it (Recommended)" for a fifth flaky test that
t38b traced to Chough itself: on macOS, one SIGKILL to a stopped command's process
group can miss a child the group gains while it is signalled, so the child survives.

On 2026-11-12T19:43Z Leif chose "Yes, fix all + re-rate (Recommended)" for the 16
findings of the second fresh-eyes round: t38k's F1 to F13 (rated 7/10 at f69889a9) and
t38h's (1), (2) and (6). Most important: a gate that drives no native step, or a close
with `--no-native`, accepted any `--decided-by`, so intent and design approvals could
be recorded in any name (F1). The rest: `next` looping on itself after a gate, a
confusing accept, a cut ship command, a gate list footer that names a refused reply
(F3 to F6); the story's verified list, its bug links, its stale `next` and its
direct-write order (F8, F9, Kimi 6); the migrate preview on a migrated tree, a start
entry for timing, output polish and an outdated `chough` on `PATH` for the merge
drivers (F10 to F13); `evidence add` and requirements a delta adds (Kimi 2); and the
docs gaps (F2, F7, Kimi 1). Each fix gets a failing test first, and a third fresh
tester re-rates the result against the 90% bar.

On 2026-11-12T21:47Z Leif chose "Yes, fix all + re-rate (Recommended)" for the 12
findings of the third fresh-eyes round (t38m, 8/10 at 425fa948): `next` at accept
passing on SpecSync's own review command (F1) and `session close` dropping an open
human gate unrecorded (F2), the two gate bugs; the dated lesson form undocumented (F3);
gate summaries that name neither the intent nor the delta (F4, F5); two messages (F6);
`next` at test and at plan (F7, F8); about 150 lines of JSON per stage move, now a
short summary with the full JSON behind `--json` so agents lose nothing (F9); the
story's readability: receipts before the story, its bug unnamed, a failed run's body
without its reason, `context`'s JSON block, a migrated bare ID (F10); timing's first
segment (F11); and the init hint for Claude Code and a text migrate preview (F12).
Each fix gets a failing test first, and a fourth fresh tester re-rates the result.

On 2026-11-12T23:32Z, after a fourth fresh-eyes round (t38n, 7.5/10 at a7d6406a), Leif
chose "Fix 4 real ones, then release (Recommended)": fix the 4 real problems with failing
tests first, run the full suite, then his review and the tag; "Polish and the Mac
leak-check go to 0.6.1. Real use on jackdaw will tell us more than another tester." The
four: a gate close's next action outliving the move it named (F1); the scope summary
ignoring uncommitted change files (F2); no pointer to `chough skill update` after a
migration leaves the skill copy outdated (F3); and accept steps that stale the evidence
twice, with `next` routing to the review gate past stale cases (F4). F5 to F11 and the
host-name leak rule are recorded as deferred bugs, quoting him. No fifth tester.
