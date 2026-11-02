# Lesson bundle: align-native-actor-examples-in-user-documentation

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Align native actor examples in user documentation
- **Kind**: Documentation
- **Paths**: docs/cli.md, docs/walkthrough.md, docs/NEXT_CHANGE.md
- **Acceptance**: The three current user documentation files show acting-agent native approve arguments and a human authority source, covered scope and limits note; existing applicable permission is used before asking a missing decision, actual implementation acceptance remains separate, historical proposal status is preserved, and the structural native-writer regression covers all three files while retaining bundle and installed-copy checks.

## Evidence

- Verification commit: `b24c881bd2b8f662a9a1b08d4a1fa11a0ae7a6c5`
- Base commit: `9c41e95949adcb1009c986f97db50faa593d81b2`
- Verified by: `specsync check (no spec in scope)`

## From the change's context.md

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

## Where these lessons go

This change declared no affected specs, so there is no module context to fold into.
