---
change: external-artifacts-stay-machine-local-committed-records-keep-no-path-14
artifact: testing
---

# Testing

| Requirement | Coverage |
|---|---|
| REQ-cli-006 | `TC-cli-external-artifact-fingerprinted-in-place` binds `an_external_artifact_is_fingerprinted_in_place`: neutral ID, no path/file name/home in the record bytes, map entry and `.gitignore`, refusals, stale on change, `external-not-checkable-here` without the mapping |

Dogfood: a throwaway clone, an aged eligible observation with an external artifact through `evidence prune --apply`, and `git grep` of committed records for the path and user name.
