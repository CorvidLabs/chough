---
change: bundle-hygiene-case-binding-environment-and-colour-safe-matching-and-actionable-requirement-and-bug-errors
artifact: context
---

# Context

A Raven run with Chough reported five usability problems (CorvidLabs/chough #10 to
#14). Triage reproduced them on main 2756dc2 and the human (0xLeif) agreed to split
them into two packages. This change is package 2: bundle hygiene and clearer case and
bug errors. It covers:

- #13 (BUG-0018): the shipped bundle has two em dashes (templates/frd.md line 23,
  templates/cases.toml line 1). `chough init --host` copies them into every host
  directory, and Raven's docs lint rejects the `.agents/` copy of frd.md.
- #14 items 4 and 5 (BUG-0019): `bug add --classification blocking` and a case
  linking another module's requirement fail with errors that do not say how to fix
  them.
- #11, error half (BUG-0017): a case linking a requirement that an approved delta
  adds fails with the same bare `requirement not found in contract companions` as a
  typo. Its other half, a documented order without double runs, is the
  evidence-freshness change's: it documents the accept order, and the repeat runs left
  after that depend on #10 and #12. BUG-0017 therefore stays open after this change.
- #14 item 1: a `command` binding has no environment; NO_COLOR or a port had to go
  through `/usr/bin/env` in argv.
- #14 item 2: `expect_output` matches raw output, so colour codes split fragments
  such as `Test Files  1 passed (1)`.
- #14 item 3: documentation only. External artifact paths stay refused (REQ-cli-006).

Ruled out during triage:

- Resolving requirement links against unmaterialized deltas: Chough would parse and
  own SpecSync's delta format, cannot confirm the delta is still the approved one
  (the approval digest is not a plain file hash, and BUG-0002 shows status keeps
  approval_valid true after an edit), and the runs would go stale at `change check
  --commit` anyway. Deferred until SpecSync publishes approved delta requirement IDs.
- Setting NO_COLOR by default: it silently changes the environment under test.
- External artifact paths: they break REQ-cli-006 and are neither reviewable nor
  portable.
- Rewriting em dashes in SpecSync-generated archives: native records stay untouched;
  the lesson-bundle heading is an upstream spec-sync report.

The staleness issues #10 and #12 (BUG-0015, BUG-0016), BUG-0014 and the documented
accept order are package 1, the evidence-freshness change. In the grouping the human
agreed, this package owns only #11's error hint, so it does not edit references/
build.md, plan.md, accept.md or lifecycle.md. Both packages edit `src/evidence.rs`,
`src/bugs.rs`, `src/skill.rs` (each adds `KNOWN_PREVIOUS` digests for different
files), `tests/bugs.rs`, `tests/cli.rs`, docs/cli.md and docs/architecture.md, so the
second to merge rebases; no bundle file is edited by both.

`.chough/cases/cli.cases.toml` gains the case definitions for BUG-0017 to BUG-0019,
but `.chough/` is not listed in the affected paths: it is Chough's own record
directory, it is not one of the SDD meaningful paths in .specsync/sdd.json, so listing
it adds no coverage, and this workflow's lesson is that listing it stales verification.
The cases are declared in tasks.md and testing.md instead.

The native Intent repeats the title. SpecSync 6.1.1 derives the title, the Intent and
the change ID from the one `change new` description, and no `change answer` question
sets the Intent alone (an `intent` answer is stored as an unrelated key); every
archived change in this repository has the same shape. The intent is this context:
fix the reported usability problems in #11 (error half), #13 and #14 without weakening
any fail-closed rule.
