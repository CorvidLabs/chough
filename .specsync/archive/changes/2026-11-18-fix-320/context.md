---
change: fix-320
artifact: context
---

# Context

GitHub #320, a follow-up from the tester's check of #315. Since #315, a session that an older Chough let hold a token in an ID reference (criterion, requirement, case, observation, bug or native change) refuses every save, even with `--redact`, saying "remove it". No command removes such a reference, and no message names the way out: `chough session shelve <id>`, or a hand edit. On main too, `chough doctor --fix --redact` rewrites such an ID to `x-<redacted:token>`, which leaves the session unreadable (macOS swarm s6-F2/F8).

Leif's decision (2026-11-18, planning interview, recorded on #320 and relayed by hc t4c as the scope approval): approved as a fix. The refusal names the recovery (`chough session shelve <id>`, or a command that removes the reference), and `doctor --fix --redact` never rewrites an ID.

The refusal's new text fits REQ-records-006's refusal shape, so that part needs no wording. The doctor part changes REQ-history-012's `leak` fix, which said every match is redacted in place, and REQ-cli-055's `--fix`, since the doctor code is `cli`'s. Not in scope, and left on #320: `session new --module` still commits a leaking module name.
