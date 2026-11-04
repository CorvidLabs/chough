---
id: context-knowledge-directory-knowledge-by-default-with-the-librarian-on-and-an-opt-out-one-regenerated-summary-per
state: archived
type: feature
base_commit: 0363d4af368bb07cc07f16c2f5fff7645c1c05bc
---

# Context knowledge directory: knowledge/ by default with the librarian on and an opt-out, one regenerated summary per archived change with a lessons index, and raw proof outside the newest 3 archived changes leaves the working tree only through a previewed, authorized, validity-checked step

## Intent

Context knowledge directory: knowledge/ by default with the librarian on and an opt-out, one regenerated summary per archived change with a lessons index, and raw proof outside the newest 3 archived changes leaves the working tree only through a previewed, authorized, validity-checked step

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- With no preference set, the librarian is on and its directory is knowledge/; pref set librarian off opts out with a recorded source, and checkpoints and cold resume still work
- library index writes knowledge/INDEX.md and one knowledge/archives/<change-id>.md per archived change with its title, criteria, requirements, commits, listed records and artifacts with digests and current location, observation outcomes and environments, linked bugs and lesson bundle; regenerating from the same sources gives identical bytes and no gate reads the directory
- The index lists existing library/ snapshots and briefs as they are, byte-identical and unmigrated, and a lessons index that points to each module lesson source and each lesson bundle without copying lesson text; chough next names the knowledge index
- library archive previews without writing which observation sets of archives outside the newest 3 would leave the working tree, the bytes and why each other set stays; checkpoint, bug, open-change, hot-archive, validation, newest-run, newest-failure, non-routine and directly named protections all still hold
- library archive --apply needs retention consent or preference, is refused while any native change is open, removes only files committed byte-identically at HEAD, writes each removal to the retention ledger first with the knowledge entry and digests, shows removed observations as pruned, and restores everything and refuses when an affected archive's terminal evidence validity changes
- fmt, lint, the full tests and fledge lanes run verify pass

## No-spec Rationale

Not applicable
