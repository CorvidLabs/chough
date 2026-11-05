---
change: keep-the-knowledge-index-under-its-size-bound-by-listing-recent-archives-and-moving-the-full-list-to-a-monthly-timeline
artifact: context
---

# Context

The move into done for #112's first change refused to refresh the knowledge directory with "index exceeds 65536 bytes". `knowledge/INDEX.md` lists every archived change and every lesson bundle, one line each, and with 104 archives it has passed the entry bound (main is at 64,918 bytes). Every later move into done would refuse the refresh, and #112's new `library index --check` step could never pass. Leif chose "Widen #112 with the fix" (2026-11-05 09:21 MST, through the orc). KNOW-28, already confirmed, says the full list of archived changes belongs on a timeline page.
