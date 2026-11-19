# Lesson bundle: fix-298

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Fix 298
- **Kind**: BugFix
- **Specs**: cli
- **Paths**: src/change.rs, tests/change.rs, docs/cli.md, .chough/cases/cli.ship-refuses-a-corrupt-history.cases.toml, changelog.d/2026-11-18-fix-298.md
- **Acceptance**: When a commit between the change's base commit and HEAD holds a change both open under .specsync/changes/<id>/ and archived under .specsync/archive/changes/<date>-<id>/, chough change ship refuses before it runs SpecSync's review or ship, naming the commit, the duplicated change and how to recover, with every file unchanged and no native step run; without such a commit the ship runs as before. The docs say the check is interim until SpecSync's preflight (CorvidLabs/spec-sync#1082).

## Evidence

- Verification commit: `dc083b9718058cfbbc16df92a718b8fbd2837bae`
- Base commit: `cef86e2622a2a908856b0c2cf11b29d3c1d3e472`
- Verified by: `specsync check --spec cli`

## From the change's context.md

# Context

GitHub #298, from CorvidLabs/starling.
- **Symptom:** `chough change ship` ran SpecSync's review and ship, both of which succeeded, and then the move into done was refused with `refused:native-archive-corrupt-history`. The ship had worked but reported an error, and the session was left at accept with its change archived.
- **Cause:** a merge of main into a build branch kept the branch's copy of a squash-merged change under `.specsync/changes/<id>/` while also adding main's archived copy under `.specsync/archive/changes/<date>-<id>/`. SpecSync reads the full history, so every change whose history holds that commit reports `terminal_evidence.validity: corrupt-history`.

Leif's decisions (2026-11-18, relayed by hc t4c):
- His decision comment on #298, "check before shipping": `change ship` first checks whether the archive's terminal evidence would authenticate, and refuses before SpecSync's review or ship if not, naming why and how to recover.
- "Both: Chough check now + SpecSync": SpecSync reports validity only after archiving, and `specsync change ship --dry-run` stops before the review, so Chough checks the change's own history now. t4c filed the SpecSync preflight as CorvidLabs/spec-sync#1082.

The Chough check is interim until that preflight exists, and the code, docs and REQ-cli-059 say so.

## From the change's design.md

# Design

Before its review relay or SpecSync runs, `chough change ship <id>` reads the change's `base_commit` from `.specsync/changes/<id>/state.json` and lists the commits `base..HEAD` (at most 1,000). For each commit it lists the paths under `.specsync/changes/` and `.specsync/archive/changes/` (`git ls-tree -r --name-only`):
- the change IDs open in it, from `.specsync/changes/<id>/…`;
- the IDs archived in it, from `.specsync/archive/changes/<YYYY-MM-DD>-<id>/…`.

An ID in both refuses the ship as `refused:corrupt-history`, naming the commit, the ID and the recovery. With no base commit or outside Git, there is nothing to check. A code comment marks the check interim until spec-sync#1082.

## From the change's testing.md

# Testing

`cargo test --locked --test change ship_refuses_a_history_with_a_change_both_open_and_archived` builds an accept-stage shipping session whose history holds a commit with `.specsync/changes/old/` and `.specsync/archive/changes/2026-11-01-old/`, later removed. `chough change ship` must refuse as `refused:corrupt-history`, naming the commit and `old`, with no SpecSync call (the stub's call log stays empty) and every file unchanged. It fails on main. Case `TC-cli-ship-refuses-a-corrupt-history` binds it. The existing ship tests keep passing.

## Where these lessons go

- `specs/cli/cli.3md`
