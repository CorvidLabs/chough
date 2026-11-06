---
id: install-test-scripts-as-fresh-files-so-a-forked-child-holding-an-old-write-handle-cannot-make-them-busy-132
state: approved
type: bug_fix
base_commit: 47237d549eb395dfb8c301322d95f371ca4ea3ae
---

# Install test scripts as fresh files so a forked child holding an old write handle cannot make them busy (132)

## Intent

Install test scripts as fresh files so a forked child holding an old write handle cannot make them busy (132)

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- Test stand-in scripts are installed as fresh files from a child sh with a rename, so a write handle a forked child leaked never makes them busy; the new test fails first with ETXTBSY and passes after; 60 stress rounds at 32 threads show no Text file busy; no assertion, gate, bound or timeout changes; fledge lanes run verify passes on Linux.

## No-spec Rationale

Issue #132: ETXTBSY reproduced 2 of 60 stress rounds; test helpers only, no requirement text changes.
