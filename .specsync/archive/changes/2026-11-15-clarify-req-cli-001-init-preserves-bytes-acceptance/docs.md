---
change: clarify-req-cli-001-init-preserves-bytes-acceptance
artifact: docs
---

# Docs

The change clarifies REQ-cli-001's acceptance line in
`specs/cli/requirements.md` to restate both preservation behaviors (repeated init
preserves edited session and skill bytes; missing bundle files can be added
without overwriting neighbors). A case is added in
`.chough/cases/cli.walk-ready.cases.toml`, and the changelog and `docs/cli.md`
are updated to note the clarification.
