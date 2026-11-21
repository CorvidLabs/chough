---
kind: session
id: 2026-11-21-docs-0-6-2-release
aliases: [docs-0-6-2-release]
objective: The roadmap's delivered table has the 0.6.2 release row, and the README's command table says it documents 0.6.2
intent_scope: ""
modules: []
stage: accept
stage_status: active
iteration: 1
callers: []
native_change: docs-the-0-6-2-release-row-and-the-readme-release-sentence
pr: "441"
criteria: [LIFE-25]
requirements: []
cases: []
design: []
files: []
observations: [2026-11-21-after-the-v0-6-2-tag-1bac91a4-and-cargo-publish-crates-io]
bugs: []
blockers: []
waiting_on_human:
  - gate: Implementation review of docs-the-0-6-2-release-row-and-the-readme-release-sentence
    holds: accept
    drives: review
    opened: 2026-11-21T01:11:25Z
run_consents: []
next: Continue accept
title: The documentation checked against Chough 0.6.2
summary:
  - "The README and all documentation, the Mermaid diagrams included, were checked against what Chough 0.6.2 does and every inaccuracy fixed: the closeout commits the session folder with the story, the lifecycle diagram's release path, refused:no-human-decision and --reason, what closes a session, temp files, leak scope and stale claims; the lifecycle and ownership SVGs re-rendered; the roadmap's 0.6.2 row; an installed 0.6.2 skill updates. Code bugs filed: #433 to #440."
  - "Verified at 417e1cc7 on macOS with SpecSync 6.4.0: all eleven verify steps (57 test binaries), the docs tests on diagrams and labels, and the skill-release test."
---
# The roadmap's delivered table has the 0.6.2 release row, and the README's command table says it documents 0.6.2

## Goal

The roadmap's delivered table has the 0.6.2 release row, and the README's command table says it documents 0.6.2

## Where it stands

## Decisions and why

