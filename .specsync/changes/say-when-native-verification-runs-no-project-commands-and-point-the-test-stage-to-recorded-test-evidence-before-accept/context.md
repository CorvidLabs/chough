---
change: say-when-native-verification-runs-no-project-commands-and-point-the-test-stage-to-recorded-test-evidence-before-accept
artifact: context
---

# Context

Issue https://github.com/CorvidLabs/chough/issues/85 comes from a field test on
2026-11-03 (chough a33c747, SpecSync 6.1.2). After `specsync init` and `specsync change
adopt`, `.specsync/sdd.json` has `verification_commands: []`, so `specsync change
check` reports "verified" while the project's `cargo test` fails: the only command it
ran was `specsync check --spec <module>`. `chough stage advance` then moved build to
test to accept with no cases or observations, and at test `chough next` only said
"Continue test: Read scoped intent...". An agent following `next` exactly can reach
accept with a failing test suite.

Leif chose the behavior on 2026-11-04 in a Claude Code session, picking "Warn and
point" from offered options; it is captured as FLOW-15 with the wording he chose. A
gate that refuses test to accept without a passed observation ("Warn and gate") was
offered and not chosen, so this change adds no gate.

Authority: Leif's 2026-11-04 standing approval, given in the same session, for
bounded Chough bug fixes recorded by the agent under that authority, and his FLOW-15
choice above.

Out of scope: SpecSync's own behavior with empty `verification_commands` (it is
SpecSync's policy, not Chough's), test-stage design (DRIVE-6, #76 item 3) and any new
gate.
