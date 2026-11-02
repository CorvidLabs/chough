---
change: align-native-actor-examples-in-user-documentation
artifact: context
---

# Context

PR44 already specifies agent-operated native workflow in REQ-cli-029. Peer review
found remaining human-actor placeholders in docs/cli.md and docs/walkthrough.md.
The deferred proposal in docs/NEXT_CHANGE.md has the same conditional example.
These examples contradict the existing acting-agent/human-source distinction; this
follow-up changes documentation only and introduces no new canonical behavior.

The actual human authorization is 0xLeif's standing chat instruction on 2026-11-02:
"I already gave it some approval and told it can do any changes. It just needs a PR."
The instruction and its Chough orchestration intent are relayed at
https://github.com/CorvidLabs/chough/issues/11#issuecomment-6581702299 .
The acting project agent account is corvid-agent. The covered scope is correcting
these three documentation examples within PR44; limits retain actual human
implementation acceptance and separate merge, release and deployment authority.
This account label does not itself provide human authorization or authentication.

This scope owns only docs/cli.md, docs/walkthrough.md and docs/NEXT_CHANGE.md.
The primary workflow scope retains tests/workflow_authority.rs and its expanded
structural coverage. The stage-reference/host-copy scope remains disjoint. Existing
native approvals, historical evidence, archives and cold-start material are
preserved. The deferred proposal stays unapproved history.
