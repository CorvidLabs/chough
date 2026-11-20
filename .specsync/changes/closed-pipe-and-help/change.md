---
id: closed-pipe-and-help
state: draft
type: bug_fix
base_commit: 7bf110dcca70bdd3565007086aee40a286f40eff
---

# Closed pipe and help

## Intent

closed pipe and help

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- Every command whose standard output or standard error is closed, as in chough next | true or chough next | head -1, stops printing and exits with the code it would have exited with, never 101 and never a panic or a Broken pipe message; a write command still saves (#383). chough --root . help --help, chough --root=. help -h and chough --redact help --help print the top-level help and exit 0, as chough help --help does, and docs/cli.md names --help and -h as the help flags and chough -V and --version as the top-level version flags (#399).

## No-spec Rationale

Bug fixes that bring Chough to what its requirements already assume: no requirement says a command panics or exits 101 when its reader goes away, or that help depends on where it stands among the global options; docs/cli.md's sentence on -V is corrected to match the helper.
