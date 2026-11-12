---
kind: bug
id: 2026-11-12-docs-cli-md-places-library-inside-the-knowledge-directory-but
aliases:
  - BUG-docs-cli-md-places-library-inside-the-knowledge-directory-but-chough-re-91d5fa03b050747dc17844f9
title: docs/cli.md places library/ inside the knowledge directory, but Chough reads the repository-root library/
status: verified
classification: product-defect
scope: in-scope
impact: A person puts snapshots where Chough never reads them
criteria: []
requirements: [REQ-cli-043]
cases: []
evidence:
  - OBS-tc-cli-docs-name-the-root-library-folder-67078d9cb15d7549a39cb8f7
  - OBS-tc-cli-docs-name-the-root-library-folder-01404d01af0a09b9b5e8c1d7
receipts:
  - id: 2026-11-12-tc-cli-docs-name-the-root-library-folder
    aliases: OBS-tc-cli-docs-name-the-root-library-folder-67078d9cb15d7549a39cb8f7
    claim: "Ran cargo test --locked --test docs -- --exact the_knowledge_table_names_the_repository_root_library for TC-cli-docs-name-the-root-library-folder: failed (exit 101, tests run 1)."
    result: failed
    command: cargo test --locked --test docs -- --exact the_knowledge_table_names_the_repository_root_library
    revision: 5f167167fd07873c887741586ccf3f6e54f5c98a
    environment: local-macos-arm64
    content: 13091614c5d3c3e95b98e6e3e65d735513f1eab60e96102761dd290cf20b94fe
    fingerprint: sha256:32dd948d06081a66fe31f660c8695a145c693d8516c45d9c9117995c01cfe541
    case: TC-cli-docs-name-the-root-library-folder
    case_digest: 18c1c042ce77a0f6567e18b7d574016a76e1d6d7f05572ec5ba4cd30d6b1a50a
    observed: 2026-11-12T01:59:32Z
    duration_seconds: 0
  - id: 2026-11-12-tc-cli-docs-name-the-root-library-folder-9471d9
    aliases: OBS-tc-cli-docs-name-the-root-library-folder-01404d01af0a09b9b5e8c1d7
    claim: "Ran cargo test --locked --test docs -- --exact the_knowledge_table_names_the_repository_root_library for TC-cli-docs-name-the-root-library-folder: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test docs -- --exact the_knowledge_table_names_the_repository_root_library
    revision: 0ee26ac0d72f466129525f27afaa5526cd55322e
    environment: local-macos-arm64
    content: cc056522bd209ed839bdb5d0a0b03f05210757d46be1edf71d7246a66228c3a2
    fingerprint: sha256:f2f886c7c840971bcd835481c855856ec105793ef581c23715029df3f76cbcac
    case: TC-cli-docs-name-the-root-library-folder
    case_digest: 18c1c042ce77a0f6567e18b7d574016a76e1d6d7f05572ec5ba4cd30d6b1a50a
    observed: 2026-11-12T02:17:27Z
    duration_seconds: 1
environment: local-macos-arm64, tester clone at 367dc86b, debug build, SpecSync 6.2.0
revision: d854ce1ac06a195e0c81d2aaf22d61bd64e2c556
found_by: t37f, test stage, area knowledge
related: []
created: 2026-11-12T01:38:48Z
updated: 2026-11-12T02:18:07Z
---
# docs/cli.md places library/ inside the knowledge directory, but Chough reads the repository-root library/

**What happened:** The Knowledge vault table lists library/ among paths under knowledge/
Actual: The table says knowledge/library/

**Steps:** Reading docs/cli.md's knowledge table

**Expected:** The docs name the repository-root library/ that the index links

**Why it matters:** A person puts snapshots where Chough never reads them

**Fix:** Recorded before 0.6.0; the Log keeps its history.

## Log

- 2026-11-12T01:38:48Z · Reported (t37f, test stage, area knowledge)
- 2026-11-12T02:01:23Z · status: open -> investigating. Red at 5f167167 (leif/0.6.0-test1-red: d854ce1a plus the regression tests 558386f3): TC-cli-docs-name-the-root-library-folder failed for this bug's reason; fix in bc8ac154.; evidence: OBS-tc-cli-docs-name-the-root-library-folder-67078d9cb15d7549a39cb8f7
- 2026-11-12T02:18:07Z · status: investigating -> verified. Green at 0ee26ac0: TC-cli-docs-name-the-root-library-folder passed after the fix; full suite 626 passed, clippy and fmt clean at 7b645d60.; evidence: OBS-tc-cli-docs-name-the-root-library-folder-01404d01af0a09b9b5e8c1d7
