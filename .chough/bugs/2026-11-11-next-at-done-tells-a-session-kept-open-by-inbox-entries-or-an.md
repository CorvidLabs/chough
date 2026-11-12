---
kind: bug
id: 2026-11-11-next-at-done-tells-a-session-kept-open-by-inbox-entries-or-an
aliases:
  - BUG-next-at-done-tells-a-session-kept-open-by-inbox-entries-or-an-authorize-304f76f916333172c17967b4
title: next at done tells a session kept open by inbox entries or an authorized release to start a new session
status: verified
classification: product-defect
scope: in-scope
impact: An agent following next opens a second session while the first still holds unfolded entries or an authorized release, leaving it open and the release undone
criteria: [LIFE-1]
requirements: [REQ-cli-021]
cases: [TC-cli-next-at-done-kept-open]
evidence:
  - OBS-tc-cli-next-at-done-kept-open-8fdf64561e39e676b9d93935
  - OBS-tc-cli-next-at-done-kept-open-c238a23f65eb7fca532b2b16
receipts:
  - id: 2026-11-11-tc-cli-next-at-done-kept-open
    aliases: OBS-tc-cli-next-at-done-kept-open-8fdf64561e39e676b9d93935
    claim: "Ran cargo test --locked --test reduce -- --exact next_at_done_names_what_keeps_the_session_open for TC-cli-next-at-done-kept-open: failed (exit 101, tests run 1)."
    result: failed
    command: cargo test --locked --test reduce -- --exact next_at_done_names_what_keeps_the_session_open
    revision: 7881bfb9cc07c008ff618c9007f038ff5b9b6a21
    environment: local-macos-arm64
    content: c4579a7878f8c62e19039da7006a110dfa2693443f433cd12859fdbe33a28c64
    fingerprint: sha256:8ebb3243287fedf97f4cc0269ece283d4567e48065d68a5a59c29b56777d83eb
    case: TC-cli-next-at-done-kept-open
    case_digest: 5a53a9c70d4145686415e0aa3a3a88d2450a4ec88f9d5075d6672e2b04ab18cf
    observed: 2026-11-11T21:24:40Z
    duration_seconds: 1
  - id: 2026-11-11-tc-cli-next-at-done-kept-open-86e5af
    aliases: OBS-tc-cli-next-at-done-kept-open-c238a23f65eb7fca532b2b16
    claim: "Ran cargo test --locked --test reduce -- --exact next_at_done_names_what_keeps_the_session_open for TC-cli-next-at-done-kept-open: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test reduce -- --exact next_at_done_names_what_keeps_the_session_open
    revision: 9bfd7c48d78938726b6fd51106bd0a8d89671008
    environment: local-macos-arm64
    content: 79e82c7fa97795b462fd7b217146bd2ab7eef49c9f443b2213552791a73a515f
    fingerprint: sha256:b8bb86e311e6d148ece6e208a537c6c866d81e385607c400de60413a225fd92b
    case: TC-cli-next-at-done-kept-open
    case_digest: 5a53a9c70d4145686415e0aa3a3a88d2450a4ec88f9d5075d6672e2b04ab18cf
    observed: 2026-11-11T21:28:14Z
    duration_seconds: 2
environment: local-macos-arm64
revision: de3e953d60d9c94f0c34d5fbd1849bb8bb8c38e9
found_by: t37j's check of delta correction 1 against stage.rs done_action while drafting the delta corrections
related: []
created: 2026-11-11T21:24:11Z
updated: 2026-11-11T21:28:22Z
---
# next at done tells a session kept open by inbox entries or an authorized release to start a new session

**What happened:** After the move into done keeps the session open, because inbox entries are unfolded or release is authorized, chough next still names chough session new as the next command
Actual: next names chough session new <key> while the session is still open and active

**Steps:** Add an inbox entry (or grant release consent), advance into done, then run chough next
1. Advance a session to accept with its native change archived
2. Add an inbox entry without folding it, or grant release consent
3. Run chough stage advance, then chough next --json

**Expected:** next names chough inbox fold, then chough doctor --fix, for unfolded entries, and chough stage enter release for an authorized release; a new session only once the session has archived itself (REQ-cli-021 as delta correction 1 states it)

**Why it matters:** An agent following next opens a second session while the first still holds unfolded entries or an authorized release, leaving it open and the release undone

**Fix:** Recorded before 0.6.0; the Log keeps its history.

## Log

- 2026-11-11T21:24:11Z · Reported (t37j's check of delta correction 1 against stage.rs done_action while drafting the delta corrections)
- 2026-11-11T21:24:48Z · Red on the current done_action: TC-cli-next-at-done-kept-open fails, next names chough session new for a session kept open by an unfolded inbox entry; revision 7881bfb9 on leif/0.6.0-next-at-done-red.; evidence: OBS-tc-cli-next-at-done-kept-open-8fdf64561e39e676b9d93935
- 2026-11-11T21:28:22Z · status: open -> verified. Green on 9bfd7c48: TC-cli-next-at-done-kept-open passes. The test's third part (the archived session) expected a success before the fix and now expects the existing 'no open session' refusal naming chough session new; the red run failed in the first part, before reaching it.; evidence: OBS-tc-cli-next-at-done-kept-open-c238a23f65eb7fca532b2b16
