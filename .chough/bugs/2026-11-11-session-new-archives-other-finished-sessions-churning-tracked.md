---
kind: bug
id: 2026-11-11-session-new-archives-other-finished-sessions-churning-tracked
aliases:
  - BUG-session-new-archives-other-finished-sessions-churning-tracked-files-in-cf78a57afe8ff93b3f3dc576
title: session new archives other finished sessions, churning tracked files in an unrelated change (Q3)
status: verified
classification: product-defect
scope: blocking
impact: Unrelated changes carry moves of other sessions' records, which collide on a shared ledger and blur review; it is the side effect Q3 rules out.
criteria: [DRIVE-3]
requirements: [REQ-history-006]
cases: [TC-history-switch-archives-sessions-left-open]
evidence:
  - OBS-tc-history-switch-archives-sessions-left-open-098c034bb55f6a0dff1be113
  - OBS-tc-history-switch-archives-sessions-left-open-fd29c97343cd4876a5d059e4
  - OBS-tc-history-switch-archives-sessions-left-open-221373f4d5ac93cdedc42426
receipts:
  - id: 2026-11-11-tc-history-switch-archives-sessions-left-open-de2d34
    aliases: OBS-tc-history-switch-archives-sessions-left-open-fd29c97343cd4876a5d059e4
    claim: "Ran cargo test --locked --test sessions -- starting_or_switching_never_touches_another_session a_done_session_with_unfolded_entries_stays_open session_list_reports_an_archived_session_as_archived inbox_add_racing_an_archive_never_strands_its_entry for TC-history-switch-archives-sessions-left-open: failed (exit 101)."
    result: failed
    command: cargo test --locked --test sessions -- starting_or_switching_never_touches_another_session a_done_session_with_unfolded_entries_stays_open session_list_reports_an_archived_session_as_archived inbox_add_racing_an_archive_never_strands_its_entry
    revision: 45f4b2789cc1dab43547337dff26b7163f6b2518
    environment: local-macos-arm64
    content: 87cd8fd8f1ef4f5df8c03f3cdeb0fcb3db94c313cbdefe77f9745c5af4517d75
    fingerprint: sha256:27638ad631e920a96d2d0f26151fe20a3141d71c9f5714f13002c6f411e38c08
    case: TC-history-switch-archives-sessions-left-open
    case_digest: 11b818fe297f4cbfeb5209555b45e050eeec7af07a62ffb5dd8c10b70217cf28
    observed: 2026-11-11T18:28:00Z
    duration_seconds: 3
  - id: 2026-11-11-tc-history-switch-archives-sessions-left-open-4347a4
    aliases: OBS-tc-history-switch-archives-sessions-left-open-221373f4d5ac93cdedc42426
    claim: "Ran cargo test --locked --test sessions -- starting_or_switching_never_touches_another_session a_done_session_with_unfolded_entries_stays_open session_list_reports_an_archived_session_as_archived inbox_add_racing_an_archive_never_strands_its_entry for TC-history-switch-archives-sessions-left-open: passed (exit 0)."
    result: passed
    command: cargo test --locked --test sessions -- starting_or_switching_never_touches_another_session a_done_session_with_unfolded_entries_stays_open session_list_reports_an_archived_session_as_archived inbox_add_racing_an_archive_never_strands_its_entry
    revision: f24170802309a358c7228a80568124265b1bfc37
    environment: local-macos-arm64
    content: 6a119819cfaeede137332f015beb65961aa66ffae75ab49cf2fec896dd9164e5
    fingerprint: sha256:03b2bc65cfecdab3780f50647bd3b45e22c4431c82d1abc710c502477ad0cac2
    case: TC-history-switch-archives-sessions-left-open
    case_digest: 11b818fe297f4cbfeb5209555b45e050eeec7af07a62ffb5dd8c10b70217cf28
    observed: 2026-11-11T18:28:29Z
    duration_seconds: 3
environment: local-macos-arm64
revision: 4998db713ff96fc43bbfbb13a21d70e2c2745158
found_by: spec-sync repository, Chough 0.5.0, worker t38b; relayed by t370 on 2026-11-11
related: []
created: 2026-11-11T16:04:42Z
updated: 2026-11-11T18:28:37Z
---
# session new archives other finished sessions, churning tracked files in an unrelated change (Q3)

**What happened:** Starting a new session moved nine other tracked sessions it judged done from .chough/sessions/ into .chough/sessions/archive/, byte-identical, so an unrelated change carried tracked-file churn on a shared ledger.
Actual: Chough 0.5.0 archived every other finished session as a side effect of session new (spec-sync repository, seen by the spec-sync worker t38b, relayed by the coordinator on 2026-11-11).

**Steps:** chough session new <key> --objective ... --intent-scope CI --module github in a fresh worktree from origin/main of a repository whose .chough/sessions/ holds other committed sessions at done or with an archived native change.
1. In a repository whose .chough/sessions/ holds committed sessions that are done or whose native change is archived, create a fresh worktree from main.
2. Run chough session new <key> --objective <text> --intent-scope CI --module github.
3. git status shows those sessions moved from .chough/sessions/ to .chough/sessions/archive/.

**Expected:** Starting or switching work never closes another piece of work: session new and session use touch no other session's files; finished open sessions are listed with their fix, chough doctor --fix (Leif's default Q3, #207, REQ-history-006).

**Why it matters:** Unrelated changes carry moves of other sessions' records, which collide on a shared ledger and blur review; it is the side effect Q3 rules out.

**Fix:** Recorded before 0.6.0; the Log keeps its history.

## Log

- 2026-11-11T16:04:42Z · Reported (spec-sync repository, Chough 0.5.0, worker t38b; relayed by t370 on 2026-11-11)
- 2026-11-11T18:17:50Z · Fixed by plan step 3 (PR #238): session new and session use touch no other session; a session archives only itself. TC-history-switch-archives-sessions-left-open passed on local-macos-arm64, including a byte-for-byte snapshot of every other session before and after. No failed Chough observation is linked yet: the reproduction was in another repository on 0.5.0, so the bug stays unverified until a red run of this case on 0.5.0 code is recorded.; evidence: OBS-tc-history-switch-archives-sessions-left-open-098c034bb55f6a0dff1be113
- 2026-11-11T18:28:15Z · Red on 0.5.0: the same case binding run on branch leif/0.6.0-q3-red at 45f4b278 (step 1's 0.5.0 code plus a 0.5.x-layout test of the same name) failed: session new moved .chough/sessions/alpha.json into the archive. Recorded there with the 0.5.0 driver and copied here byte for byte.; evidence: OBS-tc-history-switch-archives-sessions-left-open-fd29c97343cd4876a5d059e4
- 2026-11-11T18:28:37Z · status: open -> verified. Green after the red run: TC-history-switch-archives-sessions-left-open passed again on the 0.6.0 branch at f2417080, observed after the 0.5.0 failure.; evidence: OBS-tc-history-switch-archives-sessions-left-open-221373f4d5ac93cdedc42426
