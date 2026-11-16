---
id: clarify-req-cli-001-init-preserves-bytes-acceptance
state: archived
type: documentation
base_commit: cf2ff72c9f6f67cfa050cd92562dcc1b1b4f52c8
---

# Clarify REQ-cli-001 init-preserves-bytes acceptance

## Intent

Clarify REQ-cli-001 init-preserves-bytes acceptance

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- Repeated init preserves edited session and skill bytes; missing bundle files can be added without overwriting neighbors; a session with edited files stays byte-identical on later init.

## No-spec Rationale

Not applicable
