---
id: 0-6-1-chough-next-with-no-open-session-says-so-and-exits-0-and-timing-says-elapsed-wall-clock-once
state: archived
type: bug_fix
base_commit: 9d53604071df50b537adccad0ccf5cac9a2fc32b
---

# 0.6.1: chough next with no open session says so and exits 0, and timing says elapsed wall-clock once

## Intent

0.6.1: chough next with no open session says so and exits 0, and timing says elapsed wall-clock once

## Affected Canonical Specs

- `cli`
- `history`
- `reporting`

## Acceptance Criteria

- With no open session, chough next prints that there is no open session and names chough session new <key>, exiting 0, while chough status still refuses with that message; chough timing says elapsed wall-clock once, in its header, on no gate-wait, segment or estimate line, and its notes say read-only once (red then green)

## No-spec Rationale

Not applicable
