---
change: readiness-report-before-change-approval
artifact: testing
---

# Testing

`tests/ready.rs` covers REQ-cli-067 before the implementation is treated as
done:

- A draft missing its delta, with an extra delta, an unanswered question and a
  TODO artifact exits 1, and the report names the fix for each. The same tree
  with those fixed exits 0, and no tracked file and no hash cache changes.
- A delta approved against a requirement, then left unchanged while that
  requirement's canonical text changes, is named. A delta rewritten onto the
  new text is not. A delta that already carries the canonical move is not.
- An undocumented export in a declared file is named as `specsync check --spec`
  names it. An export in a file the change does not declare is not.
- A changed case that links another module's requirement is named with the
  owning module. A case that links only its own module is not.
- A changed file outside the declared paths is named. A file under a declared
  prefix is not. A file under `verification_ignored_paths` is not. On a draft
  the fix is `chough change answer` with the full path list.
- An unknown change id exits 1 and names `chough change status`.

`.chough/cases/cli.change-ready.cases.toml` binds those tests to REQ-cli-067
under module `cli`. `tests/package.rs` asserts the fast path names
`chough change ready <id>` between the delta phrase and the scope-approval
phrase, and still fits in 32 lines.

## Requirement evidence

| Requirement | Evidence |
|---|---|
| REQ-cli-067 | `tests/ready.rs` as listed above, the case file `.chough/cases/cli.change-ready.cases.toml`, and the acceptance criteria on the added requirement |
