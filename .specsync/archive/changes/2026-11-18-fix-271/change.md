---
id: fix-271
state: archived
type: bug_fix
base_commit: cef86e2622a2a908856b0c2cf11b29d3c1d3e472
---

# Fix 271

## Intent

fix 271

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- With two open sessions and the pointer on one, chough evidence add --session <other>, and the same command with CHOUGH_SESSION=<other>, record into the other session and leave the pointer unchanged, and chough next --session <other> shows that session. A --session or CHOUGH_SESSION naming an archived or unknown session refuses with every file unchanged. chough init --session and chough inbox add --session keep their meaning, and session new and session use ignore CHOUGH_SESSION. The five versioned hc templates in integrations/ tell the agent to export CHOUGH_SESSION for its session.

## No-spec Rationale

Not applicable
