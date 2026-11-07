---
change: after-the-move-into-done-chough-library-index-check-passes-with-no-command-in-between-session-notes-link-archive-notes
artifact: context
---

# Context

Issue #191: review finding F1 of the 2026-11-06 Chough review (`_audits/chough-review-2026-11-06.md`). Kimi observed it twice, and the orc reproduced it. Leif's burn-down plan (`_audits/burn-down-plan-2026-11-06.md`, relayed by the orc t18) assigned it to ch-pr69 as the first item: file F1, then fix it.

`chough stage advance` into done reports `librarian_index: refreshed`, yet `chough library index --check` run right after exits 1, and a `chough library index` makes it pass. Reproduced on a fresh clone of spec-sync main 9dd2affd with chough main f41e7ab4: `session use s5-906`, `stage reset --to accept`, `stage advance` (9 sessions rolled), then `--check` failed on 10 files. A diff of the vault written at done against a fresh build shows two causes:

1. **Session notes read earlier output.** `vault::session_notes` writes the `Native change:` link only when `archives/<change>.md` already exists on disk. If the roll-up moves sessions into a vault without those notes (a first build, or an archive note that is new in this pass), the links are missing until the next build. This accounts for all 9 `knowledge/sessions/*.md` differences.
2. **Done changes a source after its own refresh.** An archive entry's file table records where each listed `.chough/` path is now (`working tree`, `working tree (changed since archive)` or `Git <commit>`). The required order is roll-up (REQ-history-011), refresh (REQ-cli-043), rotation (REQ-cli-049), then the session self-archive (REQ-history-006). When the change's archive lists its own session file, the refresh sees it as changed in the working tree, and then the self-archive moves it. This is the `archives/...-906.md` difference.

A first `library index` on an empty vault with no rolled sessions passes its own check. On chough main, the archive for #183 doesn't list its session file, so cause 2 doesn't show there.

## Decisions that bind this change

- **HI.** Traced to the approved KNOW-4 ("regenerating with the same sources gives identical bytes"). No new criterion: the burn-down rule is "HI first, trace or draft".
- **How to make done consistent.** One more regeneration after the session archives itself, and only when the first pass refreshed and a later step changed a source (rotation `rotated` or session `archived`).
  - Rejected: moving the refresh after the rotation. REQ-cli-049 keeps ledger entries whose archive has no note yet (the KNOW-23 safeguard), so the rotation needs the refresh before it. Reordering would weaken that safeguard and change three requirements' order.
  - Rejected: patching only the affected rows. That is a second generator that could drift from the first.
- **Cost.** The extra pass costs one more index build at done. Measured `chough library index` on spec-sync (348 archives) at load 75 took 3.4–4.6 s. A small project takes well under a second. `next` and `status` are unaffected.
- **Session-note link.** It is decided from the archive IDs of the same build: the notes REQ-cli-043 writes, one per archived change.
- **Out of scope:**
  - #165 / F2: the roll-up into an untracked vault.
  - #136: an observation recorded after a rebuild.
