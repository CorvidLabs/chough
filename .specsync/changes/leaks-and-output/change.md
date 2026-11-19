---
id: leaks-and-output
state: verifying
type: bug_fix
base_commit: bd77bbd15f0e373776192a7d058a09c70fdbc00e
---

# Leaks and output

## Intent

leaks and output

## Affected Canonical Specs

- `records`
- `cli`
- `history`

## Acceptance Criteria

- checkpoint --resolve-question --answer is leak-checked, redacted under --redact and bounded at 4096 bytes (#364). Conflict-marker refusals and doctor findings name only the marker's seven characters (#365). An error shows a value a person gave only when it holds no leak and no control character, otherwise it names the field and what it holds (#367). The leak guard adds password JSON keys, SECRET_KEY and AWS_SECRET_ACCESS_KEY assignments, xapp- and hf_ tokens, ~user/ paths, the macOS LocalHostName and its .local name, dotted host names inside hyphenated build names, and checks session new --module; evidence add --redact reports its count (#368). The user name comes from the account, and a system root such as /dev/ is never a user-name path (#379). gate record's refusal says Error: once (#380).

## No-spec Rationale

Not applicable
