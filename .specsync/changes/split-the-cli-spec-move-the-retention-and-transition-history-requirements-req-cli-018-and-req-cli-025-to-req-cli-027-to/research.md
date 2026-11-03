---
change: split-the-cli-spec-move-the-retention-and-transition-history-requirements-req-cli-018-and-req-cli-025-to-req-cli-027-to
artifact: research
---

# Research

- SpecSync 6.1.2 `change check` validation: a non-removed requirement in a module
  delta must start with `REQ-<module>-`; added IDs may not reuse a tombstone from any
  applied or archived REMOVED delta (spec-sync src/change.rs).
- `specsync check --strict` refuses a source file mapped by two specs ("duplicate
  spec ownership") and a spec without a `files:` list (probe in a scratch clone).
- `chough cases` binds each cases file to one module contract, and case IDs must be
  `TC-<module>-...` (src/cases.rs validate_shape, src/evidence.rs for_cases).
- Case co-linking at main 7bd1b62: REQ-cli-018 and REQ-cli-025 to 027 share no case
  with any other requirement; the timing, bug and stage-gate groups do.
- Sizes at 7bd1b62: specs/cli/requirements.md 65,051 bytes; the four moved blocks
  6,613 bytes.
