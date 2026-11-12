---
kind: bug
id: 2026-11-11-the-knowledge-vault-copies-records-instead-of-linking-them
aliases:
  - BUG-the-knowledge-vault-copies-records-instead-of-linking-them-with-no-jour-382fe8705d3959835e0f34a1
title: The knowledge vault copies records instead of linking them, with no journal or feed (#216)
status: verified
classification: product-defect
scope: in-scope
impact: "Two sources of truth that drift, and churn at every done (GitHub #216)"
criteria: [KNOW-33, KNOW-30]
requirements: [REQ-cli-044]
cases: [TC-records-journal-and-feed]
evidence:
  - OBS-tc-records-journal-and-feed-8bf0043fc731ac0f5b8c6663
  - OBS-tc-records-journal-and-feed-a1dff57a20a259ad7fccfaed
  - OBS-tc-records-tree-and-readme-fd25ddb1a81d344f37dc5f4c
receipts:
  - id: 2026-11-11-tc-records-journal-and-feed
    aliases: OBS-tc-records-journal-and-feed-8bf0043fc731ac0f5b8c6663
    claim: "Ran cargo test --locked --test journal -- --exact done_writes_a_short_post_and_the_feed_lists_it for TC-records-journal-and-feed: failed (exit 101, tests run 1)."
    result: failed
    command: cargo test --locked --test journal -- --exact done_writes_a_short_post_and_the_feed_lists_it
    revision: 370d221d68840f2e263e90b927a200438a7a0173
    environment: local-macos-arm64
    content: 843a083f60a67703f38514e1fcffa147b0e3c500ec307ca879614ca5ee672a4c
    fingerprint: sha256:5932692f4495327113983ffd2d5251aac8e87559928ba0911799c553d845c8f1
    case: TC-records-journal-and-feed
    case_digest: e841dbd34c7ad1f50eea02debd501bf9ef7e39e64f6b75256a5c650edec731c2
    observed: 2026-11-11T20:36:03Z
    duration_seconds: 1
  - id: 2026-11-11-tc-records-journal-and-feed-2a4cbc
    aliases: OBS-tc-records-journal-and-feed-a1dff57a20a259ad7fccfaed
    claim: "Ran cargo test --locked --test journal -- --exact done_writes_a_short_post_and_the_feed_lists_it for TC-records-journal-and-feed: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test journal -- --exact done_writes_a_short_post_and_the_feed_lists_it
    revision: 7b4cd3eb827f70c821255c0efb0326f17d7ca23a
    environment: local-macos-arm64
    content: e287cef970a2e70b5d716af6bc20f127df4d7275ee92871b760314f7393345e9
    fingerprint: sha256:c4ec74628357e8ef26edf172df992f32333b81bbb32de384712c60118842d5fd
    case: TC-records-journal-and-feed
    case_digest: e841dbd34c7ad1f50eea02debd501bf9ef7e39e64f6b75256a5c650edec731c2
    observed: 2026-11-11T20:38:55Z
    duration_seconds: 4
  - id: 2026-11-11-tc-records-tree-and-readme
    aliases: OBS-tc-records-tree-and-readme-fd25ddb1a81d344f37dc5f4c
    claim: "Ran cargo test --locked --test records -- --exact the_tree_holds_only_the_listed_paths for TC-records-tree-and-readme: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test records -- --exact the_tree_holds_only_the_listed_paths
    revision: 7b4cd3eb827f70c821255c0efb0326f17d7ca23a
    environment: local-macos-arm64
    content: e287cef970a2e70b5d716af6bc20f127df4d7275ee92871b760314f7393345e9
    fingerprint: sha256:ac34203b628e3ef1fecade56b5aebcb25a6759f2b070c29e6dbddb03320f3496
    case: TC-records-tree-and-readme
    case_digest: 7588a9b5564a8a8aff9f16e90895b27a2810355501f861fe6fd4c36af2a26250
    observed: 2026-11-11T20:39:07Z
    duration_seconds: 3
environment: local-macos-arm64
revision: 67d001e55e59fa271166d92c95ea44fac6d4f304
found_by: "GitHub issue #216; red on 0.5.0 at leif/0.6.0-batch-red 370d221d"
related: []
created: 2026-11-11T20:36:57Z
updated: 2026-11-12T00:12:37Z
---
# The knowledge vault copies records instead of linking them, with no journal or feed (#216)

**What happened:** the move into done writes no journal post or feed, and the vault regenerates copies of archives, sessions and bugs
Actual: no knowledge/journal/feed.xml after done

**Steps:** Advance a session to done with the librarian on
1. chough init
2. chough stage advance (six times, to done)

**Expected:** one short journal post per finished piece of work, a feed, and an index of links that copies no record (KNOW)

**Why it matters:** Two sources of truth that drift, and churn at every done (GitHub #216)

**Fix:** Recorded before 0.6.0; the Log keeps its history.

## Log

- 2026-11-11T20:36:57Z · Reported (GitHub issue #216; red on 0.5.0 at leif/0.6.0-batch-red 370d221d); evidence: OBS-tc-records-journal-and-feed-8bf0043fc731ac0f5b8c6663
- 2026-11-11T20:39:42Z · status: open -> verified. Fixed by plan step 8 (PR #238): index.md links, the journal post at done and the feed; the case passed after its 0.5.0 failure.; evidence: OBS-tc-records-journal-and-feed-a1dff57a20a259ad7fccfaed
- 2026-11-12T00:12:37Z · Also proved by TC-records-tree-and-readme, passed at 7b4cd3eb (the records tree's reading guide links hi/, specs/ and .specsync/archive/). The KNOW gap list check is a step 14 dogfood check.; evidence: OBS-tc-records-tree-and-readme-fd25ddb1a81d344f37dc5f4c