- 0xLeif decided "Scope approval of docs-the-0-6-2-release-row-and-the-readme-release-sentence (native approval)": "Standing OK, tell me after" (Leif's standing OK for scope approvals ('Standing OK, tell me after', a Claude Code question dialog in his planning session hc t4c, 2026-11-20 about 04:17Z), confirmed by t4c for this documentation-only change: no spec wording, no refusal. Its scope is Leif's own ask, relayed by t4c on 2026-11-21: the README and all documentation, Mermaid diagrams included, checked and made accurate for 0.6.2, code bugs filed not fixed (#433 to #440); with the roadmap's 0.6.2 row under his 'Yes, release 0.6.2' of 2026-11-20 about 23:59Z. src/skill.rs and the skill-release fixture change only so an installed 0.6.2 skill updates to the corrected files. Recorded by t4c); recorded by t4c; a relayed claim, not authenticated; ran specsync change approve docs-the-0-6-2-release-row-and-the-readme-release-sentence as 0xLeif

## What was verified

- [After the v0.6.2 tag (1bac91a4) and cargo publish (crates.io chough 0.6.2, 2026-11-21T00:29Z), cargo install chough --version 0.6.2 --locked into an empty root with a fresh target directory and cargo 1.100.0 on macOS arm64 exited 0; the installed chough --version printed 'chough 0.6.2'; in a new Git repository chough init and chough next exited 0, and chough status exited 0 once a session was open (with none open it says to run chough session new, exit 1). The chough in this Mac's cargo and local bin folders also prints 0.6.2.](evidence/2026-11-21-after-the-v0-6-2-tag-1bac91a4-and-cargo-publish-crates-io.md)

## Open threads

## Timeline

- 2026-11-21T00:32:34Z · start · intent → intent · The roadmap's delivered table has the 0.6.2 release row, and the README's command table says it documents 0.6.2 ^t-424390
  - revision: 1bac91a4136691074c731f6fc24fd06f57af45d6
  - by: fable (claude-opus-5-5, hc t85)
- 2026-11-21T00:32:34Z · advance · intent → design ·  ^t-5ace7a
  - revision: 1bac91a4136691074c731f6fc24fd06f57af45d6
  - by: fable (claude-opus-5-5, hc t85)
- 2026-11-21T00:33:10Z · gate-opened · design → design · Scope approval of docs-the-0-6-2-release-row-and-the-readme-release-sentence (native approval) ^t-b53ada
  - native_change: docs-the-0-6-2-release-row-and-the-readme-release-sentence
  - revision: 99d5d2cb694f7aed7857442072174d4b5e314ee4
  - holds: design
  - drives: approve
  - by: fable (claude-opus-5-5, hc t85)
- 2026-11-21T00:58:53Z · gate-closed · design → design · Scope approval of docs-the-0-6-2-release-row-and-the-readme-release-sentence (native approval) ^t-a35af9
  - native_change: docs-the-0-6-2-release-row-and-the-readme-release-sentence
  - revision: f6bd0d69ac69b52efc86ac08fa12b75505fb36d6
  - relayed_by:
    - decided_by: 0xLeif
    - words: Standing OK, tell me after
    - where: Leif's standing OK for scope approvals ('Standing OK, tell me after', a Claude Code question dialog in his planning session hc t4c, 2026-11-20 about 04:17Z), confirmed by t4c for this documentation-only change: no spec wording, no refusal. Its scope is Leif's own ask, relayed by t4c on 2026-11-21: the README and all documentation, Mermaid diagrams included, checked and made accurate for 0.6.2, code bugs filed not fixed (#433 to #440); with the roadmap's 0.6.2 row under his 'Yes, release 0.6.2' of 2026-11-20 about 23:59Z. src/skill.rs and the skill-release fixture change only so an installed 0.6.2 skill updates to the corrected files. Recorded by t4c
    - recorded_by: t4c
    - native: specsync change approve docs-the-0-6-2-release-row-and-the-readme-release-sentence
    - actor: 0xLeif
  - by: t4c (claude-opus-5-5, hc t4c)
- 2026-11-21T00:58:58Z · advance · design → plan ·  ^t-5dae59
  - native_change: docs-the-0-6-2-release-row-and-the-readme-release-sentence
  - revision: 853add98bd9110f120a0ef7fdf1bcb74de762cdc
  - by: fable (claude-opus-5-5, hc t85)
- 2026-11-21T00:58:58Z · advance · plan → build · native:approved ^t-8f5ad5
  - native_change: docs-the-0-6-2-release-row-and-the-readme-release-sentence
  - gate: native:approved
  - revision: 853add98bd9110f120a0ef7fdf1bcb74de762cdc
  - by: fable (claude-opus-5-5, hc t85)
- 2026-11-21T01:11:25Z · advance · build → test · native:verifying ^t-6ad03a
  - native_change: docs-the-0-6-2-release-row-and-the-readme-release-sentence
  - gate: native:verifying
  - revision: 5b680fa2c6ff71cbee9cfe18782effaaa517a5af
  - by: t4c (claude-opus-5-5, hc t4c)
- 2026-11-21T01:11:25Z · advance · test → accept · native:verifying ^t-32c2ec
  - native_change: docs-the-0-6-2-release-row-and-the-readme-release-sentence
  - gate: native:verifying
  - revision: 5b680fa2c6ff71cbee9cfe18782effaaa517a5af
  - by: t4c (claude-opus-5-5, hc t4c)
- 2026-11-21T01:11:25Z · gate-opened · accept → accept · Implementation review of docs-the-0-6-2-release-row-and-the-readme-release-sentence ^t-270d18
  - native_change: docs-the-0-6-2-release-row-and-the-readme-release-sentence
  - revision: 5b680fa2c6ff71cbee9cfe18782effaaa517a5af
  - holds: accept
  - drives: review
  - by: t4c (claude-opus-5-5, hc t4c)
