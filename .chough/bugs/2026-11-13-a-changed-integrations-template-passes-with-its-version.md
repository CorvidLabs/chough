---
kind: bug
id: 2026-11-13-a-changed-integrations-template-passes-with-its-version
aliases: []
title: a changed integrations template passes with its version unchanged
status: open
classification: product-defect
scope: blocking
impact: A changed template could ship under an old version; found before merge
criteria: [LIFE-24]
requirements: [REQ-cli-065]
cases: []
evidence: [2026-11-13-integrations-templates/2026-11-13-tc-cli-integrations-manifest-1e9fe6]
receipts: []
environment: local-macos-arm64
revision: 81eccae15ccf4aac8ba89692cabb5aa59740b782
found_by: claude-code (hc t38r) (manual report)
actor:
  - agent: claude-code
    terminal: t38r
related: []
created: 2026-11-13T05:34:36Z
updated: 2026-11-13T06:07:38Z
---
# a changed integrations template passes with its version unchanged

**What happened:** the_manifest_versions_every_template_with_its_digest checks only version >= 1 and the digest, so updating a changed template's sha256 alone passes
Actual: The tests pass with the version left at 1 (t384)

**Steps:** Trigger: Change a template, update only its sha256 in integrations/templates.toml
1. Append a line to integrations/kimi/hc-template.md
2. Set its sha256 in templates.toml to the new digest and run cargo test --test integrations

**Expected:** REQ-cli-065 and the README: the tests fail until the template's digest and version are both updated

**Why it matters:** A changed template could ship under an old version; found before merge

**Fix:** Not fixed yet.

## Log

- 2026-11-13T05:34:36Z · Reported (manual report) · by claude-code (hc t38r)
- 2026-11-13T06:07:38Z · evidence: 2026-11-13-integrations-templates/2026-11-13-tc-cli-integrations-manifest-1e9fe6 · by claude-code (hc t38r)
