---
kind: bug
id: 2026-11-11-committed-chough-records-can-leak-local-paths-names-and
aliases:
  - BUG-committed-chough-records-can-leak-local-paths-names-and-credentials-234-183650866477bd9df85aa8b9
title: Committed Chough records can leak local paths, names and credentials (#234)
status: verified
classification: product-defect
scope: in-scope
impact: "Personal and machine details reach the repository and its history (GitHub #234)"
criteria: [LIFE-26, FLOW-22]
requirements: [REQ-cli-007]
cases: [TC-records-leak-guard]
evidence:
  - OBS-tc-records-leak-guard-ed7a4af2c493f36279b90fd7
  - OBS-tc-records-leak-guard-cc49d0dfd3ef6736d6292aeb
receipts:
  - id: 2026-11-11-tc-records-leak-guard
    aliases: OBS-tc-records-leak-guard-ed7a4af2c493f36279b90fd7
    claim: "Ran cargo test --locked --test sessions --test bugs -- a_leak_in_a_session_value_is_refused_or_redacted bug_writes_refuse_or_redact_leaks for TC-records-leak-guard: failed (exit 101)."
    result: failed
    command: cargo test --locked --test sessions --test bugs -- a_leak_in_a_session_value_is_refused_or_redacted bug_writes_refuse_or_redact_leaks
    revision: 370d221d68840f2e263e90b927a200438a7a0173
    environment: local-macos-arm64
    content: 843a083f60a67703f38514e1fcffa147b0e3c500ec307ca879614ca5ee672a4c
    fingerprint: sha256:6066833c755b4c0d84845c5d94960964bf1d74ea45458a9abaa80a69109aa43c
    case: TC-records-leak-guard
    case_digest: 650f217b5ffa289bf5ddd5b9a09efa220e215b2dde5e943dc780f69f9221cb69
    observed: 2026-11-11T20:36:24Z
    duration_seconds: 0
  - id: 2026-11-11-tc-records-leak-guard-f843d4
    aliases: OBS-tc-records-leak-guard-cc49d0dfd3ef6736d6292aeb
    claim: "Ran cargo test --locked --test sessions --test bugs -- a_leak_in_a_session_value_is_refused_or_redacted bug_writes_refuse_or_redact_leaks for TC-records-leak-guard: passed (exit 0)."
    result: passed
    command: cargo test --locked --test sessions --test bugs -- a_leak_in_a_session_value_is_refused_or_redacted bug_writes_refuse_or_redact_leaks
    revision: 7b4cd3eb827f70c821255c0efb0326f17d7ca23a
    environment: local-macos-arm64
    content: e287cef970a2e70b5d716af6bc20f127df4d7275ee92871b760314f7393345e9
    fingerprint: sha256:e8869fc1ecbac04443876234454a1933167fd8b419bc8d6677de1c17591645d0
    case: TC-records-leak-guard
    case_digest: 650f217b5ffa289bf5ddd5b9a09efa220e215b2dde5e943dc780f69f9221cb69
    observed: 2026-11-11T20:38:49Z
    duration_seconds: 9
environment: local-macos-arm64
revision: ec9c3c422f9d169443945095115d5e0b0b5cf386
found_by: "GitHub issue #234; red on 0.5.0 at leif/0.6.0-batch-red 370d221d"
related: []
created: 2026-11-11T20:37:21Z
updated: 2026-11-11T20:39:42Z
---
# Committed Chough records can leak local paths, names and credentials (#234)

**What happened:** a home-folder path, user or host name, email or credential in a session value is written into a committed record
Actual: the value is written into the committed session record

**Steps:** Record a checkpoint decision whose text holds a path inside a home folder
1. chough init
2. chough checkpoint --decision with a home-folder path in its text

**Expected:** the write is refused naming the field and kind, or redacted with --redact, with every file unchanged on refusal

**Why it matters:** Personal and machine details reach the repository and its history (GitHub #234)

**Fix:** Recorded before 0.6.0; the Log keeps its history.

## Log

- 2026-11-11T20:37:21Z · Reported (GitHub issue #234; red on 0.5.0 at leif/0.6.0-batch-red 370d221d); evidence: OBS-tc-records-leak-guard-ed7a4af2c493f36279b90fd7
- 2026-11-11T20:39:42Z · status: open -> verified. Session and bug writes fixed in plans steps 3 and 5 (PR #238); the case passed after its 0.5.0 failure. The doctor scan and the verify lane's leaks step (t37j) join this case when they land.; evidence: OBS-tc-records-leak-guard-cc49d0dfd3ef6736d6292aeb
