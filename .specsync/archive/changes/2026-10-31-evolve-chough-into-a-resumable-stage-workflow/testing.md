---
change: evolve-chough-into-a-resumable-stage-workflow
artifact: testing
---

# Testing

Product checks run through `fledge run test` (Rust integration tests in isolated
temporary Git repositories with stubbed `hi` and `specsync` where native state must
be controlled). Structural checks run through `fledge run spec`, HI structure through
`fledge run intent`, and the full set through `fledge lanes run verify`. A structural
pass never stands in for the product checks below. Each requirement gets one or more
stable cases in `.chough/cases/cli.cases.toml`; case IDs are allocated during
implementation and recorded here.

## Track selection for this change

- Functional: always; every requirement below.
- Copy and terminology: relevant, because CLI output, skill references and docs are
  user-visible. Checked by review against a terminology list (observation, not proof;
  run consent versus standing preference; repeated run, not soak) and by assertions on
  key CLI strings.
- Design QA: Chough has no UI. Applied to the visual sample prototype and to rendered
  diagrams; recorded as rendered observations with method and limits.

## Requirement verification

| Requirement | Verification responsibility |
|---|---|
| REQ-cli-007 | Cross-process checkpoint round trip; migrate preserves schema 1 bytes and refuses an existing backup; stage commands refuse schema 1; context and status read schema 1 |
| REQ-cli-008 | Each transition kind; skip needs a reason; back and reset keep history and list refresh targets; build gate refused for draft and unavailable status, allowed for approved; leaving accept refused for verifying, allowed for archived; side-stage caller restored |
| REQ-cli-009 | Reordered file gives identical IDs and digests; edited expected outcome changes digest; duplicate ID, unknown criterion, unknown requirement, unknown field and missing binding rejected with location |
| REQ-cli-010 | Affected requirement selects linked functional cases; unrelated cases omitted with reason; design-QA and copy cases only with their track |
| REQ-cli-011 | Failing assertion recorded failed beside a passing case; zero-match cargo filter recorded not-run; missing expected output recorded failed; changed definition yields stale-case-definition; stop request halts a repeated run and resume keeps stored limits |
| REQ-cli-012 | Failure becomes reproduced classified bug; search finds duplicate and evidence is appended; regression without ancestor pass refused; out-of-scope bug listed without checkpoint scope change; report prints Markdown only |
| REQ-cli-013 | Consent leaves preferences byte-identical; preference requires source; validation refused without authorization; new objective clears consent; validation failure persists as a bug after a new process starts |
| REQ-cli-014 | Accept advances to done; release entry refused without authorization and allowed with consent; dry-run observation reported as dry-run |
| REQ-cli-015 | Snapshot keeps source bytes and digest; escape and symlink rejected; index lists open bugs; checkpoint works with the librarian off |
| REQ-cli-016 | Claude host install creates the complete bundle; repeat preserves edits; unmodified 0.1.0 file updated; edited file reported unchanged |
| REQ-cli-017 | Schema 1 observations keep freshness results; new kinds accepted; existing kinds valid |

REQ-cli-001 to REQ-cli-006 remain covered by the existing tests, which must keep
passing, including the Windows-portable record-limit test.

## Exercises recorded as observations

| Exercise | Method | Evidence |
|---|---|---|
| Feature and bug loop in an isolated sample repository: case fails on a deliberate assertion, bug recorded and deduplicated, fix, focused retest passes | Automated plus agent-assisted | Case observations, bug record, logs |
| Nonvisual sample skips design with a reason | Automated | Transition log entry |
| Visual sample prototype reviewed at two viewports and several states in a real browser | Agent-assisted rendered review | Screenshots and reviewer notes; human design approval left pending or explicitly simulated |
| Preference and consent behavior with synthetic configuration | Automated | Test results |
| Intent exercise: wording preserved, FRD derived, role findings synthesized into one decision | Agent-assisted | Sample files and findings |
| Cold start: a separate agent CLI, given only the repository, resumes one scoped task | Agent-assisted, independent session | Transcript of visible output, assessment against expected next action and gates |
| Diagrams rendered and inspected | Agent-assisted rendered review | SVG, PNG previews, verification record |
| Hosted CI at the final branch tip | Observed | Run URL and job results, reported per layer |

Missing capability is recorded as pending, not passed: for example, if no
independent agent session can run, FLOW-13 stays pending.
