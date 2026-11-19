---
id: fix-g1-gates-and-pins-fail-closed
state: implementing
type: bug_fix
base_commit: 9d260310bca8553a0203e5ef90bc8372b4e15aae
---

# Fix g1 gates and pins fail closed

## Intent

fix g1 gates and pins fail closed

## Affected Canonical Specs

- `cli`
- `history`

## Acceptance Criteria

- A hand-edited blank, placeholder or doubled no_native_change is refused by every reader with the edit named, and session list and doctor still run; inbox list honors --session and CHOUGH_SESSION; session use and session new say when a pin still names another session; session shelve resolves a folder name, alias or bare slug; change ship closes its gate and moves its stage in the session it resolved; the ship history check covers the whole range and refuses when Git cannot read it; chough change refuses a leaking or control-character argument before SpecSync runs

## No-spec Rationale

Not applicable
