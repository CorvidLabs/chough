---
kind: bug
id: 2026-11-11-the-move-into-done-regenerates-the-whole-knowledge-library
aliases:
  - BUG-the-move-into-done-regenerates-the-whole-knowledge-library-churning-tra-9e0f47c021a4aec725ee4b03
title: The move into done regenerates the whole knowledge library, churning tracked files in an unrelated change
status: verified
classification: product-defect
scope: blocking
impact: Unrelated changes carry dozens of generated knowledge files, which collide on a shared repository and blur review.
criteria: [KNOW-30, KNOW-31]
requirements: [REQ-cli-043]
cases: [TC-records-done-touches-only-its-own-post, TC-records-journal-and-feed]
evidence:
  - OBS-tc-records-done-touches-only-its-own-post-dda883ef8447da2d71961304
  - OBS-tc-records-done-touches-only-its-own-post-71ebc3909a0cce4378df2bc1
receipts:
  - id: 2026-11-12-tc-records-done-touches-only-its-own-post
    aliases: OBS-tc-records-done-touches-only-its-own-post-dda883ef8447da2d71961304
    claim: "Ran cargo test --locked --test journal -- --exact done_changes_no_knowledge_file_but_its_own_post_and_the_feed for TC-records-done-touches-only-its-own-post: failed (exit 101, tests run 1)."
    result: failed
    command: cargo test --locked --test journal -- --exact done_changes_no_knowledge_file_but_its_own_post_and_the_feed
    revision: bc8020474b181bbdbfc6671513016ecb1fd0bfb2
    environment: local-macos-arm64
    content: ccde5070f89f807682fe59e75ed34633c9b8d0d2c5b3cdd8a21533d45f2e20bc
    fingerprint: sha256:9dfef55cf4ae24386b9baf21fbeb551966666b2a3d07f391be39d7f23214b866
    case: TC-records-done-touches-only-its-own-post
    case_digest: cc93f3b2c7fd4e09c5cf4cb3a9d28b4784bf24f9fdfe248b216092e9675e731b
    observed: 2026-11-12T00:19:14Z
    duration_seconds: 1
  - id: 2026-11-12-tc-records-done-touches-only-its-own-post-1eb4a7
    aliases: OBS-tc-records-done-touches-only-its-own-post-71ebc3909a0cce4378df2bc1
    claim: "Ran cargo test --locked --test journal -- --exact done_changes_no_knowledge_file_but_its_own_post_and_the_feed for TC-records-done-touches-only-its-own-post: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test journal -- --exact done_changes_no_knowledge_file_but_its_own_post_and_the_feed
    revision: 54ed7c4cd8cfbbf8574a1b685b680bd25e767c27
    environment: local-macos-arm64
    content: f4706304eb513cc26319b7fb72787977781ccff31bfe4ba11e6a37108ffc82c5
    fingerprint: sha256:7d5260e54c66f8d55adf817c6e266c203b3070134f04f559c5d30b19ed400d57
    case: TC-records-done-touches-only-its-own-post
    case_digest: 356b2d6b5332add74202731b95dcb54c533480dea4f4dc51b3e2ec86691b22af
    observed: 2026-11-12T00:21:07Z
    duration_seconds: 1
environment: local-macos-arm64
revision: 279fc74f043b5b60a89913f687021f235d7ec501
found_by: spec-sync repository, Chough 0.5.0, worker t38b; relayed by t370 on 2026-11-11
related: []
created: 2026-11-11T16:37:45Z
updated: 2026-11-12T00:21:25Z
---
# The move into done regenerates the whole knowledge library, churning tracked files in an unrelated change

**What happened:** chough stage advance from accept to done modified 35 tracked files under knowledge/ and added 39, indexing 34 archived changes main had not indexed.
Actual: Chough 0.5.0 rebuilt archive, module, intent and timeline notes for every archived change at done (spec-sync repository, seen by the spec-sync worker t38b, relayed by the coordinator on 2026-11-11).

**Steps:** chough stage advance accept to done in a repository with many archived native changes and a knowledge library that is behind them.
1. In a repository whose knowledge library lags its archived native changes, advance a session from accept to done.
2. git status shows dozens of modified and new files under knowledge/.

**Expected:** The move into done writes only its own journal post and the feed entry; every other knowledge file stays byte-identical, because the library only links (KNOW-30, KNOW-31, KNOW-33).

**Why it matters:** Unrelated changes carry dozens of generated knowledge files, which collide on a shared repository and blur review.

**Fix:** Recorded before 0.6.0; the Log keeps its history.

## Log

- 2026-11-11T16:37:45Z · Reported (spec-sync repository, Chough 0.5.0, worker t38b; relayed by t370 on 2026-11-11)
- 2026-11-12T00:20:54Z · Red on 0.5.0 (t38b, bc802047 on leif/0.6.0-ledger-red): the move into done wrote two other knowledge files from the session roll-up and no journal post.; evidence: OBS-tc-records-done-touches-only-its-own-post-dda883ef8447da2d71961304
- 2026-11-12T00:21:25Z · status: open -> verified. Green on integration: tests/journal.rs done_changes_no_knowledge_file_but_its_own_post_and_the_feed passes; the move into done writes only its own post and the feed lines.; evidence: OBS-tc-records-done-touches-only-its-own-post-71ebc3909a0cce4378df2bc1
