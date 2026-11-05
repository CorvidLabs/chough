---
change: no-conflicting-files-per-change-case-files-a-knowledge-rebuild-check-and-a-bug-merge-driver-drive-12
artifact: plan
---

# Plan

## Approach

Three small product changes, each behind a failing test, plus repository wiring: `.gitattributes`, `fledge.toml` and AGENTS.md.

## Out of scope

- SpecSync's spec change logs and `requirements.md` (CorvidLabs/spec-sync#862).
- Moving existing cases out of `cli.cases.toml`: new changes add their own files, and old ones stay put.
- Reduction and the useful vault (KNOW-21 to 28), which come in later pull requests.

## Steps

1. Approve the definition, then move to build.
2. Add the failing tests in tests/cases.rs, tests/vault.rs and tests/bugs.rs.
3. Implement the loader, `index --check` and `bug merge`.
4. Wire `.gitattributes`, `fledge.toml`, AGENTS.md and the docs.
5. Run a parallel pull request simulation that also adds a case file each, rebuilds the vault, and edits one bug on both sides.
6. Run verify and close out.

## Risks

- **Union output in `knowledge/` is incoherent until it is rebuilt.** The check fails CI until `chough library index` runs, and every move into done rebuilds anyway.
- **An unconfigured clone falls back to a normal text merge for bugs.** That is no worse than today, and AGENTS.md shows the setup.

## Constraints consulted

DRIVE-12, DRIVE-3, KNOW-14, KNOW-3 (the vault points to sources and no gate reads it; the check compares only generated files and gates no lifecycle), and REQ-cli-020 (bug records).
