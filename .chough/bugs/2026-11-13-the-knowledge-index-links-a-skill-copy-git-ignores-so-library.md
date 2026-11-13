---
kind: bug
id: 2026-11-13-the-knowledge-index-links-a-skill-copy-git-ignores-so-library
aliases: []
title: The knowledge index links a skill copy Git ignores, so library index --check fails in every fresh clone
status: open
classification: product-defect
scope: in-scope
impact: Every project that gitignores a host skill directory has an index check that can never pass in CI (seen in CorvidLabs/peck)
criteria: []
requirements: [REQ-cli-043, REQ-cli-044]
cases: []
evidence: [2026-11-13-fix-245-index-ignored/2026-11-13-tc-cli-index-links-no-source-git-ignores]
receipts: []
environment: local-macos-arm64
revision: 825447c42fc4a6e85dc6849af6f2384b9fdfc258
found_by: "claude-code (hc t382) (GitHub #245 (corvid-agent, adopting Chough 0.6.0 in CorvidLabs/peck))"
actor:
  - agent: claude-code
    terminal: t382
related: []
created: 2026-11-13T14:20:36Z
updated: 2026-11-13T14:50:05Z
---
# The knowledge index links a skill copy Git ignores, so library index --check fails in every fresh clone

**What happened:** In a repository whose .gitignore ignores .grok, chough init --host grok writes the skill to .grok/skills/chough-workflow/ and knowledge/index.md links it; in a fresh clone or CI, chough library index --check fails: the knowledge directory is not what its sources generate
Actual: src/vault.rs rule_files() links every host's <dir>/chough-workflow/SKILL.md that exists on disk, with no Git check

**Steps:** Trigger: git init, ignore .grok, commit, chough init --host grok, commit, git clone, chough library index --check in the clone
1. git init; printf '.grok\n' > .gitignore; commit
2. chough init --host grok; commit
3. git clone; chough library index --check fails

**Expected:** The index links only sources Git does not ignore, so the committed index passes --check in the working tree and in any clone

**Why it matters:** Every project that gitignores a host skill directory has an index check that can never pass in CI (seen in CorvidLabs/peck)

**Fix:** Not fixed yet.

## Log

- 2026-11-13T14:20:36Z · Reported (GitHub #245 (corvid-agent, adopting Chough 0.6.0 in CorvidLabs/peck)) · by claude-code (hc t382)
- 2026-11-13T14:20:36Z · Added despite similar reports: Distinct from #148's empty deltas folder: this is an ignored skill copy linked by the index · by claude-code (hc t382)
- 2026-11-13T14:50:05Z · evidence: 2026-11-13-fix-245-index-ignored/2026-11-13-tc-cli-index-links-no-source-git-ignores. Red at 825447c4 (t37f's test): a fresh clone's chough library index --check fails with the issue's exact error · by claude-code (hc t382)
