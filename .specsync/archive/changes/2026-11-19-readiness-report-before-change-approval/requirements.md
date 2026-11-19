---
change: readiness-report-before-change-approval
artifact: requirements
---

# Requirements

REQ-cli-067 is added, in `deltas/cli.md`, and is not edited into
`specs/cli/requirements.md` until `chough change check` materializes it.

`chough change ready <id>` reads the named change, prints one report, and exits
0 when the report has no finding and 1 when it has any. It changes no tracked
file and restores SpecSync's gitignored hash cache. An unknown id exits 1 and
names `chough change status`. Each finding names one exact fix command, with the
real change id and paths filled in.

The report covers only: a missing or extra delta, a delta whose approved body is
unchanged while the canonical block has moved and the approved body does not
already carry that move, an undocumented export in a declared file as
`specsync check --spec` reports it, a case that links another module's
requirement, a changed file outside the declared paths, and an unanswered
interview question or an unfinished selected artifact.

Acceptance criterion on the change: `chough change ready` exits 0 when nothing
is missing and 1 when a finding remains, and each finding names the command
that fixes it.
