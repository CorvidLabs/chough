---
id: keep-trimming-a-loose-artifact-group-that-gains-a-file-released-by-an-earlier-removal-109-closeout
state: implementing
type: bug_fix
base_commit: 6009d42756364fd1c338b215ac8539c1b2a0d0b0
---

# Keep trimming a loose artifact group that gains a file released by an earlier removal (109 closeout)

## Intent

Keep trimming a loose artifact group that gains a file released by an earlier removal (109 closeout)

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- A loose artifact group that gains a file because an earlier removal in the same apply released it is still trimmed: its planned files are removed and the released file stays; the re-check still refuses when a planned loose file becomes referenced or protected. The new test fails first, the trim on this repository then completes, and fledge lanes run verify passes on Linux.

## No-spec Rationale

Bug fix within REQ-cli-046: only planned files are removed and each is still loose; no requirement text changes.
