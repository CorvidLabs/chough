---
id: external-artifacts-stay-machine-local-committed-records-keep-no-path-14
state: implementing
type: feature
base_commit: 0b280cb22efa7595e4528304836e22141b8875d2
---

# External artifacts stay machine-local: committed records keep no path (14)

## Intent

External artifacts stay machine-local: committed records keep no path (14)

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- evidence add and cases record with an absolute artifact path commit only a neutral external:<16 hex> ID and the SHA-256 in the record, never the path, file name, user name or home directory; the path is kept only in the git-ignored .chough/local/external-artifacts.json; on the recording machine a changed file reads stale and an unchanged one current; where the mapping or the file is absent the observation reads external-not-checkable-here, neither current nor historical; evidence prune --apply never removes the external file; docs say outside files verify only on the recording machine.

## No-spec Rationale

Not applicable
