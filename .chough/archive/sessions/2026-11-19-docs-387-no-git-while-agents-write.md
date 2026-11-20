---
kind: session
id: 2026-11-19-docs-387-no-git-while-agents-write
aliases: [docs-387-no-git-while-agents-write]
objective: The docs and the workflow skill tell agents not to run git merge, rebase, checkout or stash in a worktree while Chough writers run there, since Git can drop writes that already exited 0 (#387)
stage: done
stage_status: archived
iteration: 1
native_change: docs-387-no-git-while-agents-write
pr: 416
criteria: [DRIVE-11, DRIVE-3]
requirements: [REQ-cli-040]
observations:
  - 2026-11-20-tc-cli-the-docs-say-not-to-rewrite-the-worktree-while-another
  - 2026-11-20-tc-cli-the-docs-say-not-to-rewrite-the-worktree-while-another-6bd32f
title: No Git rewrites of a worktree while another agent works there
summary:
  - The workflow skill's checkpoint reference, in the bundle and in an installed copy, and docs/cli.md each say not to run git pull, merge, rebase, checkout or stash in a worktree while another agent works there, why (such a command can replace a record another agent's Chough command just saved), and that a lone agent may run one between its own commands; the 0.6.1 digest of the reference is known, so a 0.6.1 install updates (#387).
  - "Verified at 560a8780: all eleven verify steps and the case on macOS; tester2's PASS; corvid-agent's review at ddc210fb and recheck at 560a8780 (955 tests on Linux)."
closed: 2026-11-20
outcome: done
---
# The docs and the workflow skill tell agents not to run git merge, rebase, checkout or stash in a worktree while Chough writers run there, since Git can drop writes that already exited 0 (#387)

## Goal

The docs and the workflow skill tell agents not to run git merge, rebase, checkout or stash in a worktree while Chough writers run there, since Git can drop writes that already exited 0 (#387)

## Where it stands

## Decisions and why

- 0xLeif decided "Scope approval of docs-387-no-git-while-agents-write": "Yes I approve" (Relayed by Leif's planning session (hc t4c) through fable-lead (hc t53), from a Claude Code chat message on 2026-11-19 at about 14:37Z, on docs-387-no-git-while-agents-write at 0892b125: the one sentence and criterion in REQ-cli-040 and the skill digest in src/skill.rs, approved as drafted); recorded by fable-t4b; a relayed claim, not authenticated; ran specsync change approve docs-387-no-git-while-agents-write as 0xLeif
- 0xLeif decided "One send-back of docs-387-no-git-while-agents-write to correct its stale acceptance-criteria answer, run by t4b": "a, all yes.. im back" (Claude Code chat message relayed by hc t4c through fable-lead (hc t53), about 00:38Z on 2026-11-20, answering t4c's eight-item round; item 7: yes to one send-back of #387 before any code, run by t4b, correcting only the acceptance-criteria answer to match the delta he approved at 0892b125 (git pull named; a lone agent may run Git between its own commands), with no delta word changed); recorded by fable-t4b; a relayed claim, not authenticated; ran specsync change approve docs-387-no-git-while-agents-write as 0xLeif
- 0xLeif decided "Scope approval of docs-387-no-git-while-agents-write with its corrected acceptance-criteria answer (native approval)": "Yes, record it" (His direct answer in t4b's Claude Code session (AskUserQuestion), 2026-11-20, to recording this re-approval. Scope: his 'Yes I approve' of the delta at 0892b125 (relayed by hc t4c through fable-lead, 2026-11-19), delta unchanged. Send-back: his 'a, all yes.. im back' (relayed the same way, about 00:38Z on 2026-11-20, item 7), one send-back by t4b correcting only the acceptance-criteria answer); recorded by fable-t4b; a relayed claim, not authenticated; ran specsync change approve docs-387-no-git-while-agents-write as 0xLeif
- 0xLeif decided "Implementation review of docs-387-no-git-while-agents-write": "Yes, record and ship" (Leif's answer in t4b's Claude Code session (AskUserQuestion), 2026-11-20, to recording his implementation review of #416 at 560a8780 and shipping it. Under standing OK 6(b) (his 'Go' to the six rules, relayed by hc t4c): tester2's PASS, corvid-agent approved ddc210fb and 560a8780, no approved sentence changed, checked by fable-lead); recorded by fable-t4b; a relayed claim, not authenticated; ran specsync change review docs-387-no-git-while-agents-write as 0xLeif with verdict pass

## What was verified

- 1 earlier run of the cases below is kept in this story's receipts, not listed here.
- Ran cargo test --locked --test docs -- --exact the_docs_say_not_to_rewrite_the_worktree_while_another_agent_works for TC-cli-the-docs-say-not-to-rewrite-the-worktree-while-another-agent-works: passed (exit 0, tests run 1). - 251b9e6157a5; limits: inherited environment variables were not recorded ^2026-11-20-tc-cli-the-docs-say-not-to-rewrite-the-worktree-while-another-6bd32f

## Bugs

None.

## Open threads

## Timeline

- 2026-11-19T14:34:18Z · start · intent → intent · The docs and the workflow skill tell agents not to run git merge, rebase, checkout or stash in a worktree while Chough writers run there, since Git can drop writes that already exited 0 (#387) ^t-961b5f
  - revision: 9d260310bca8553a0203e5ef90bc8372b4e15aae
  - by: fable (claude-opus-5-5, hc t4b)
- 2026-11-19T14:34:18Z · advance · intent → design ·  ^t-d95e39
  - revision: 9d260310bca8553a0203e5ef90bc8372b4e15aae
  - by: fable (claude-opus-5-5, hc t4b)
- 2026-11-19T14:34:18Z · skip · design → plan · Guidance text in the docs and the skill; nothing visual or interactive changes ^t-9fb446
  - revision: 9d260310bca8553a0203e5ef90bc8372b4e15aae
  - by: fable (claude-opus-5-5, hc t4b)
- 2026-11-19T14:37:52Z · gate-opened · plan → plan · Scope approval of docs-387-no-git-while-agents-write ^t-49bda7
  - native_change: docs-387-no-git-while-agents-write
  - revision: 0892b125ba3a7c857efbb4f7c14f02ee290a3f5e
  - holds: plan
  - drives: approve
  - by: fable (claude-opus-5-5, hc t4b)
- 2026-11-19T14:37:53Z · gate-closed · plan → plan · Scope approval of docs-387-no-git-while-agents-write ^t-2c9611
  - native_change: docs-387-no-git-while-agents-write
  - revision: 0892b125ba3a7c857efbb4f7c14f02ee290a3f5e
  - relayed_by:
    - decided_by: 0xLeif
    - words: Yes I approve
    - where: Relayed by Leif's planning session (hc t4c) through fable-lead (hc t53), from a Claude Code chat message on 2026-11-19 at about 14:37Z, on docs-387-no-git-while-agents-write at 0892b125: the one sentence and criterion in REQ-cli-040 and the skill digest in src/skill.rs, approved as drafted
    - recorded_by: fable-t4b
    - native: specsync change approve docs-387-no-git-while-agents-write
    - actor: 0xLeif
  - by: fable (claude-opus-5-5, hc t4b)
- 2026-11-20T00:40:48Z · gate-opened · plan → plan · One send-back of docs-387-no-git-while-agents-write to correct its stale acceptance-criteria answer, run by t4b ^t-828be7
  - native_change: docs-387-no-git-while-agents-write
  - revision: 90f0c2263c81d737eace1d28583d9f2b9e372a2f
  - holds: plan
  - drives: approve
  - by: fable (claude-opus-5-5, hc t4b)
- 2026-11-20T00:40:48Z · gate-closed · plan → plan · One send-back of docs-387-no-git-while-agents-write to correct its stale acceptance-criteria answer, run by t4b ^t-dbaead
  - native_change: docs-387-no-git-while-agents-write
  - revision: 90f0c2263c81d737eace1d28583d9f2b9e372a2f
  - relayed_by:
    - decided_by: 0xLeif
    - words: a, all yes.. im back
    - where: Claude Code chat message relayed by hc t4c through fable-lead (hc t53), about 00:38Z on 2026-11-20, answering t4c's eight-item round; item 7: yes to one send-back of #387 before any code, run by t4b, correcting only the acceptance-criteria answer to match the delta he approved at 0892b125 (git pull named; a lone agent may run Git between its own commands), with no delta word changed
    - recorded_by: fable-t4b
    - native: specsync change approve docs-387-no-git-while-agents-write
    - actor: 0xLeif
  - by: fable (claude-opus-5-5, hc t4b)
- 2026-11-20T00:43:41Z · gate-opened · plan → plan · Scope approval of docs-387-no-git-while-agents-write with its corrected acceptance-criteria answer (native approval) ^t-6e0dd8
  - native_change: docs-387-no-git-while-agents-write
  - revision: 90f0c2263c81d737eace1d28583d9f2b9e372a2f
  - holds: plan
  - drives: approve
  - by: fable (claude-opus-5-5, hc t4b)
- 2026-11-20T00:43:41Z · direct-native-write · plan → plan · docs-387-no-git-while-agents-write's lifecycle records changed outside Chough: state.json changed, approvals.json changed; state approved → draft ^t-100594
  - native_change: docs-387-no-git-while-agents-write
  - revision: 90f0c2263c81d737eace1d28583d9f2b9e372a2f
  - direct_write:
    - change: docs-387-no-git-while-agents-write
    - files:
      - state.json: changed
      - approvals.json: changed
    - before_state: approved
    - after_state: draft
  - by: fable (claude-opus-5-5, hc t4b)
- 2026-11-20T00:43:41Z · gate-closed · plan → plan · Scope approval of docs-387-no-git-while-agents-write with its corrected acceptance-criteria answer (native approval) ^t-71bf21
  - native_change: docs-387-no-git-while-agents-write
  - revision: 90f0c2263c81d737eace1d28583d9f2b9e372a2f
  - relayed_by:
    - decided_by: 0xLeif
    - words: Yes, record it
    - where: His direct answer in t4b's Claude Code session (AskUserQuestion), 2026-11-20, to recording this re-approval. Scope: his 'Yes I approve' of the delta at 0892b125 (relayed by hc t4c through fable-lead, 2026-11-19), delta unchanged. Send-back: his 'a, all yes.. im back' (relayed the same way, about 00:38Z on 2026-11-20, item 7), one send-back by t4b correcting only the acceptance-criteria answer
    - recorded_by: fable-t4b
    - native: specsync change approve docs-387-no-git-while-agents-write
    - actor: 0xLeif
  - by: fable (claude-opus-5-5, hc t4b)
- 2026-11-20T00:46:00Z · advance · plan → build · native:approved ^t-a98dc9
  - native_change: docs-387-no-git-while-agents-write
  - gate: native:approved
  - revision: 347d9344a245aa3df6366cf2a63ff3903c900f56
  - by: fable (claude-opus-5-5, hc t4b)
- 2026-11-20T02:04:21Z · advance · build → test · native:verifying ^t-764b3a
  - native_change: docs-387-no-git-while-agents-write
  - gate: native:verifying
  - revision: 560a87805735b57704f82db8a2fdad9d8ca73673
  - by: fable (claude-opus-5-5, hc t4b)
- 2026-11-20T02:04:21Z · advance · test → accept · native:verifying ^t-a59f5a
  - native_change: docs-387-no-git-while-agents-write
  - gate: native:verifying
  - revision: 560a87805735b57704f82db8a2fdad9d8ca73673
  - by: fable (claude-opus-5-5, hc t4b)
- 2026-11-20T02:04:21Z · gate-opened · accept → accept · Implementation review of docs-387-no-git-while-agents-write ^t-494d6a
  - native_change: docs-387-no-git-while-agents-write
  - revision: 560a87805735b57704f82db8a2fdad9d8ca73673
  - holds: accept
  - drives: review
  - by: fable (claude-opus-5-5, hc t4b)
- 2026-11-20T02:16:28Z · gate-closed · accept → accept · Implementation review of docs-387-no-git-while-agents-write ^t-c6eea2
  - native_change: docs-387-no-git-while-agents-write
  - revision: acd64e321f218ae98b87ba0fb935b2af351ce3af
  - relayed_by:
    - decided_by: 0xLeif
    - words: Yes, record and ship
    - where: Leif's answer in t4b's Claude Code session (AskUserQuestion), 2026-11-20, to recording his implementation review of #416 at 560a8780 and shipping it. Under standing OK 6(b) (his 'Go' to the six rules, relayed by hc t4c): tester2's PASS, corvid-agent approved ddc210fb and 560a8780, no approved sentence changed, checked by fable-lead
    - recorded_by: fable-t4b
    - native: specsync change review docs-387-no-git-while-agents-write
    - actor: 0xLeif
  - by: fable (claude-opus-5-5, hc t4b)
- 2026-11-20T02:16:30Z · advance · accept → done · native:archived ^t-0f2dba
  - native_change: docs-387-no-git-while-agents-write
  - gate: native:archived
  - revision: acd64e321f218ae98b87ba0fb935b2af351ce3af
  - by: fable (claude-opus-5-5, hc t4b)

Closed 2026-11-20: done.

## Receipts

```yaml
receipts:
  - id: 2026-11-20-tc-cli-the-docs-say-not-to-rewrite-the-worktree-while-another
    claim: "Ran cargo test --locked --test docs -- --exact the_docs_say_not_to_rewrite_the_worktree_while_another_agent_works for TC-cli-the-docs-say-not-to-rewrite-the-worktree-while-another-agent-works: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test docs -- --exact the_docs_say_not_to_rewrite_the_worktree_while_another_agent_works
    revision: 705c7a97f66d1c1152f87392259ecec13f3ce5b3
    environment: local-macos-arm64
    content: 80426c9eac481c1e21a8963db755e9a4f4f3a234b502d8a9912113b0a7c4cf66
    fingerprint: sha256:19bd75f1e0742071771bfb46ffc6e923cac77d1c1d4907ade6345e00a18bbf7a
    case: TC-cli-the-docs-say-not-to-rewrite-the-worktree-while-another-agent-works
    case_digest: 564225552610950daaefdbc4ec93ef0dde54bff1caa408fe481de66a82d71c61
    criteria: DRIVE-11, DRIVE-3
    requirements: REQ-cli-040
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T00:57:22Z
    duration_seconds: 0
    actor: fable (claude-opus-5-5, hc t4b)
  - id: 2026-11-20-tc-cli-the-docs-say-not-to-rewrite-the-worktree-while-another-6bd32f
    claim: "Ran cargo test --locked --test docs -- --exact the_docs_say_not_to_rewrite_the_worktree_while_another_agent_works for TC-cli-the-docs-say-not-to-rewrite-the-worktree-while-another-agent-works: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test docs -- --exact the_docs_say_not_to_rewrite_the_worktree_while_another_agent_works
    revision: 37255266c625664b9a5c3091c37806083dd9c438
    environment: local-macos-arm64
    content: 8186b3e3919e28e50b043943bccd0cb5a8e90296f6805bf14cb73b86e8bcacf3
    fingerprint: sha256:251b9e6157a540247075521c5407e80d012991991fa49f74f86b1af78388c80b
    case: TC-cli-the-docs-say-not-to-rewrite-the-worktree-while-another-agent-works
    case_digest: 564225552610950daaefdbc4ec93ef0dde54bff1caa408fe481de66a82d71c61
    criteria: DRIVE-11, DRIVE-3
    requirements: REQ-cli-040
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T01:42:21Z
    duration_seconds: 0
    actor: fable (claude-opus-5-5, hc t4b)
```
