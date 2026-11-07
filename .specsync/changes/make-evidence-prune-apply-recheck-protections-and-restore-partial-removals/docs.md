---
change: make-evidence-prune-apply-recheck-protections-and-restore-partial-removals
artifact: docs
---

# Docs

- `specs/history/requirements.md`: materialize the modified REQ-history-004 and its new rollback acceptance criteria.
- `specs/history/testing.md`: bind the four per-change cases to the deterministic race, same-set and earlier-set rollback, and recreated-file regressions.
- `specs/history/history.spec.md`: add this change to the contract history and increment its version without renumbering existing rows.
- `docs/FRD.md`: summarize immediate protection rechecks, byte-checked removal and honest rollback under REQ-history-004.
- `docs/cli.md`: document stale-hold refusal, restored events, create-new rollback and recovery holds for uncommitted bytes in the Retention section.
- `.chough/cases/history.prune-race.cases.toml`: add only this change's cases with curated stable IDs and existing module/requirement links.
- `CHANGELOG.md`: add one Unreleased bug-fix entry for #65.

The command syntax, successful output schema and retention rules do not change, so no template, installed skill copy, README or release document needs an edit. Existing ledger entries and historical documents remain untouched.
