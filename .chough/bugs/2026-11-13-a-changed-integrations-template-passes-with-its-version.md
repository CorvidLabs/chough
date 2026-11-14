---
kind: bug
id: 2026-11-13-a-changed-integrations-template-passes-with-its-version
aliases: []
title: a changed integrations template passes with its version unchanged
status: verified
classification: product-defect
scope: blocking
impact: A changed template could ship under an old version; found before merge
criteria: [LIFE-24]
requirements: [REQ-cli-065]
cases: []
evidence:
  - 2026-11-13-integrations-templates/2026-11-13-tc-cli-integrations-manifest-1e9fe6
  - 2026-11-13-integrations-templates/2026-11-13-tc-cli-integrations-manifest-51d9b1
receipts: []
environment: local-macos-arm64
revision: 81eccae15ccf4aac8ba89692cabb5aa59740b782
found_by: claude-code (hc t38r) (manual report)
actor:
  - agent: claude-code
    terminal: t38r
related: []
created: 2026-11-13T05:34:36Z
updated: 2026-11-13T06:20:02Z
---
# a changed integrations template passes with its version unchanged

**What happened:** the_manifest_versions_every_template_with_its_digest checks only version >= 1 and the digest, so updating a changed template's sha256 alone passes
Actual: The tests pass with the version left at 1 (t384)

**Steps:** Trigger: Change a template, update only its sha256 in integrations/templates.toml
1. Append a line to integrations/kimi/hc-template.md
2. Set its sha256 in templates.toml to the new digest and run cargo test --test integrations

**Expected:** REQ-cli-065 and the README: the tests fail until the template's digest and version are both updated

**Why it matters:** A changed template could ship under an old version; found before merge

**Fix:** Verified: 2026-11-13-integrations-templates/2026-11-13-tc-cli-integrations-manifest-51d9b1 passed at c0e3c340ae4fce048f37442831d242d42eeeef28.

## Log

- 2026-11-13T05:34:36Z · Reported (manual report) · by claude-code (hc t38r)
- 2026-11-13T06:07:38Z · evidence: 2026-11-13-integrations-templates/2026-11-13-tc-cli-integrations-manifest-1e9fe6 · by claude-code (hc t38r)
- 2026-11-13T06:20:02Z · status: open -> verified; evidence: 2026-11-13-integrations-templates/2026-11-13-tc-cli-integrations-manifest-51d9b1. Fixed at c0e3c340: templates.toml keeps an ordered versions list per template; the newest entry must be the file's digest, versions run 1, 2, 3 and no digest repeats. TC-cli-integrations-manifest red at 2cf5facb, green at c0e3c340; appending a byte to the Kimi template made it fail asking to append version 2 (manual mutation, restored). An in-place edit of the newest entry is not detectable without Git history, which the Product job's shallow checkout lacks · by claude-code (hc t38r)
