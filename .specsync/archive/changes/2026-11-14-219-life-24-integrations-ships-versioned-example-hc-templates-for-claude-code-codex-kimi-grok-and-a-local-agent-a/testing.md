---
change: 219-life-24-integrations-ships-versioned-example-hc-templates-for-claude-code-codex-kimi-grok-and-a-local-agent-a
artifact: testing
---

# Testing

`tests/integrations.rs` checks the templates' front matter, bodies and paths, the
manifest's entries and digests, that `src/` and `skills/` neither start nor name
the tools, and the README. It is red at the test commit and green at the fix.
Chough cases bind each test through `cargo-test`.

## Requirement evidence

| Requirement | Evidence |
|---|---|
| REQ-cli-065 | `tests/integrations.rs`: five tests, one per acceptance criterion plus the README |
