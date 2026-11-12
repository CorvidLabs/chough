---
kind: bug
id: 2026-11-02-identity-main-librarian-reference-is-mistaken-for-a-user-edit
aliases:
  - BUG-identity-main-librarian-reference-is-mistaken-for-a-user-edit-5c40a0c138d1a902783af838
title: Identity main librarian reference is mistaken for a user edit
status: verified
classification: product-defect
scope: in-scope
impact: Not yet assessed.
criteria: [LIFE-4]
requirements: [REQ-cli-016, REQ-cli-019]
cases: []
evidence:
  - OBS-product-test-bfc159865a39075249fb6e40
  - OBS-product-test-b0ea9516761a4e7adcf61406
  - OBS-product-test-691b9012d1892af5c103d51d
receipts:
  - id: 2026-11-02-the-new-identity-main-librarian-upgrade-regression-failed-the
    aliases: OBS-product-test-bfc159865a39075249fb6e40
    claim: "The new identity-main librarian upgrade regression failed: the exact unedited 4fa746a8 predecessor is reported edited instead of known-outdated, so the learning bundle cannot update it. Fledge exits 1 after the Rust test exits 101."
    result: failed
    revision: a1920f7f0ade695b718273a0ebfa7138e1805821
    environment: local-macos-arm64
    content: dbd7ac4955ff39aeb983c13994bdba67fc019bfd3108f53a1b4088d06adb08c3
    fingerprint: sha256:e173c84f233178c3ef79801ea43a02bff55a1823c76bec26591d0713a1249626
    observed: 2026-11-02T02:30:30Z
  - id: 2026-11-02-registering-the-exact-identity-main-librarian-predecessor
    aliases: OBS-product-test-b0ea9516761a4e7adcf61406
    claim: "Registering the exact identity-main librarian predecessor digest makes the new integration regression pass: unedited copies update and real user edits remain preserved."
    result: passed
    revision: a1920f7f0ade695b718273a0ebfa7138e1805821
    environment: local-macos-arm64
    content: fe7300eebcb4d452fa251e79ac937c56137d2baff26c6f66c7900f2926550eff
    fingerprint: sha256:83709182fd96eefa7fb3cce8420c678c5186e78de08d5a84d9b64048e2b45526
    observed: 2026-11-02T02:34:39Z
  - id: OBS-product-test-691b9012d1892af5c103d51d
    removed: .chough/archive/migrations/2026-11-12-from-0-5.md
environment: local-macos-arm64
revision: a1920f7f0ade695b718273a0ebfa7138e1805821
found_by: manual report
related: []
created: 2026-11-02T02:30:31Z
updated: 2026-11-02T03:48:33Z
---
# Identity main librarian reference is mistaken for a user edit

**What happened:** The unedited librarian reference shipped by the just-integrated identity change is classified edited, preventing its safe upgrade to the learning bundle.
Actual: The named integration regression gets edited instead of known-outdated at the first status assertion.

**Steps:** Install the exact identity-main librarian reference and run the learning helper skill status and update.
1. fledge run test -- --locked --test package -- --exact librarian_reference_from_identity_main_updates_without_overwriting_edits

**Expected:** Recognize the known predecessor, update it to current, and still preserve truly edited copies.

**Why it matters:** Not yet assessed.

**Fix:** Recorded before 0.6.0; the Log keeps its history.

## Log

- 2026-11-02T02:30:31Z · Reported (manual report); evidence: OBS-product-test-bfc159865a39075249fb6e40
- 2026-11-02T02:30:31Z · Added despite similar reports: This is a new predecessor digest introduced by the identity-main integration, not a user-edited bundle or the earlier checkpoint predecessor.
- 2026-11-02T02:34:40Z · status: open -> verified. The strictly later named regression passes after adding the missing identity-main predecessor digest. Real edited-copy preservation is asserted by the same test.; evidence: OBS-product-test-b0ea9516761a4e7adcf61406
- 2026-11-02T03:48:33Z · The final combined tree also passes all 194 tests on Rust 1.99.1 and 1.88.0, including the predecessor upgrade and edit-preservation regression.; evidence: OBS-product-test-691b9012d1892af5c103d51d
