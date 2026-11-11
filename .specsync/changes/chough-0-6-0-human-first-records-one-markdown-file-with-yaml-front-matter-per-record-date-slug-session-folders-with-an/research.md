---
change: chough-0-6-0-human-first-records-one-markdown-file-with-yaml-front-matter-per-record-date-slug-session-folders-with-an
artifact: research
---

# Research

What was looked at while planning, on 2026-11-10 and 11, macOS arm64, with the
chough built from this branch (0.5.0), specsync 6.2.0 and hi 0.8.0.

## Does removing the vault copies weaken a native archive? (one probe)

The 0.5.x trims existed partly to protect archive validity (KNOW-10), and every
archived change's acceptance manifest lists `knowledge/` files, for example 915
paths in the newest archive, most of them `knowledge/archives/*.md`.

Probe: in a throwaway clone at 250148ce, `specsync change status
return-the-observation-id-from-cases-record-and-evidence-add-issue-60 --json`, then
`git rm -r knowledge/archives knowledge/intent`, commit, and the same status read.
Both replies were byte-identical (382,025 bytes): `approval_valid: true`,
`terminal_evidence.validity: authenticated-history`. The first read took 3 min 42 s,
the second 36 s.

Limits: one archived change, one clone, status only. It does not cover the CI
archive authentication shards or the other 168 archives. The dogfood step therefore
runs the full verify lane and CI archive authentication after migrating
(plan.md). Recorded as evidence on this session.

## This repository's 0.5.x records (at 250148ce)

| Path | Tracked files | Size |
|---|---|---|
| `.chough/evidence` | 584 | 2.6 MB |
| `.chough/artifacts` | 585 | 3.3 MB |
| `.chough/history` | 644 | 2.5 MB |
| `.chough/bugs` | 76 | 312 KB |
| `.chough/sessions` (open and archive) | 10 + 7 | 76 KB |
| `.chough/transitions.jsonl` | 1 (602 lines) | 220 KB |
| `.chough/pruned.jsonl` | 1 | 211 KB |
| `knowledge/` | 906 | 14 MB |
| of which `knowledge/attachments` | 612 | 12 MB |

`chough next` reports 253 referenced observations that are not current and 466
unreferenced ones. `knowledge/attachments/` holds 60 sessions the old roll-up moved
there (`knowledge/sessions/` has a note for each); migration turns them back into
archived stories. The migration receipt lists roughly 2,800 files, about 400 KB,
inside its 4 MiB bound.

## Issue 165, the new repro

algochat, 2026-11-10, chough 0.5.0: `stage advance` accept to done on `release-0-7-0`
archived that session and also deleted the committed `.chough/sessions/archive/release-a.json`,
with no ledger entry. Cause, from the 0.5.0 code and REQ-history-011: the roll-up at
done keeps only the five newest archived sessions and moves the others into
`knowledge/attachments/<folder>/sessions/`; where `knowledge/` is not committed that is
a silent deletion. 0.6.0 removes the roll-up and bounds nothing (KNOW-34), and the
regression is written in the 0.5.x layout first so it fails on 0.5.0.

## SpecSync behaviour relied on

- `specsync change answer <id> acceptance_criteria` takes a JSON array for a list
  with commas inside items; a plain string becomes one item, and a second answer
  replaces the first (checked in a throwaway clone).
- Deltas use `## ADDED`, `## MODIFIED`, `## REMOVED` with `### REQUIREMENT REQ-<module>-<n>`;
  removed IDs stay tombstones. A new module's spec files are created in build, as the
  2026-11-04 reporting split did.
- The plan stage's lessons: a delta is frozen at approval; `change check` refuses
  unchecked `tasks.md` boxes, so human gates and case runs are plain text.

## Folded PRs, read at their heads

- #233: actor fields from `CHOUGH_ACTOR`, `CHOUGH_ACTOR_MODEL`, `CHOUGH_HC_TERMINAL`,
  `CLAUDECODE`, `HC_TERMINAL_ID`; never fails; fixture clears the variables. Kept,
  with the terminal as an ID only (LIFE-17 as amended) and values leak-checked.
- #210: `chough correct <EVT-…> --reason`; refusals for unknown, blank, over 4,096
  bytes, correction of a correction; bounded list in next. Kept, retargeted to
  timeline entry IDs and archived stories.
- #208: absolute external artifacts fingerprinted in place, 64 MiB bound, refusals
  for missing, directory, symlink, oversize. Kept, except that the committed record
  now holds a neutral ID instead of the path (#234).
- #211: copy-never-move roll-up. Superseded by KNOW-30 and KNOW-34.

## The independent alignment audit

An auditor (chough-060-audit) classified 629 items against Leif's direction: 332 keep,
191 rework, 94 remove, 2 elsewhere, 10 for Leif. It was read-only and is not
committed here. What this plan took from it:

- REQ-cli-029, 034 and 042 needed changes the first draft missed (the agent-actor
  rule, and acceptance lines naming removed commands).
- REQ-history-006 still archived other finished sessions on a switch, against
  Leif's #207 decision; it no longer does.
- Six open bug records were already fixed elsewhere (#157, #142, #172, the 6.2.0
  config comment, spec-sync #823 and #824); they close with links.
- 96 committed files under `.chough/` and `knowledge/` hold home-folder paths; the
  migration redacts or drops them, and Git history keeps them (Leif: no rewrite).
- Migration fixtures from every release; KNOW-24, 26 and 27 covered; a keep-ours
  driver instead of union merges for generated files; one home for cli lessons; one
  roadmap; flakes as bug files; DECISIONS.md as an index; the shared skill copy
  refreshed after release; the upgrade warning in the changelog.

Where this plan still differs from it: TC-history-0054 stays, re-linked to
REQ-records-009, because LIFE-8's "a removed record never reads as passed" still
applies to migrated IDs.

## Not found

- No timing history to estimate from: `chough timing` has one cycle for this session
  and its native reads ran out of budget. Estimates in plan.md are relative sizes.
- No SpecSync command to change a draft's affected paths; the draft is recreated
  with the full path list instead when it needs one.
