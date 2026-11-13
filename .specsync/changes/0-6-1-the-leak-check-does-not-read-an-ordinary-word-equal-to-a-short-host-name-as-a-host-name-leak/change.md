---
id: 0-6-1-the-leak-check-does-not-read-an-ordinary-word-equal-to-a-short-host-name-as-a-host-name-leak
state: implementing
type: bug_fix
base_commit: 9d53604071df50b537adccad0ccf5cac9a2fc32b
---

# 0.6.1: the leak check does not read an ordinary word equal to a short host name as a host-name leak

## Intent

0.6.1: the leak check does not read an ordinary word equal to a short host name as a host-name leak

## Affected Canonical Specs

- `records`

## Acceptance Criteria

- With a short all-letter host name, a record using it as an ordinary word is written and chough doctor reports nothing, while the same name inside a path, a URL or as <user>@<host> is still refused; a host name of 8 or more characters, or one holding a digit, hyphen or dot, is still refused anywhere (red then green)

## No-spec Rationale

Not applicable
