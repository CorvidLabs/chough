---
kind: bug
id: 2026-11-12-the-lesson-count-misses-dated-lessons-written-as-or-numbered
aliases:
  - BUG-the-lesson-count-misses-dated-lessons-written-as-or-numbered-list-items-5800d2e425f4b4354ac06315
title: The lesson count misses dated lessons written as + or numbered list items, or indented
status: verified
classification: product-defect
scope: in-scope
impact: Undercounted lessons for some writing styles
criteria: []
requirements: [REQ-cli-043]
cases: []
evidence:
  - OBS-tc-cli-knowledge-lesson-count-list-forms-6024a543bf0d2015655457d3
  - OBS-tc-cli-knowledge-lesson-count-list-forms-04a2fc4fa42190204f477e3e
receipts:
  - id: 2026-11-12-tc-cli-knowledge-lesson-count-list-forms
    aliases: OBS-tc-cli-knowledge-lesson-count-list-forms-6024a543bf0d2015655457d3
    claim: "Ran cargo test --locked --test vault -- --exact the_lesson_count_reads_plus_numbered_and_indented_items for TC-cli-knowledge-lesson-count-list-forms: failed (exit 101)."
    result: failed
    command: cargo test --locked --test vault -- --exact the_lesson_count_reads_plus_numbered_and_indented_items
    revision: e13c3bcb7ec275a4fd7b2dde9fcaa5dbe4809664
    environment: local-macos-arm64
    content: 7eb09bff741b568ef5d5eda07592ed4166cbf601b500288cba346986165b8540
    fingerprint: sha256:2e89e2a65ab479cece3734bd7ccb8d81e8a7f771a56d692bc6c915950c8875c3
    case: TC-cli-knowledge-lesson-count-list-forms
    case_digest: cefb2a86676a844255825ca585705551627a9c2779abf812c994d51052e35028
    observed: 2026-11-12T02:31:35Z
    duration_seconds: 0
  - id: 2026-11-12-tc-cli-knowledge-lesson-count-list-forms-753a78
    aliases: OBS-tc-cli-knowledge-lesson-count-list-forms-04a2fc4fa42190204f477e3e
    claim: "Ran cargo test --locked --test vault -- --exact the_lesson_count_reads_plus_numbered_and_indented_items for TC-cli-knowledge-lesson-count-list-forms: passed (exit 0)."
    result: passed
    command: cargo test --locked --test vault -- --exact the_lesson_count_reads_plus_numbered_and_indented_items
    revision: 4038b7653b32337f202ab6a1061fbd723c9ef04b
    environment: local-macos-arm64
    content: d33a406fdec8a9ddf468c042c23d34503e7cb7514bbb345786c754937a942e96
    fingerprint: sha256:c89feb02693b74135a9986739ddacf7425b92eae5ab0c1fcb3ddf142b92fef88
    case: TC-cli-knowledge-lesson-count-list-forms
    case_digest: cefb2a86676a844255825ca585705551627a9c2779abf812c994d51052e35028
    observed: 2026-11-12T02:43:32Z
    duration_seconds: 2
environment: local-macos-arm64, tester build at 0ee26ac0, debug, SpecSync 6.2.0
revision: 3864e43194583a70c97cae212194cf39a4d5a17d
found_by: t38b, test stage, independent retest of K3 to K5
related: []
created: 2026-11-12T02:23:09Z
updated: 2026-11-12T02:43:32Z
---
# The lesson count misses dated lessons written as + or numbered list items, or indented

**What happened:** '+ 2026-11-02 x', '1. 2026-11-03 x' and an indented '  - 2026-11-04 x' count 0
Actual: Only '- ' and '* ' bullets at the start of a line are read

**Steps:** Lessons written with a + bullet, a numbered list or indentation

**Expected:** The count of recorded lessons counts every list item that starts with a date (REQ-cli-043)

**Why it matters:** Undercounted lessons for some writing styles

**Fix:** Recorded before 0.6.0; the Log keeps its history.

## Log

- 2026-11-12T02:23:09Z · Reported (t38b, test stage, independent retest of K3 to K5)
- 2026-11-12T02:23:09Z · Added despite similar reports: K3 fixed the separator after the date for '- ' and '* ' items; this is the + and numbered list forms and indented items, which the K3 fix still does not read
- 2026-11-12T02:31:48Z · status: open -> investigating. Red at e13c3bcb (t37f's red commit): failed for this bug's reason; fix in 8b39fd23.; evidence: OBS-tc-cli-knowledge-lesson-count-list-forms-6024a543bf0d2015655457d3
- 2026-11-12T02:43:32Z · status: investigating -> verified. Green at 4038b765: TC-cli-knowledge-lesson-count-list-forms passed; full suite 631 passed, clippy and fmt clean at 4038b765.; evidence: OBS-tc-cli-knowledge-lesson-count-list-forms-04a2fc4fa42190204f477e3e
