---
kind: session
id: 2026-11-11-human-first-records-0-6-0
aliases: [human-first-records-0-6-0]
objective: "Ship human-first Chough records as one change, Chough 0.6.0: readable one-file Markdown records, date+slug session folders with agent/ working files, one-file archived stories, journal and feed, one source of truth, no leaks, gate approvals as the approving human; dogfooded on the chough repo only"
intent_scope: DRIVE
modules: [cli, reporting, history]
stage: accept
stage_status: active
iteration: 1
callers: []
native_change: chough-0-6-0-human-first-records-one-markdown-file-with-yaml-front-matter-per-record-date-slug-session-folders-with-an
pr: ""
criteria: []
requirements: []
cases: []
design: [design/prototype/index.html, design/prototype/COMPANION.md]
files: []
observations:
  - OBS-tc-history-165-earlier-archives-stay-e355ee5b96eaa89fb762f797
  - OBS-tc-history-switch-archives-sessions-left-open-098c034bb55f6a0dff1be113
bugs: []
blockers: []
waiting_on_human: []
run_consents: []
next: Test on SpecSync 6.3.0 is green once hosted CI passes at the final check commit; send t370 the acceptance review summary for Leif. Acceptance is Leif's review at accept; record nothing for him and tag nothing.
---
# Ship human-first Chough records as one change, Chough 0.6.0: readable one-file Markdown records, date+slug session folders with agent/ working files, one-file archived stories, journal and feed, one source of truth, no leaks, gate approvals as the approving human; dogfooded on the chough repo only

## Goal

Ship human-first Chough records as one change, Chough 0.6.0: readable one-file Markdown records, date+slug session folders with agent/ working files, one-file archived stories, journal and feed, one source of truth, no leaks, gate approvals as the approving human; dogfooded on the chough repo only

## Where it stands

- Plan step 3 (sessions): session folders, the Timeline in session.md, finish into one story, no roll-up, Markdown inbox, leak guard; fmt, clippy --all-targets -D warnings and the full suite (561 passed, 11 ignored for steps 6, 8 and 10) passed at c1d75ceb

## Decisions and why

