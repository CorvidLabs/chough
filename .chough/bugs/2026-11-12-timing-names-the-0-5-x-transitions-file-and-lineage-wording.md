---
kind: bug
id: 2026-11-12-timing-names-the-0-5-x-transitions-file-and-lineage-wording
aliases:
  - BUG-timing-names-the-0-5-x-transitions-file-and-lineage-wording-for-an-unre-cc7b97595721275d26d52f29
title: timing names the 0.5.x transitions file and lineage wording for an unreadable Timeline entry
status: verified
classification: product-defect
scope: in-scope
impact: A reader is sent to a file that does not exist
criteria: []
requirements: [REQ-reporting-001]
cases: []
evidence:
  - OBS-tc-reporting-timing-names-the-session-file-5ee6d0208f9d3c2431b3761b
  - OBS-tc-reporting-timing-names-the-session-file-d24444ae49320b9a5200846b
receipts:
  - id: 2026-11-12-tc-reporting-timing-names-the-session-file
    aliases: OBS-tc-reporting-timing-names-the-session-file-5ee6d0208f9d3c2431b3761b
    claim: "Ran cargo test --locked --test timing -- --exact an_unknown_timeline_kind_is_named_by_its_session_file for TC-reporting-timing-names-the-session-file: failed (exit 101, tests run 1)."
    result: failed
    command: cargo test --locked --test timing -- --exact an_unknown_timeline_kind_is_named_by_its_session_file
    revision: 5f167167fd07873c887741586ccf3f6e54f5c98a
    environment: local-macos-arm64
    content: 13091614c5d3c3e95b98e6e3e65d735513f1eab60e96102761dd290cf20b94fe
    fingerprint: sha256:44e39c9ed708e26ef0630bc757c682ff9b41bbd4651b1dd375558081d225cf75
    case: TC-reporting-timing-names-the-session-file
    case_digest: 153d89036c0988ddeb2bbb71fa25d46066f194d4b39444d6f8a41e7433e3afc5
    observed: 2026-11-12T01:59:30Z
    duration_seconds: 0
  - id: 2026-11-12-tc-reporting-timing-names-the-session-file-942e1c
    aliases: OBS-tc-reporting-timing-names-the-session-file-d24444ae49320b9a5200846b
    claim: "Ran cargo test --locked --test timing -- --exact an_unknown_timeline_kind_is_named_by_its_session_file for TC-reporting-timing-names-the-session-file: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test timing -- --exact an_unknown_timeline_kind_is_named_by_its_session_file
    revision: 0ee26ac0d72f466129525f27afaa5526cd55322e
    environment: local-macos-arm64
    content: cc056522bd209ed839bdb5d0a0b03f05210757d46be1edf71d7246a66228c3a2
    fingerprint: sha256:11488df5ada8419d609a9003ac2328e5515596950402449a89f166c326291910
    case: TC-reporting-timing-names-the-session-file
    case_digest: 153d89036c0988ddeb2bbb71fa25d46066f194d4b39444d6f8a41e7433e3afc5
    observed: 2026-11-12T02:17:23Z
    duration_seconds: 1
environment: local-macos-arm64, tester clone at 367dc86b, debug build, SpecSync 6.2.0
revision: 18025ad47c730af7589317f76608634b2487896f
found_by: t37j, test stage, areas sessions and bugs
related: []
created: 2026-11-12T01:41:12Z
updated: 2026-11-12T02:18:07Z
---
# timing names the 0.5.x transitions file and lineage wording for an unreadable Timeline entry

**What happened:** With one Timeline entry of an unknown kind, timing --json says 1 records of unrecognised kinds skipped from .chough/transitions.jsonl, which does not exist, and keeps the 0.5.x parent chain wording
Actual: The 0.5.x source and wording are printed

**Steps:** Hand-edit a Timeline entry's kind in session.md, then chough timing --json

**Expected:** A session or story that cannot be read is named by its own file (docs/cli.md Timing, REQ-reporting-001)

**Why it matters:** A reader is sent to a file that does not exist

**Fix:** Recorded before 0.6.0; the Log keeps its history.

## Log

- 2026-11-12T01:41:12Z · Reported (t37j, test stage, areas sessions and bugs)
- 2026-11-12T02:01:23Z · status: open -> investigating. Red at 5f167167 (leif/0.6.0-test1-red: d854ce1a plus the regression tests 558386f3): TC-reporting-timing-names-the-session-file failed for this bug's reason; fix in bc8ac154.; evidence: OBS-tc-reporting-timing-names-the-session-file-5ee6d0208f9d3c2431b3761b
- 2026-11-12T02:18:07Z · status: investigating -> verified. Green at 0ee26ac0: TC-reporting-timing-names-the-session-file passed after the fix; full suite 626 passed, clippy and fmt clean at 7b645d60.; evidence: OBS-tc-reporting-timing-names-the-session-file-d24444ae49320b9a5200846b
