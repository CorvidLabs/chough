---
id: case-recording-order-the-shipped-skill-keeps-running-and-recording-chough-cases-out-of-tasks-md-checkboxes-and-runs-the
state: verifying
type: bug_fix
base_commit: 825a3805530456005e361dfa6d96d93eee52cd09
---

# Case recording order: the shipped skill keeps running and recording Chough cases out of tasks.md checkboxes and runs the scoped specsync change check first, so a case can link a requirement an approved change adds (BUG-0017, issue 11)

## Intent

Case recording order: the shipped skill keeps running and recording Chough cases out of tasks.md checkboxes and runs the scoped specsync change check first, so a case can link a requirement an approved change adds (BUG-0017, issue 11)

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- The shipped plan, testing and accept references say a requirement an approved change adds is linkable only after the scoped specsync change check materializes it, so that check runs before cases that link it are added, run or recorded
- The same references say running or recording Chough cases is not a tasks.md checkbox, because specsync change check refuses while a box is unchecked
- REQ-cli-005 and the chough cases check refusal are unchanged; Chough reads no native deltas
- Unedited copies of the plan, testing and accept references from main 825a380 read known-outdated and update; edited copies stay edited; both host copies in this repository read current
- A Linux run with SpecSync 6.1.2 shows the refusal with an open case-recording box and the passing order without it
- fledge lanes run verify passes locally; hosted CI is reported only as observed

## No-spec Rationale

Not applicable
