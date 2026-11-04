---
id: stop-what-chough-started-when-chough-is-stopped-issue-75
state: archived
type: bug_fix
base_commit: 5b69b2f0ffee9c0a8f322e557421d8f4de8a3e07
---

# Stop what Chough started when Chough is stopped (issue 75)

## Intent

Stop what Chough started when Chough is stopped (issue 75)

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- On Unix SIGINT or SIGTERM or SIGHUP sent to chough during chough cases run stops the case command and its descendants and chough ends by that signal,The same signals during a native SpecSync read by chough next stop the native command and its descendants,An interrupted run records no observation and no artifact and leaves the checkpoint and the .chough directory byte-identical,A signal that chough inherited as ignored stays ignored and the deadline and held-output behavior of REQ-cli-011 is unchanged,docs/cli.md states the behavior and each new test fails on main edbc3c1 and fledge lanes run verify passes

## No-spec Rationale

Not applicable
