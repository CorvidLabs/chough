---
id: a-host-label-equal-to-the-local-user-name-does-not-count-as-a-host-name-leak-as-a-plain-path-component-while-urls-user
state: archived
type: bug_fix
base_commit: 6e00b813a0d52be20a361b625cb96db03be171d6
---

# A host label equal to the local user name does not count as a host-name leak as a plain path component, while URLs, user@host and scp-style hosts keep matching (REQ-records-006)

## Intent

A host label equal to the local user name does not count as a host-name leak as a plain path component, while URLs, user@host and scp-style hosts keep matching (REQ-records-006)

## Affected Canonical Specs

- `records`

## Acceptance Criteria

- chough doctor on this Mac reports no host-name finding for a case path component equal to the user name; a record holding the same name in a URL, as <user>@<host> or as an scp-style host is still refused; cargo test passes with new leak tests for each of these.

## No-spec Rationale

Not applicable
