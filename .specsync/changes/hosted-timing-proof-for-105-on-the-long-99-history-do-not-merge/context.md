---
change: hosted-timing-proof-for-105-on-the-long-99-history-do-not-merge
artifact: context
---

# Context

Draft PR #119 is a proof only. It will be closed unmerged, and its branches deleted.

It is #99's final head `6a9bfad6` plus one commit (`d0480d65`), which copies three files from #118's head `e18e0941`:
- `.github/scripts/check-specsync-archive.py`
- `.github/scripts/test-specsync-archive.py`
- `.github/workflows/ci.yml`

The aim is a hosted run of the four `SpecSync Archive` shards on issue #105's recorded long candidate: 361 PR revisions over trusted base `8b7f8bf6`, with 86 archives.

The first hosted run, 42128424556, refused correctly at the native audit, because no change covered those paths. Leif chose "A: add a native change" (2026-11-05 16:22 MST, via the orc): cover the three paths, with nothing relaxed. This change adds one archive (87).
