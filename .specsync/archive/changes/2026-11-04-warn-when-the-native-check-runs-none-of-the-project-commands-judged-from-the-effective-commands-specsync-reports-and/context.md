---
change: warn-when-the-native-check-runs-none-of-the-project-commands-judged-from-the-effective-commands-specsync-reports-and
artifact: context
---

# Context

Issue https://github.com/CorvidLabs/chough/issues/85 (field test 2026-11-03): with no
project commands in native verification, `specsync change check` reports "verified"
while the project's `cargo test` fails, and `chough stage advance` reaches accept with
no evidence. Leif chose "Warn and point" on 2026-11-04 (FLOW-15); a gate was offered
and not chosen.

This replaces PR #89, closed unmerged by Leif's decision. #89 warned when
`.specsync/sdd.json` listed no `verification_commands`. The 0xLeif agent review showed,
and a probe on 2026-11-04 confirmed (CorvidLabs/spec-sync#848), that SpecSync 6.1.2's
`change check` does not run configured `verification_commands` at all: a configured
`sh -c "printf invoked > .native-command-probe; exit 9"` never ran and the check passed.
The change's effective commands, reported as `summary.verification_commands` by
`specsync change status --json` (the read `next` and `status` already make) and recorded
in its attempts, were only `specsync check --spec cli`. So the configuration is not
what runs; the effective list is. #89's change never reached main; correcting its
approved criteria in place was not possible (spec-sync#844).

Criteria: FLOW-15, FLOW-6, LIFE-7. Authority: Leif's 2026-11-04 standing approval for
bounded Chough bug fixes, his brief to fix issues 81 to 88 one PR each, his "Warn and
point" choice, and his choice to close #89 and redo it from main.
