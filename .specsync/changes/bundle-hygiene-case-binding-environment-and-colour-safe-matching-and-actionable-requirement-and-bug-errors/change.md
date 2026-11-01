---
id: bundle-hygiene-case-binding-environment-and-colour-safe-matching-and-actionable-requirement-and-bug-errors
state: draft
type: feature
base_commit: 85650d93ad9a811c2c54e08256620d351754da8d
---

# Bundle hygiene, case binding environment and colour-safe matching, and actionable requirement and bug errors

## Intent

Bundle hygiene, case binding environment and colour-safe matching, and actionable requirement and bug errors

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- Bundle hygiene: no file in the shipped skill bundle (skills/chough-workflow) contains U+2014 or U+2013; templates/frd.md and templates/cases.toml use colons in their place, and a test in tests/package.rs walks every bundle file and fails on either character. KNOWN_PREVIOUS lists the dashed digests (templates/frd.md 5647b56e..., templates/cases.toml 6c7ebaa1...) and the 2756dc2 digest of every other bundle file this change edits, so `chough skill status` reports unedited copies from earlier builds as known-outdated, `chough skill update` replaces them, and edited copies stay edited. Chough's own .agents/ and .claude/ copies are refreshed and read current (BUG-0018, LIFE-4, LIFE-9, REQ-cli-016, REQ-cli-019).
- Actionable requirement refusals: a requirement link that cannot be resolved is still refused with the same exit code, and the message names the companion files searched. When the ID has the form REQ-<module>-NNN (module found by removing `REQ-` and the last `-NNN`, so hyphenated modules such as checkout-ui work) and that module's bounded requirements.md contains it under the same token match the refusal uses, the message names that module and says what to do: a cases file binds one module's contract, so move the case to .chough/cases/<module>.cases.toml (cases check, run and record), or pass --contract specs/<module>/<module>.spec.md (evidence add). Otherwise it says the ID may be a typo or a requirement that an approved native change adds but `specsync change check` has not yet materialized, and to run that check before running or recording cases that link it. The hint reads neither the checkpoint nor any delta, and no unresolved link is accepted (BUG-0017, BUG-0019, FLOW-7, LIFE-5, REQ-cli-005, REQ-cli-009).
- Actionable bug flag errors: `chough bug add` and `chough bug update` given a scope value in --classification say that the value is a scope and to use --scope, and given a classification value in --scope say the reverse; `--help` for both commands lists the allowed values of both flags. Validation stays in Chough rather than moving to clap, so exit codes and bug record shapes are unchanged (BUG-0019, LIFE-5, REQ-cli-012).
- Declared binding environment: a case binding for any adapter may declare `env`, a table of variable names to string values. Names match [A-Za-z_][A-Za-z0-9_]*, values contain no NUL, and a binding holds at most 32 entries of at most 4 KiB each; violations are refused by `cases check` with the case location. The declared variables are set for the executed command on top of the inherited environment; native reads are unchanged. An empty `env` is not serialized, so the digests of existing cases are unchanged and their observations keep their freshness, while adding or editing `env` changes the digest. An observation of a case that declares `env` names the declared variable names, not their values, in a limitation that also says other inherited variables were not recorded. Docs say `env` lives in committed case files and is not for secrets (FLOW-7, REQ-cli-009, REQ-cli-011).
- Colour-safe output matching: expect_output fragments and the cargo-test count are matched against the captured text with terminal escape sequences removed (CSI sequences, OSC sequences ended by BEL or ESC backslash, and two-byte ESC sequences), while the stored artifact stays the raw captured bytes. When any sequence was removed the observation carries the limitation `terminal escape sequences were removed before matching; the artifact keeps the raw output`. An expect_output fragment that contains ESC (U+001B) is refused by `cases check`, because it could never match the cleaned text. Vitest-style coloured output whose visible text holds `Test Files  1 passed (1)` passes that fragment, and a fragment absent from the visible text still fails. Chough sets no NO_COLOR or other variable by default (FLOW-7, REQ-cli-009, REQ-cli-011).
- Guidance: docs/cli.md documents `env` (not for secrets), escape removal, the ESC refusal and the new error hints. docs/architecture.md and the skill's references/testing.md say artifacts stay inside the project (no external artifact paths; REQ-cli-006 is unchanged) and that large files such as screenshots should be kept small and may be ignored by Git or tracked with Git LFS: the observation keeps the fingerprint, and a clone without the file reads stale-artifact, never passed. The skill's references/build.md, plan.md and testing.md say tasks.md lists implementation work only, and that running or recording cases for requirement IDs a delta adds is a post-verification step after `specsync change check <id> --commit` has materialized them (BUG-0017, LIFE-4, FLOW-7).
- Behavior these rules do not name is unchanged. The full suite, the feature-and-bug exercise and native structural checks pass on Ubuntu, macOS and Windows CI; regression tests cover each rule above, and BUG-0017, BUG-0018 and BUG-0019 are verified by passing observations of new cases.

## No-spec Rationale

Not applicable
