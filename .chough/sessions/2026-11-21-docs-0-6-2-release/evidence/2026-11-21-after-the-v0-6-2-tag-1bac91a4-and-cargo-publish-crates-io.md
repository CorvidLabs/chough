---
kind: evidence
id: 2026-11-21-after-the-v0-6-2-tag-1bac91a4-and-cargo-publish-crates-io
aliases: []
claim: After the v0.6.2 tag (1bac91a4) and cargo publish (crates.io chough 0.6.2, 2026-11-21T00:29Z), cargo install chough --version 0.6.2 --locked into an empty root with a fresh target directory and cargo 1.100.0 on macOS arm64 exited 0; the installed chough --version printed 'chough 0.6.2'; in a new Git repository chough init and chough next exited 0, and chough status exited 0 once a session was open (with none open it says to run chough session new, exit 1). The chough in this Mac's cargo and local bin folders also prints 0.6.2.
result: passed
evidence_kind: structural
revision: 24243509d873755c4d794bea4822ac678d0753d3
content: f13e2dfe5a74d43bfc973382a7adef57d578b770807c57c2b93be1299cc9edad
environment: local-macos-arm64
criteria: [LIFE-25]
requirements: []
contracts: []
limitations: []
observed: 2026-11-21T00:33:22Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t85
---
**After the v0.6.2 tag (1bac91a4) and cargo publish (crates.io chough 0.6.2, 2026-11-21T00:29Z), cargo install chough --version 0.6.2 --locked into an empty root with a fresh target directory and cargo 1.100.0 on macOS arm64 exited 0; the installed chough --version printed 'chough 0.6.2'; in a new Git repository chough init and chough next exited 0, and chough status exited 0 once a session was open (with none open it says to run chough session new, exit 1). The chough in this Mac's cargo and local bin folders also prints 0.6.2.**

Recorded as passed in local-macos-arm64 at revision 24243509d873, observed 2026-11-21T00:33:22Z.
