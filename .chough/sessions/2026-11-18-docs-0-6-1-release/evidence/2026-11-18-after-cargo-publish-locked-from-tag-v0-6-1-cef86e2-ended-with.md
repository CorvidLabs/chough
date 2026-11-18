---
kind: evidence
id: 2026-11-18-after-cargo-publish-locked-from-tag-v0-6-1-cef86e2-ended-with
aliases: []
claim: After cargo publish --locked from tag v0.6.1 (cef86e2) ended with 'Published chough v0.6.1 at registry crates-io', cargo install chough --version 0.6.1 --locked into an empty root with cargo 1.100.0 on macOS arm64 exited 0; chough --version printed 'chough 0.6.1'; chough init, next and status exited 0 in a new Git repository.
result: passed
evidence_kind: installation-observation
revision: 28b55ce1555d7bf9dc6a336143c0bf3e25534640
content: 581ea4e4e1db180bb1bb42d5bbb52ffa4f2de7ab1fe0af85fa028f5f4a50e249
environment: local-macos-arm64
criteria: [LIFE-25]
requirements: []
contracts: []
limitations: []
observed: 2026-11-18T17:56:34Z
actor:
  - agent: fable
    model: claude-fable-5-2
    terminal: t4b
---
**After cargo publish --locked from tag v0.6.1 (cef86e2) ended with 'Published chough v0.6.1 at registry crates-io', cargo install chough --version 0.6.1 --locked into an empty root with cargo 1.100.0 on macOS arm64 exited 0; chough --version printed 'chough 0.6.1'; chough init, next and status exited 0 in a new Git repository.**

Recorded as passed in local-macos-arm64 at revision 28b55ce1555d, observed 2026-11-18T17:56:34Z.
