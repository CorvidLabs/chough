---
kind: bug
id: 2026-11-12-a-journal-post-s-own-heading-writes-its-title-raw-so-a-title
aliases:
  - BUG-a-journal-post-s-own-heading-writes-its-title-raw-so-a-title-can-add-a-463f7e7f0cc3606e22de1fa0
title: A journal post's own heading writes its title raw, so a title can add a dead link or raw HTML to the post
status: verified
classification: product-defect
scope: in-scope
impact: Dead links and stray HTML in posts
criteria: []
requirements: [REQ-cli-044]
cases: []
evidence:
  - OBS-tc-cli-journal-post-heading-inert-9d93f8dc67cab65b03f785fe
  - OBS-tc-cli-journal-post-heading-inert-23b996942afbf83ee77f8dae
receipts:
  - id: 2026-11-12-tc-cli-journal-post-heading-inert
    aliases: OBS-tc-cli-journal-post-heading-inert-9d93f8dc67cab65b03f785fe
    claim: "Ran cargo test --locked --test journal -- --exact a_posts_heading_holds_its_title_as_inert_markdown for TC-cli-journal-post-heading-inert: failed (exit 101)."
    result: failed
    command: cargo test --locked --test journal -- --exact a_posts_heading_holds_its_title_as_inert_markdown
    revision: e13c3bcb7ec275a4fd7b2dde9fcaa5dbe4809664
    environment: local-macos-arm64
    content: 7eb09bff741b568ef5d5eda07592ed4166cbf601b500288cba346986165b8540
    fingerprint: sha256:2b02d74bf9a0e653ff7228942dc197ae7e070b4431d95d5d6d23571f4432e6e8
    case: TC-cli-journal-post-heading-inert
    case_digest: 402ac711881013320d232737f372fbf2a8da501c2b4a080d2c0a28b0ad45606a
    observed: 2026-11-12T02:31:32Z
    duration_seconds: 1
  - id: 2026-11-12-tc-cli-journal-post-heading-inert-e3b925
    aliases: OBS-tc-cli-journal-post-heading-inert-23b996942afbf83ee77f8dae
    claim: "Ran cargo test --locked --test journal -- --exact a_posts_heading_holds_its_title_as_inert_markdown for TC-cli-journal-post-heading-inert: passed (exit 0)."
    result: passed
    command: cargo test --locked --test journal -- --exact a_posts_heading_holds_its_title_as_inert_markdown
    revision: 4038b7653b32337f202ab6a1061fbd723c9ef04b
    environment: local-macos-arm64
    content: d33a406fdec8a9ddf468c042c23d34503e7cb7514bbb345786c754937a942e96
    fingerprint: sha256:19e685ba8677c8068eb8d62330c6793f2a2691e8d093f984d84c87b393fd24d8
    case: TC-cli-journal-post-heading-inert
    case_digest: 402ac711881013320d232737f372fbf2a8da501c2b4a080d2c0a28b0ad45606a
    observed: 2026-11-12T02:43:28Z
    duration_seconds: 1
environment: local-macos-arm64, tester build at 0ee26ac0, debug, SpecSync 6.2.0
revision: 3864e43194583a70c97cae212194cf39a4d5a17d
found_by: t38b, test stage, independent retest of K3 to K5
related: []
created: 2026-11-12T02:23:08Z
updated: 2026-11-12T02:43:28Z
---
# A journal post's own heading writes its title raw, so a title can add a dead link or raw HTML to the post

**What happened:** journal add and the done and close post writers write '# <title>' unescaped; a title holding [x](y) or <b> gives the post a dead link and raw HTML
Actual: The heading is the raw title

**Steps:** A post title, or a session objective for a done or close post, holding brackets, parentheses or angle brackets

**Expected:** Every relative Markdown link in the vault resolves (REQ-cli-044); the index line now escapes the same title

**Why it matters:** Dead links and stray HTML in posts

**Fix:** Recorded before 0.6.0; the Log keeps its history.

## Log

- 2026-11-12T02:23:08Z · Reported (t38b, test stage, independent retest of K3 to K5)
- 2026-11-12T02:23:08Z · Added despite similar reports: K5 fixed the index's link text; this is the post file's own heading, written by journal add and the done and close post writers
- 2026-11-12T02:31:48Z · status: open -> investigating. Red at e13c3bcb (t37f's red commit): failed for this bug's reason; fix in 8b39fd23.; evidence: OBS-tc-cli-journal-post-heading-inert-9d93f8dc67cab65b03f785fe
- 2026-11-12T02:43:28Z · status: investigating -> verified. Green at 4038b765: TC-cli-journal-post-heading-inert passed; full suite 631 passed, clippy and fmt clean at 4038b765.; evidence: OBS-tc-cli-journal-post-heading-inert-23b996942afbf83ee77f8dae
