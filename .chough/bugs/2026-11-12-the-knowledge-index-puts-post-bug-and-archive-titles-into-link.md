---
kind: bug
id: 2026-11-12-the-knowledge-index-puts-post-bug-and-archive-titles-into-link
aliases:
  - BUG-the-knowledge-index-puts-post-bug-and-archive-titles-into-link-text-une-f7c4bba9d926154fd504513d
title: The knowledge index puts post, bug and archive titles into link text unescaped
status: verified
classification: product-defect
scope: in-scope
impact: Dead links and stray HTML in the reading guide
criteria: []
requirements: [REQ-cli-044]
cases: []
evidence:
  - OBS-tc-cli-knowledge-index-titles-inert-a7c1b95199bf50b68b19b227
  - OBS-tc-cli-knowledge-index-titles-inert-6b12badb8b8c281041a49d64
receipts:
  - id: 2026-11-12-tc-cli-knowledge-index-titles-inert
    aliases: OBS-tc-cli-knowledge-index-titles-inert-a7c1b95199bf50b68b19b227
    claim: "Ran cargo test --locked --test vault -- --exact index_titles_are_inert_markdown for TC-cli-knowledge-index-titles-inert: failed (exit 101)."
    result: failed
    command: cargo test --locked --test vault -- --exact index_titles_are_inert_markdown
    revision: 3b0bf3cab0ec736fc74ce9add391954d6e410c20
    environment: local-macos-arm64
    content: d3886369e15a507e84e99699fa6249625a098c2eb321ae006bf59051833133f3
    fingerprint: sha256:b2c10bbdc42f6b0e873c7044536671e6eb6b1f130791fa03968117b416c7bd70
    case: TC-cli-knowledge-index-titles-inert
    case_digest: 57bd03177d67aac4734363bdefd371892e98f8186c2b283f15833817b010abcd
    observed: 2026-11-12T02:02:32Z
    duration_seconds: 1
  - id: 2026-11-12-tc-cli-knowledge-index-titles-inert-3ce6d8
    aliases: OBS-tc-cli-knowledge-index-titles-inert-6b12badb8b8c281041a49d64
    claim: "Ran cargo test --locked --test vault -- --exact index_titles_are_inert_markdown for TC-cli-knowledge-index-titles-inert: passed (exit 0)."
    result: passed
    command: cargo test --locked --test vault -- --exact index_titles_are_inert_markdown
    revision: 0ee26ac0d72f466129525f27afaa5526cd55322e
    environment: local-macos-arm64
    content: cc056522bd209ed839bdb5d0a0b03f05210757d46be1edf71d7246a66228c3a2
    fingerprint: sha256:c41fca39a1b20ca4c17562cc2ebfa5733e80bd0f40491ac3fca7b14cf855d53e
    case: TC-cli-knowledge-index-titles-inert
    case_digest: 57bd03177d67aac4734363bdefd371892e98f8186c2b283f15833817b010abcd
    observed: 2026-11-12T02:17:43Z
    duration_seconds: 0
environment: local-macos-arm64, tester clone at 367dc86b, debug build, SpecSync 6.2.0
revision: d854ce1ac06a195e0c81d2aaf22d61bd64e2c556
found_by: t37f, test stage, area knowledge
related: []
created: 2026-11-12T01:38:48Z
updated: 2026-11-12T02:18:08Z
---
# The knowledge index puts post, bug and archive titles into link text unescaped

**What happened:** A journal title holding a Markdown link becomes its own dead link in index.md, ends the outer link text early, and raw HTML passes through
Actual: Titles are pasted into link text as written

**Steps:** A post, bug or archived change title with brackets, parentheses or angle brackets, then library index

**Expected:** Every relative Markdown link in the vault resolves (REQ-cli-044); the feed and the session writer already escape titles

**Why it matters:** Dead links and stray HTML in the reading guide

**Fix:** Recorded before 0.6.0; the Log keeps its history.

## Log

- 2026-11-12T01:38:48Z · Reported (t37f, test stage, area knowledge)
- 2026-11-12T02:02:52Z · status: open -> investigating. Red at 3b0bf3ca (TC-cli-knowledge-index-titles-inert, on t37x's branch before its fix): failed for this bug's reason; fix in 8d5c5c38.; evidence: OBS-tc-cli-knowledge-index-titles-inert-a7c1b95199bf50b68b19b227
- 2026-11-12T02:18:08Z · status: investigating -> verified. Green at 0ee26ac0: TC-cli-knowledge-index-titles-inert passed after the fix; full suite 626 passed, clippy and fmt clean at 7b645d60.; evidence: OBS-tc-cli-knowledge-index-titles-inert-6b12badb8b8c281041a49d64
