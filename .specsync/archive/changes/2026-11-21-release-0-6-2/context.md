---
change: release-0-6-2
artifact: context
---

# Context

0.6.2 is a patch release, decided by Leif on 2026-11-21 (about 23:59Z, relayed by his
planning session, hc t4c): "Yes, release 0.6.2 (Recommended)", to releasing Chough 0.6.2
once the last regression check passes, with the remainder moved to 0.6.3. #404 stays open
for its second item (accepting a delta SpecSync rebased, spec-sync #1089) and moves to
0.6.3.

This change is the release preparation of `docs/releasing.md`: the version, the change
log assembled from the 43 merged fragments in `changelog.d/`, and the rebuilt knowledge
index, on a branch, merged before the tag. The tag waits for the tester's regression
line on main e272a6f3. Tagging, publishing, the GitHub release and the installed
binaries on this Mac are Leif's separate steps and are not part of this change.

0.6.2 needs SpecSync 6.4.0 (#404, #429); its changelog says so.
