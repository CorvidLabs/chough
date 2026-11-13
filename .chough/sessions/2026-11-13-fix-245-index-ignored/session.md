---
kind: session
id: 2026-11-13-fix-245-index-ignored
aliases: [fix-245-index-ignored]
objective: "Fix #245: chough library index links no source Git ignores, so a fresh clone's index check passes"
intent_scope: ""
modules: []
stage: test
stage_status: active
iteration: 1
callers: []
native_change: fix-245-chough-library-index-links-no-source-git-ignores-so-a-fresh-clone-s-index-check-passes
pr: ""
criteria: []
requirements: []
cases: []
design: []
files: []
observations:
  - 2026-11-13-tc-cli-index-links-no-source-git-ignores
  - 2026-11-13-tc-cli-index-links-no-source-git-ignores-dca64a
bugs: [2026-11-13-the-knowledge-index-links-a-skill-copy-git-ignores-so-library]
blockers: []
waiting_on_human: []
run_consents: []
next: Continue test
---
# Fix #245: chough library index links no source Git ignores, so a fresh clone's index check passes

## Goal

Fix #245: chough library index links no source Git ignores, so a fresh clone's index check passes

## Where it stands

## Decisions and why

- Leif decided "Approve fixing #245: chough library index stops linking skill copies that Git ignores": "Approve (Recommended)" (coordinator chat (Claude Code, hc t370), 2026-11-13T14:12Z, answering: 'Chough #245 (from corvid-agent): chough library index stops linking skill copies that Git ignores, so the index check passes in a fresh clone and in CI, where it fails today. Small wording change, with a failing test ready. Approve?' (option text: 't37f builds it. A separate warning in chough init would come later as its own change.'); relayed by claude-code-t370); recorded by claude-code-t382; a relayed claim, not authenticated
- Leif decided "Approve the scope of fix-245-chough-library-index-links-no-source-git-ignores-so-a-fresh-clone-s-index-check-passes": "Approve (Recommended)" (coordinator chat (Claude Code, hc t370), 2026-11-13T14:12Z, answering: 'Chough #245 (from corvid-agent): chough library index stops linking skill copies that Git ignores, so the index check passes in a fresh clone and in CI, where it fails today. Small wording change, with a failing test ready. Approve?' (option text: 't37f builds it. A separate warning in chough init would come later as its own change.'); relayed by claude-code-t370); recorded by claude-code-t382; a relayed claim, not authenticated; ran specsync change approve fix-245-chough-library-index-links-no-source-git-ignores-so-a-fresh-clone-s-index-check-passes as 0xLeif

## What was verified

- [Ran cargo test --locked --test vault -- --exact the_index_links_no_source_git_ignores_so_a_fresh_clone_checks_clean for TC-cli-index-links-no-source-git-ignores: failed (exit 101, tests run 1).](evidence/2026-11-13-tc-cli-index-links-no-source-git-ignores.md)
- [Ran cargo test --locked --test vault -- --exact the_index_links_no_source_git_ignores_so_a_fresh_clone_checks_clean for TC-cli-index-links-no-source-git-ignores: passed (exit 0, tests run 1).](evidence/2026-11-13-tc-cli-index-links-no-source-git-ignores-dca64a.md)

## Open threads

## Timeline

- 2026-11-13T14:13:02Z · start · intent → intent · Fix #245: chough library index links no source Git ignores, so a fresh clone's index check passes ^t-643827
  - revision: d2d2f60d5be07b6c43a827f576a3b974578af8b7
  - by: claude-code (hc t382)
- 2026-11-13T14:13:02Z · gate-opened · intent → intent · Approve fixing #245: chough library index stops linking skill copies that Git ignores ^t-c4cdfb
  - revision: d2d2f60d5be07b6c43a827f576a3b974578af8b7
  - holds: intent
  - drives: none
  - by: claude-code (hc t382)
- 2026-11-13T14:13:02Z · gate-closed · intent → intent · Approve fixing #245: chough library index stops linking skill copies that Git ignores ^t-797fe6
  - revision: d2d2f60d5be07b6c43a827f576a3b974578af8b7
  - relayed_by:
    - decided_by: Leif
    - words: Approve (Recommended)
    - where: coordinator chat (Claude Code, hc t370), 2026-11-13T14:12Z, answering: 'Chough #245 (from corvid-agent): chough library index stops linking skill copies that Git ignores, so the index check passes in a fresh clone and in CI, where it fails today. Small wording change, with a failing test ready. Approve?' (option text: 't37f builds it. A separate warning in chough init would come later as its own change.'); relayed by claude-code-t370
    - recorded_by: claude-code-t382
  - by: claude-code (hc t382)
- 2026-11-13T14:13:02Z · advance · intent → design · Leif approved the intent (14:12Z, relayed by t370) ^t-780060
  - revision: d2d2f60d5be07b6c43a827f576a3b974578af8b7
  - by: claude-code (hc t382)
- 2026-11-13T14:13:02Z · skip · design → plan · No visuals or interactions change ^t-8f1126
  - revision: d2d2f60d5be07b6c43a827f576a3b974578af8b7
  - by: claude-code (hc t382)
- 2026-11-13T14:13:47Z · gate-opened · plan → plan · Approve the scope of fix-245-chough-library-index-links-no-source-git-ignores-so-a-fresh-clone-s-index-check-passes ^t-4c589e
  - native_change: fix-245-chough-library-index-links-no-source-git-ignores-so-a-fresh-clone-s-index-check-passes
  - revision: 85304969e053e7477dff11d2ee53b6dddcb6a070
  - holds: plan
  - drives: approve
  - by: claude-code (hc t382)
- 2026-11-13T14:13:47Z · gate-closed · plan → plan · Approve the scope of fix-245-chough-library-index-links-no-source-git-ignores-so-a-fresh-clone-s-index-check-passes ^t-ef3036
  - native_change: fix-245-chough-library-index-links-no-source-git-ignores-so-a-fresh-clone-s-index-check-passes
  - revision: 85304969e053e7477dff11d2ee53b6dddcb6a070
  - relayed_by:
    - decided_by: Leif
    - words: Approve (Recommended)
    - where: coordinator chat (Claude Code, hc t370), 2026-11-13T14:12Z, answering: 'Chough #245 (from corvid-agent): chough library index stops linking skill copies that Git ignores, so the index check passes in a fresh clone and in CI, where it fails today. Small wording change, with a failing test ready. Approve?' (option text: 't37f builds it. A separate warning in chough init would come later as its own change.'); relayed by claude-code-t370
    - recorded_by: claude-code-t382
    - native: specsync change approve fix-245-chough-library-index-links-no-source-git-ignores-so-a-fresh-clone-s-index-check-passes
    - actor: 0xLeif
  - by: claude-code (hc t382)
- 2026-11-13T14:13:49Z · advance · plan → build · Leif approved the scope (14:12Z, relayed by t370) ^t-e7479d
  - native_change: fix-245-chough-library-index-links-no-source-git-ignores-so-a-fresh-clone-s-index-check-passes
  - gate: native:approved
  - revision: 85304969e053e7477dff11d2ee53b6dddcb6a070
  - by: claude-code (hc t382)
- 2026-11-13T14:59:33Z · advance · build → test · Built: the red and the fix, the cases red then green, the bug verified ^t-18648a
  - native_change: fix-245-chough-library-index-links-no-source-git-ignores-so-a-fresh-clone-s-index-check-passes
  - gate: native:approved
  - revision: f50a25cb2399fb57bf8074046d2662fb73e7d7ae
  - by: claude-code (hc t382)
