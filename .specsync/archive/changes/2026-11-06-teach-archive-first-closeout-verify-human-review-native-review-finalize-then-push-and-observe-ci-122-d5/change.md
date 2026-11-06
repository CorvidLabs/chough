---
id: teach-archive-first-closeout-verify-human-review-native-review-finalize-then-push-and-observe-ci-122-d5
state: archived
type: bug_fix
base_commit: d3b301e35fcf78430da3ca49b172ae00babaa657
---

# Teach archive-first closeout: verify, human review, native review, finalize, then push and observe CI (122, D5)

## Intent

Teach archive-first closeout: verify, human review, native review, finalize, then push and observe CI (122, D5)

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- Every place that teaches the closeout (bundled and installed lifecycle and accept references, docs/lifecycle.md, the walkthrough and the diagram) puts pushing and observing CI after local verification, the human implementation review, the native review and finalize, keeping the no-intervening-commit rule; installed 0.4.0 copies update; the exercise runs the corrected closeout against the archive-first inspector, which refuses the open change and passes once archived. The new test fails first, and fledge lanes run verify and the exercise pass on Linux.

## No-spec Rationale

Audit finding D5 (#122): the workflow guidance told agents to push and wait for CI before the human review and finalize, which the archive-first CI gate refuses; documentation and guidance fix, no requirement text changes.
