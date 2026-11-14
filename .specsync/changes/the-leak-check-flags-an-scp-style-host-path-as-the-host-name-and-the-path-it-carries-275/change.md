---
id: the-leak-check-flags-an-scp-style-host-path-as-the-host-name-and-the-path-it-carries-275
state: implementing
type: bug_fix
base_commit: 9b729e908adf30b6511531776f6976b1afd01395
---

# The leak check flags an scp-style host path as the host name and the path it carries (#275)

## Intent

The leak check flags an scp-style host path as the host name and the path it carries (#275)

## Affected Canonical Specs

- `records`

## Acceptance Criteria

- A committed value holding an scp-style remote path <host>:/<absolute path> is refused as a path leak when a home or system root follows the colon, and its host part is refused as a host-name leak when it is the local host, in the write guard, --redact and doctor's scan alike.

## No-spec Rationale

Not applicable
