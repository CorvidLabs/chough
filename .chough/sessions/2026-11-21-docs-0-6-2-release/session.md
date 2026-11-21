---
kind: session
id: 2026-11-21-docs-0-6-2-release
aliases: [docs-0-6-2-release]
objective: The roadmap's delivered table has the 0.6.2 release row, and the README's command table says it documents 0.6.2
intent_scope: ""
modules: []
stage: design
stage_status: active
iteration: 1
callers: []
native_change: docs-the-0-6-2-release-row-and-the-readme-release-sentence
pr: ""
criteria: [LIFE-25]
requirements: []
cases: []
design: []
files: []
observations: [2026-11-21-after-the-v0-6-2-tag-1bac91a4-and-cargo-publish-crates-io]
bugs: []
blockers: []
waiting_on_human:
  - gate: Scope approval of docs-the-0-6-2-release-row-and-the-readme-release-sentence (native approval)
    holds: design
    drives: approve
    opened: 2026-11-21T00:33:10Z
run_consents: []
next: Read scoped intent and native status, then choose the next action within the authorized scope.
---
# The roadmap's delivered table has the 0.6.2 release row, and the README's command table says it documents 0.6.2

## Goal

The roadmap's delivered table has the 0.6.2 release row, and the README's command table says it documents 0.6.2

## Where it stands

## Decisions and why

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
