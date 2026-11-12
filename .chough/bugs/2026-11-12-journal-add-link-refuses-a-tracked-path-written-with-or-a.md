---
kind: bug
id: 2026-11-12-journal-add-link-refuses-a-tracked-path-written-with-or-a
aliases:
  - BUG-journal-add-link-refuses-a-tracked-path-written-with-or-a-fragment-sayi-c42412d03e5dfd12e4d28a9a
title: "journal add --link refuses a tracked path written with ./ or a #fragment, saying it is not a tracked path"
status: verified
classification: product-defect
scope: in-scope
impact: A misleading refusal for a valid link
criteria: []
requirements: [REQ-cli-044]
cases: []
evidence:
  - OBS-tc-cli-journal-link-dot-slash-and-fragment-115039dcdad143c8719c8fea
  - OBS-tc-cli-journal-link-dot-slash-and-fragment-88c631155d1b30dfcd655287
receipts:
  - id: 2026-11-12-tc-cli-journal-link-dot-slash-and-fragment
    aliases: OBS-tc-cli-journal-link-dot-slash-and-fragment-115039dcdad143c8719c8fea
    claim: "Ran cargo test --locked --test journal -- --exact journal_links_take_a_dot_slash_path_and_keep_a_fragment for TC-cli-journal-link-dot-slash-and-fragment: failed (exit 101)."
    result: failed
    command: cargo test --locked --test journal -- --exact journal_links_take_a_dot_slash_path_and_keep_a_fragment
    revision: e13c3bcb7ec275a4fd7b2dde9fcaa5dbe4809664
    environment: local-macos-arm64
    content: 7eb09bff741b568ef5d5eda07592ed4166cbf601b500288cba346986165b8540
    fingerprint: sha256:e428cde2716ae75d0d32e487de0f591d558db5ceff2105d22c64829e0ec9da8c
    case: TC-cli-journal-link-dot-slash-and-fragment
    case_digest: 50d20bea4934a4af84542e19fa1d86163837921f60d088fb2a058045c5b5437d
    observed: 2026-11-12T02:31:33Z
    duration_seconds: 0
  - id: 2026-11-12-tc-cli-journal-link-dot-slash-and-fragment-a7bd20
    aliases: OBS-tc-cli-journal-link-dot-slash-and-fragment-88c631155d1b30dfcd655287
    claim: "Ran cargo test --locked --test journal -- --exact journal_links_take_a_dot_slash_path_and_keep_a_fragment for TC-cli-journal-link-dot-slash-and-fragment: passed (exit 0)."
    result: passed
    command: cargo test --locked --test journal -- --exact journal_links_take_a_dot_slash_path_and_keep_a_fragment
    revision: 4038b7653b32337f202ab6a1061fbd723c9ef04b
    environment: local-macos-arm64
    content: d33a406fdec8a9ddf468c042c23d34503e7cb7514bbb345786c754937a942e96
    fingerprint: sha256:8ca402967fb7316f6b3837453a0ce686f96acf8a34a333a5551d3e8b7de7f3a5
    case: TC-cli-journal-link-dot-slash-and-fragment
    case_digest: 50d20bea4934a4af84542e19fa1d86163837921f60d088fb2a058045c5b5437d
    observed: 2026-11-12T02:43:29Z
    duration_seconds: 1
environment: local-macos-arm64, tester build at 0ee26ac0, debug, SpecSync 6.2.0
revision: 3558685321381f73f7180c0bedd11a0755f112e0
found_by: t38b, test stage, independent retest of K3 to K5
related: []
created: 2026-11-12T02:22:51Z
updated: 2026-11-12T02:43:30Z
---
# journal add --link refuses a tracked path written with ./ or a #fragment, saying it is not a tracked path

**What happened:** --link ./AGENTS.md and --link AGENTS.md#section are refused as neither a URL nor a project path Git tracks, though AGENTS.md is tracked
Actual: The literal string is checked against git ls-files

**Steps:** chough journal add --link ./<tracked file> or <tracked file>#<anchor>

**Expected:** A tracked file is accepted however a person writes the relative path, with the anchor kept on the link (REQ-cli-044)

**Why it matters:** A misleading refusal for a valid link

**Fix:** Recorded before 0.6.0; the Log keeps its history.

## Log

- 2026-11-12T02:22:51Z · Reported (t38b, test stage, independent retest of K3 to K5)
- 2026-11-12T02:31:48Z · status: open -> investigating. Red at e13c3bcb (t37f's red commit): failed for this bug's reason; fix in 8b39fd23.; evidence: OBS-tc-cli-journal-link-dot-slash-and-fragment-115039dcdad143c8719c8fea
- 2026-11-12T02:43:30Z · status: investigating -> verified. Green at 4038b765: TC-cli-journal-link-dot-slash-and-fragment passed; full suite 631 passed, clippy and fmt clean at 4038b765.; evidence: OBS-tc-cli-journal-link-dot-slash-and-fragment-88c631155d1b30dfcd655287
