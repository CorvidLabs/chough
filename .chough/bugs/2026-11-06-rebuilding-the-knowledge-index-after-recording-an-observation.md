---
kind: bug
id: 2026-11-06-rebuilding-the-knowledge-index-after-recording-an-observation
aliases:
  - BUG-rebuilding-the-knowledge-index-after-recording-an-observation-makes-tha-1e3c3ae5fbe4e8987cf025ef
title: Rebuilding the knowledge index after recording an observation makes that observation stale
status: verified
classification: product-defect
scope: out-of-scope
impact: "Branch observations read historical at the reviewed head (seen on C4, #126, at 01aebd6c); reviewers must rerun or discount them"
criteria: [LIFE-7]
requirements: [REQ-cli-004]
cases: []
evidence:
  - OBS-tc-cli-knowledge-rebuild-keeps-observation-current-5b97190bd03da89461567287
  - OBS-tc-cli-knowledge-rebuild-keeps-observation-current-1de06628dbf05c27fc9e7edc
receipts:
  - id: 2026-11-10-tc-cli-knowledge-rebuild-keeps-observation-current
    aliases: OBS-tc-cli-knowledge-rebuild-keeps-observation-current-5b97190bd03da89461567287
    claim: "Ran cargo test --locked --test evidence -- --exact rebuilding_the_knowledge_index_keeps_the_observation_current for TC-cli-knowledge-rebuild-keeps-observation-current: failed (exit 101, tests run 1)."
    result: failed
    command: cargo test --locked --test evidence -- --exact rebuilding_the_knowledge_index_keeps_the_observation_current
    revision: e659153cdb87709c316ce21d2a81d2982976c18f
    environment: local-macos-arm64
    content: 16e556525cc353e1af2e4922428bb68ec54b1e9dc56608c5462e4076a3befa5a
    fingerprint: sha256:0a2391099cd0ba8ec1f245e963bb13291a50e6b3c64a03e73c0be741f62233ab
    case: TC-cli-knowledge-rebuild-keeps-observation-current
    case_digest: 1a8cf4ac50403134723c7756b10d141bf8cbbac4d4351a16db275eb5f6ff9706
    observed: 2026-11-10T21:10:05Z
    duration_seconds: 3
  - id: 2026-11-10-tc-cli-knowledge-rebuild-keeps-observation-current-d46954
    aliases: OBS-tc-cli-knowledge-rebuild-keeps-observation-current-1de06628dbf05c27fc9e7edc
    claim: "Ran cargo test --locked --test evidence -- --exact rebuilding_the_knowledge_index_keeps_the_observation_current for TC-cli-knowledge-rebuild-keeps-observation-current: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test evidence -- --exact rebuilding_the_knowledge_index_keeps_the_observation_current
    revision: e659153cdb87709c316ce21d2a81d2982976c18f
    environment: local-macos-arm64
    content: 1148611ab0ea207bbddf8e84f129a7cdf4786e00b7fcdbf72409f7bd9616c417
    fingerprint: sha256:443597a6d57b11b957cc84f198c8f8e1fa304ee743d080ca59c18860bc1514da
    case: TC-cli-knowledge-rebuild-keeps-observation-current
    case_digest: 1a8cf4ac50403134723c7756b10d141bf8cbbac4d4351a16db275eb5f6ff9706
    observed: 2026-11-10T21:10:41Z
    duration_seconds: 2
environment: local-macos-arm64
revision: 01aebd6c729499767aff4a140266b46ad003a294
found_by: ch-c456 agent, C4 (#126) closeout, 2026-11-05
related: []
created: 2026-11-06T03:40:59Z
updated: 2026-11-10T21:10:49Z
---
# Rebuilding the knowledge index after recording an observation makes that observation stale

**What happened:** A case observation recorded current becomes historical (stale-revision-or-inputs) once knowledge/INDEX.md is rebuilt to list it, which the verify lane's library index --check requires
Actual: knowledge/INDEX.md lists the most recent observations and is inside the working content identity, so the rebuild that lists a new observation changes the identity and every observation recorded before it reads historical; at a PR head that passes library index --check, no observation recorded on that branch is current

**Steps:** chough cases run <case> (or evidence add), then chough library index, with the librarian on (the default)
1. Record TC-cli-native-check-safe-verifying with chough cases run on 01aebd6c: status reads it current (environment-unchecked)
2. Run chough library index: knowledge/INDEX.md gains the observation's line
3. Run chough status: the observation reads historical, freshness stale-revision-or-inputs; restoring INDEX.md makes it current again

**Expected:** Recording another observation does not self-invalidate the content identity (REQ-cli-004 AC2): the generated knowledge index that lists observations does not make them stale

**Why it matters:** Branch observations read historical at the reviewed head (seen on C4, #126, at 01aebd6c); reviewers must rerun or discount them

**Fix:** Recorded before 0.6.0; the Log keeps its history.

## Log

- 2026-11-06T03:40:59Z · Reported (ch-c456 agent, C4 (#126) closeout, 2026-11-05)
- 2026-11-06T03:40:59Z · Added despite similar reports: BUG-0015 (#10) was committing .chough records, fixed by excluding .chough/ from the content identity; this one is the generated knowledge/ directory (KNOW-1, default on since #106), which lists observations and is still inside the identity
- 2026-11-06T04:51:30Z · Filed as GitHub issue CorvidLabs/chough#136 at Leif's request (2026-11-05 20:58 MST, through the orc).
- 2026-11-10T21:10:23Z · status: open -> investigating. Reproduced on failing-tests commit e659153c through TC-cli-knowledge-rebuild-keeps-observation-current: the observation reads historical after chough library index lists it.; evidence: OBS-tc-cli-knowledge-rebuild-keeps-observation-current-5b97190bd03da89461567287
- 2026-11-10T21:10:49Z · status: investigating -> verified. Passes after the fix (knowledge directory left out of the content identity) through the same case.; evidence: OBS-tc-cli-knowledge-rebuild-keeps-observation-current-1de06628dbf05c27fc9e7edc
