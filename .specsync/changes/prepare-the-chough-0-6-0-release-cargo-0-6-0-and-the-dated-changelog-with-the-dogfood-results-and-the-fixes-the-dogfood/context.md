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
