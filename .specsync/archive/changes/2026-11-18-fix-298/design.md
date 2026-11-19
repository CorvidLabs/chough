---
change: fix-298
artifact: design
---

# Design

Before its review relay or SpecSync runs, `chough change ship <id>` reads the change's `base_commit` from `.specsync/changes/<id>/state.json` and lists the commits `base..HEAD` (at most 1,000). For each commit it lists the paths under `.specsync/changes/` and `.specsync/archive/changes/` (`git ls-tree -r --name-only`):
- the change IDs open in it, from `.specsync/changes/<id>/…`;
- the IDs archived in it, from `.specsync/archive/changes/<YYYY-MM-DD>-<id>/…`.

An ID in both refuses the ship as `refused:corrupt-history`, naming the commit, the ID and the recovery. With no base commit or outside Git, there is nothing to check. A code comment marks the check interim until spec-sync#1082.