- Run one Chough cycle on chough itself for human-first records, because Leif wants to see the design before one big change
- Ship the human-first redesign as ONE change, released as Chough 0.6.0, dogfooded on the chough repo only; update the other repos only after 0.6.0 is released at >=95% confidence, because Leif chose 'all at once for chough 0.6.0 and only use it for chough itself' (2026-11-10)
- Windows: FLOW-17 and FLOW-21 read together as 'Windows is best-effort, through WSL'; offered to Leif as the default with the #212 summary and not changed (2026-11-10)
- Leif (0xLeif) decided "Design review of design/prototype (Artifact 1gaHcJYeiULDxWtKMGUZim)": "Approve the design" (Coordinator chat with Leif (Claude Code, hc terminal t370), 2026-11-10, answer to the design approval question); recorded by claude-code (claude-opus-5-5, hc t370); a relayed claim, not authenticated
- Leif (0xLeif) decided "Intent approval of the human-first cycle as a whole (chough #212)": "Approve it" (Coordinator chat with Leif (Claude Code, hc terminal t370), 2026-11-10, answer to: Do you approve this as Chough's intent? (the 10-point #212 summary, Windows read as best-effort through WSL)); recorded by claude-code (claude-opus-5-5, hc t370); a relayed claim, not authenticated
- Leif (0xLeif) decided "Should all finished-session stories stay in the archive?": "Keep all (Recommended)", meaning every finished session stays as one small story file in the archive and nothing is trimmed, so links never break; the retired KNOW-29 five-session bound does not come back (Coordinator chat with Leif (Claude Code, hc t370), 2026-11-10); relayed by t370, recorded by claude-code (claude-opus-5-5, hc t382); a relayed claim, not authenticated
- Leif (0xLeif) decided "Where should proof files live: failure logs and screenshots?": "Split them (Recommended)", meaning logs go to CI, the record keeps the key failing lines, the exact command and a fingerprint, and a few small key screenshots are committed next to the session story with a size cap so they never expire; the cap and layout are proposed in the plan for Leif to confirm at scope approval (Coordinator chat with Leif (Claude Code, hc t370), 2026-11-10); relayed by t370, recorded by claude-code (claude-opus-5-5, hc t382); a relayed claim, not authenticated
- Leif (0xLeif) decided "Which open Chough bugs go into 0.6.0?": he picked all four groups offered and added "all bugs?", meaning 0.6.0 fixes every open Chough bug, GitHub #237, #12, #190, #76, #14 (all four items) and #132, and the local BUG-0001, BUG-0020, BUG-0022, BUG-0023, the empty change directory counted as open, history writes refused after the adopting session rolls into the vault, and the three flakes, each with a regression case; BUG-0002 and BUG-0003 describe SpecSync behaviour and are tracked by the spec-sync workers, not in 0.6.0 (Coordinator chat with Leif (Claude Code, hc t370), 2026-11-10); relayed by t370, recorded by claude-code (claude-opus-5-5, hc t382); a relayed claim, not authenticated
- Coordinator t370 instruction (2026-11-10): BUG-0002 and BUG-0003 were checked by t37b on spec-sync main 46f55652 (SpecSync 6.2.0) and neither reproduces, so 0.6.0 closes both as fixed upstream citing that check, not as fixed by Chough; agent plan: a new closed bug status fixed-upstream (REQ-cli-012) because verified needs a Chough pass after a failure and wont-fix would be untrue
- Leif (0xLeif) decided the audit Q1 and Q4 (retire the rest of the old cleanup rules; where the approved design and criteria disagree, the design wins, with the wording corrected and KNOW-34 and KNOW-35 captured): "Yes, make it match"; t37d puts the wording into #212 and the 0.6.0 change retires them with hi retire, rewords KNOW-24 and carries preview first and never weaken a SpecSync archive into the one-time upgrade (Coordinator chat with Leif (Claude Code, hc t370), 2026-11-10); relayed by t370, recorded by claude-code (claude-opus-5-5, hc t382); a relayed claim, not authenticated
- Leif (0xLeif) decided the audit Q2 "When an agent acts on permission you gave in advance, whose name goes on the SpecSync approval?": "The agent's (Recommended)", meaning the approval names the agent and quotes his earlier words and where, and only a decision made at that gate is in the human's name (FLOW-23, and FLOW-24 that t37d captures in #212) (Coordinator chat with Leif (Claude Code, hc t370), 2026-11-10); relayed by t370, recorded by claude-code (claude-opus-5-5, hc t382); a relayed claim, not authenticated
- Leif (0xLeif) decided "Rewrite Git history to purge leaked local paths?": "No, clean from 0.6.0 on (Recommended)", meaning no history rewrite; the migration removes home-folder paths from every current committed file and the no-leak check blocks new ones; history keeps the old paths, which are the user name and folder layout only, since a secret-pattern scan of origin/main found no keys or tokens (Coordinator chat with Leif (Claude Code, hc t370), 2026-11-10); relayed by t370, recorded by claude-code (claude-opus-5-5, hc t382); a relayed claim, not authenticated
- Leif (0xLeif) decided the 0.6.0 release condition: "yes.. but we dog food and fix all bugs before tagging and releasing", meaning no 0.6.0 tag or release until the chough-repo dogfood is complete and every bug found in dogfood or testing is fixed with its regression case, with zero open Chough bugs at tag time; the release itself still waits for his go (Coordinator chat with Leif (Claude Code, hc t370), 2026-11-10); relayed by t370, recorded by claude-code (claude-opus-5-5, hc t382); a relayed claim, not authenticated
- Leif (0xLeif) set a standing rule, effective 2026-11-10: SpecSync approvals and reviews made on Leif's standing permission (the bug-fix rule, or an earlier 'merge when green') name the acting agent as <agent>-<terminal> (for example kimi-code-t37b), as --actor or --reviewer, with a note naming the rule, where it was given and the recording agent, model and hc terminal; only a decision Leif made at that gate is in Leif's name (--actor 0xLeif), quoting the exact words, where, and the recording agent; archived records are not edited; a refused non-0xLeif actor is reported to the coordinator and never retried as 0xLeif. Folded into REQ-cli-022, REQ-cli-029 and REQ-records-008 (Coordinator chat with Leif, relayed by t370 on 2026-11-10 as the coordinator's summary, not Leif's verbatim words); recorded by claude-code (claude-opus-5-5, hc t382); a relayed claim, not authenticated
- Leif (0xLeif) said on 2026-11-11: "we need to make sure we use chough to fix and close all the bugs and tickets for 0.6.0 before tagging 0.6.0.. we need to dogfood and make sure chough is perfect". The coordinator set from it a ticket ledger (one disposition per open issue, pull request and local bug; only #221-#229, #231, #232 and items Leif defers himself may wait for after 0.6.0), a Chough 0.6.0 milestone, and the release gate: every item in that milestone closed through Chough (bug record -> fix -> regression case -> evidence -> closed), the chough-repo dogfood complete with every bug it finds fixed the same way, then Leif says go (Coordinator chat with Leif, relayed by t370 on 2026-11-11, Leif's words quoted as relayed); recorded by claude-code (claude-opus-5-5, hc t382); a relayed claim, not authenticated
- 0xLeif decided "Scope approval of the Chough 0.6.0 native change": "ok i approve" (coordinator chat, 2026-11-11 (relayed by the coordinator agent t370)); recorded by claude-code (claude-opus-5-5, hc t382); a relayed claim, not authenticated
- decided: test by area with five independent testers on fresh clones, none writing Chough records, because Leif reviews area by area (t370 relaying Leif, 2026-11-12)
- decided: Chough 0.6.0 ships with SpecSync 6.3.0. Leif: 'yes chough 0.6.0 ships with spec-sync 6.3.0' (coordinator chat, 2026-11-12, relayed by t370). When 6.3.0 is released: send the change back, add .specsync/config.toml to its paths (its version comment is an SDD policy file the audit always counts), move every pin, rerun the full suite and the SpecSync-dependent test areas on 6.3.0, then one re-approval by Leif
- decided: sent the native change back to draft on SpecSync 6.3.0 (send-back as 0xLeif, quoting his decision 'yes chough 0.6.0 ships with spec-sync 6.3.0', relayed by t370, who gave the go after the v6.3.0 release), added .specsync/config.toml to its affected paths, and folded the test stage's contract fixes K1, K2, K6, S6, U2, R9, G1 and Q1 into the deltas; rejected: a separate change for the pins, because Leif asked for them inside this change
- decided: Leif re-approved the 0.6.0 definition after the SpecSync 6.3.0 send-back; he ran the approve command himself as 0xLeif (note ends 'as committed at cf759125'; relayed by t370), covering the 6.3.0 pins and K1, K2, K6, S6, U2, R9, G1, Q1 and N1

## What was verified

- OBS-tc-history-165-earlier-archives-stay-e355ee5b96eaa89fb762f797
- OBS-tc-history-switch-archives-sessions-left-open-098c034bb55f6a0dff1be113

## Open threads

## Timeline

- 2026-11-11T02:09:03Z · gate-opened · intent → intent · Design review of design/prototype (Artifact 1gaHcJYeiULDxWtKMGUZim) ^t-4a5709
  - native_change: chough-0-6-0-human-first-records-one-markdown-file-with-yaml-front-matter-per-record-date-slug-session-folders-with-an
  - revision: 6f45bc7ee5be4b4e9367d0594e0f06da76bd5d9e
- 2026-11-11T02:09:03Z · gate-opened · intent → intent · Intent approval of the human-first cycle as a whole (chough #212) ^t-124360
  - native_change: chough-0-6-0-human-first-records-one-markdown-file-with-yaml-front-matter-per-record-date-slug-session-folders-with-an
  - revision: 6f45bc7ee5be4b4e9367d0594e0f06da76bd5d9e
- 2026-11-11T02:09:13Z · gate-closed · intent → intent · Design review of design/prototype (Artifact 1gaHcJYeiULDxWtKMGUZim) ^t-fa249a
  - native_change: chough-0-6-0-human-first-records-one-markdown-file-with-yaml-front-matter-per-record-date-slug-session-folders-with-an
  - revision: 6f45bc7ee5be4b4e9367d0594e0f06da76bd5d9e
  - relayed_by: {"decided_by":"Leif (0xLeif)","words":"Approve the design","where":"Coordinator chat with Leif (Claude Code, hc terminal t370), 2026-11-10, answer to the design approval question","recorded_by":"claude-code (claude-opus-5-5, hc t370)"}
- 2026-11-11T02:09:14Z · gate-closed · intent → intent · Intent approval of the human-first cycle as a whole (chough #212) ^t-3ca2e9
  - native_change: chough-0-6-0-human-first-records-one-markdown-file-with-yaml-front-matter-per-record-date-slug-session-folders-with-an
  - revision: 6f45bc7ee5be4b4e9367d0594e0f06da76bd5d9e
  - relayed_by: {"decided_by":"Leif (0xLeif)","words":"Approve it","where":"Coordinator chat with Leif (Claude Code, hc terminal t370), 2026-11-10, answer to: Do you approve this as Chough's intent? (the 10-point #212 summary, Windows read as best-effort through WSL)","recorded_by":"claude-code (claude-opus-5-5, hc t370)"}
- 2026-11-11T02:09:22Z · advance · intent → design ·  ^t-3a8e0d
  - native_change: chough-0-6-0-human-first-records-one-markdown-file-with-yaml-front-matter-per-record-date-slug-session-folders-with-an
  - revision: 6f45bc7ee5be4b4e9367d0594e0f06da76bd5d9e
- 2026-11-11T02:09:36Z · advance · design → plan ·  ^t-93a3f4
  - native_change: chough-0-6-0-human-first-records-one-markdown-file-with-yaml-front-matter-per-record-date-slug-session-folders-with-an
  - revision: 6f45bc7ee5be4b4e9367d0594e0f06da76bd5d9e
- 2026-11-11T14:24:19Z · gate-opened · plan → plan · Scope approval of the Chough 0.6.0 native change ^t-b9f7bd
  - native_change: chough-0-6-0-human-first-records-one-markdown-file-with-yaml-front-matter-per-record-date-slug-session-folders-with-an
  - revision: bb1f8b58444f79c08f30ee13f4ec0d73ec7a277b
- 2026-11-11T15:44:54Z · gate-closed · plan → plan · Scope approval of the Chough 0.6.0 native change ^t-88a902
  - native_change: chough-0-6-0-human-first-records-one-markdown-file-with-yaml-front-matter-per-record-date-slug-session-folders-with-an
  - revision: b646456ab1f5150404bc97ddd66cfa5205574e52
  - relayed_by: {"decided_by":"0xLeif","words":"ok i approve","where":"coordinator chat, 2026-11-11 (relayed by the coordinator agent t370)","recorded_by":"claude-code (claude-opus-5-5, hc t382)"}
- 2026-11-11T15:48:10Z · direct-native-write · plan → plan ·  ^t-206a2c
  - native_change: chough-0-6-0-human-first-records-one-markdown-file-with-yaml-front-matter-per-record-date-slug-session-folders-with-an
  - revision: 24545ca7ee6e23da00aa7f7fb1414f532062995a
  - direct_write: {"change":"chough-0-6-0-human-first-records-one-markdown-file-with-yaml-front-matter-per-record-date-slug-session-folders-with-an","files":[{"path":"state.json","change":"changed"},{"path":"approvals.json","change":"changed"}],"before_state":"draft","after_state":"approved"}
- 2026-11-11T15:48:10Z · advance · plan → build · native:approved ^t-cddf8b
  - native_change: chough-0-6-0-human-first-records-one-markdown-file-with-yaml-front-matter-per-record-date-slug-session-folders-with-an
  - gate: native:approved
  - revision: 24545ca7ee6e23da00aa7f7fb1414f532062995a
- 2026-11-12T01:22:03Z · direct-native-write · build → build ·  ^t-9f574c
  - native_change: chough-0-6-0-human-first-records-one-markdown-file-with-yaml-front-matter-per-record-date-slug-session-folders-with-an
  - revision: 545bfc6662ceb3fe248fb25d847a851298e50c8e
  - direct_write: {"change":"chough-0-6-0-human-first-records-one-markdown-file-with-yaml-front-matter-per-record-date-slug-session-folders-with-an","files":[{"path":"state.json","change":"changed"},{"path":"approvals.json","change":"changed"},{"path":"verification.json","change":"added"},{"path":"verification-attempts.json","change":"added"}],"before_state":"approved","after_state":"verifying"}
- 2026-11-12T01:22:03Z · advance · build → test · native:verifying ^t-bafed1
  - native_change: chough-0-6-0-human-first-records-one-markdown-file-with-yaml-front-matter-per-record-date-slug-session-folders-with-an
  - gate: native:verifying
  - revision: 545bfc6662ceb3fe248fb25d847a851298e50c8e
- 2026-11-12T06:42:21Z · direct-native-write · test → test ·  ^t-90de91
  - native_change: chough-0-6-0-human-first-records-one-markdown-file-with-yaml-front-matter-per-record-date-slug-session-folders-with-an
  - revision: b8aa318617844c7f6e4564d5e1e6583058bf93e2
  - direct_write: {"change":"chough-0-6-0-human-first-records-one-markdown-file-with-yaml-front-matter-per-record-date-slug-session-folders-with-an","files":[{"path":"state.json","change":"changed"},{"path":"approvals.json","change":"changed"},{"path":"verification.json","change":"changed"},{"path":"verification-attempts.json","change":"changed"}],"before_state":"verifying","after_state":"draft"}
- 2026-11-12T06:42:21Z · back · test → plan · Native change sent back to draft on SpecSync 6.3.0 for re-scoping: the 6.3.0 pin move plus the contract fixes K1, K2, K6, S6, U2, R9, G1 and Q1; Leif re-approves the definition once. ^t-31f2b2
  - native_change: chough-0-6-0-human-first-records-one-markdown-file-with-yaml-front-matter-per-record-date-slug-session-folders-with-an
  - revision: b8aa318617844c7f6e4564d5e1e6583058bf93e2
  - refresh: plan: native definition; scope or contract changes need native reapproval, build: implementation against the current definition, test: rerun affected cases; earlier observations may be stale, checkpoint references: 0 case(s) and 2 observation(s) to re-check with `chough status`
- 2026-11-12T14:21:17Z · direct-native-write · plan → plan ·  ^t-da83e1
  - native_change: chough-0-6-0-human-first-records-one-markdown-file-with-yaml-front-matter-per-record-date-slug-session-folders-with-an
  - revision: 5ba1afb07a10f58615e480c32fdef3caf79ab3fc
  - direct_write: {"change":"chough-0-6-0-human-first-records-one-markdown-file-with-yaml-front-matter-per-record-date-slug-session-folders-with-an","files":[{"path":"state.json","change":"changed"},{"path":"approvals.json","change":"changed"},{"path":"verification.json","change":"changed"},{"path":"verification-attempts.json","change":"changed"}],"before_state":"draft","after_state":"verifying"}
- 2026-11-12T14:21:17Z · advance · plan → build · Leif re-approved the 6.3.0 definition; the build is the code already on the branch (pins, K1, K2, Q1, K6, U2, R9, N1) ^t-285c48
  - native_change: chough-0-6-0-human-first-records-one-markdown-file-with-yaml-front-matter-per-record-date-slug-session-folders-with-an
  - gate: native:verifying
  - revision: 5ba1afb07a10f58615e480c32fdef3caf79ab3fc
- 2026-11-12T14:21:21Z · advance · build → test · Built and verified on SpecSync 6.3.0: change check --commit materialized the deltas; every send-back item red then green ^t-939c8c
  - native_change: chough-0-6-0-human-first-records-one-markdown-file-with-yaml-front-matter-per-record-date-slug-session-folders-with-an
  - gate: native:verifying
  - revision: 5ba1afb07a10f58615e480c32fdef3caf79ab3fc
- 2026-11-12T14:24:07Z · direct-native-write · test → test ·  ^t-64a16f
  - native_change: chough-0-6-0-human-first-records-one-markdown-file-with-yaml-front-matter-per-record-date-slug-session-folders-with-an
  - revision: 7bb490f38e6ac06edca054a0a168008796edfa7a
  - direct_write: {"change":"chough-0-6-0-human-first-records-one-markdown-file-with-yaml-front-matter-per-record-date-slug-session-folders-with-an","files":[{"path":"state.json","change":"changed"},{"path":"verification.json","change":"changed"},{"path":"verification-attempts.json","change":"changed"}],"before_state":"verifying","after_state":"verifying"}
- 2026-11-12T14:24:07Z · advance · test → accept · Test green on SpecSync 6.3.0; at accept for Leif's review (t370 is bringing him the acceptance summary) ^t-62dcc7
  - native_change: chough-0-6-0-human-first-records-one-markdown-file-with-yaml-front-matter-per-record-date-slug-session-folders-with-an
  - gate: native:verifying
  - revision: 7bb490f38e6ac06edca054a0a168008796edfa7a

