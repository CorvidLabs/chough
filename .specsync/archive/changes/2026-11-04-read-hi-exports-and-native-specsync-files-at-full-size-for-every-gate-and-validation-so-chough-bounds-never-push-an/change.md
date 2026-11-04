---
id: read-hi-exports-and-native-specsync-files-at-full-size-for-every-gate-and-validation-so-chough-bounds-never-push-an
state: archived
type: bug_fix
base_commit: 91a1fdd82a4e314e5037c6256f17199ef0d604f0
---

# Read HI exports and native SpecSync files at full size for every gate and validation so Chough bounds never push an agent to edit human intent or specs (issue 100)

## Intent

Read HI exports and native SpecSync files at full size for every gate and validation so Chough bounds never push an agent to edit human intent or specs (issue 100)

## Affected Canonical Specs

- `cli`
- `reporting`

## Acceptance Criteria

- chough evidence add and chough cases check validate a criterion against a HI family export larger than 16384 bytes and still refuse an unknown or retired criterion,chough evidence prune reads an archived finalization.json larger than 65536 bytes and the stage gates read a SpecSync policy larger than 65536 bytes,library index reads an archived change.md larger than 65536 bytes and timing reads approval ledgers larger than 65536 bytes,display truncation markers tell the reader to read the source in full and never to shorten it,the workflow skill states that a Chough size bound never justifies editing HI intent or criteria or specs or SpecSync files and that an oversized native source is a Chough bug to report,Chough records keep their bounds and each new test fails on main 91a1fdd and fledge lanes run verify passes

## No-spec Rationale

Not applicable
