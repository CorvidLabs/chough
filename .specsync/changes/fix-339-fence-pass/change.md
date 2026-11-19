---
id: fix-339-fence-pass
state: implementing
type: bug_fix
base_commit: 082f1734397527910603c96624c5faf58f34bae0
---

# Fix 339 fence pass

## Intent

fix 339 fence pass

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- With a session body holding many code fence openers that never close, a record write and chough doctor finish in time that grows linearly with the record, not quadratically: each line is visited a bounded number of times. A conflict after those openers is still refused, since an unclosed fence hides nothing, and closed fences, Chough's own sections and the advisory findings behave as before.

## No-spec Rationale

REQ-records-001 already fixes which lines a code fence exempts and that an unclosed fence hides nothing; this change finds the same closes in a bounded pass instead of rescanning the rest of the text for every unclosed opener, so only code, a test and the changelog change.
