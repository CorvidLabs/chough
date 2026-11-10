# Lesson bundle: refuse-new-session-writes-when-the-adopted-frozen-legacy-log-is-missing-113

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Refuse new-session writes when the adopted frozen legacy log is missing (113)
- **Kind**: BugFix
- **Specs**: history
- **Paths**: src/history.rs, tests/sessions.rs, .chough/cases/history.missing-log-113.cases.toml, CHANGELOG.md, knowledge/INDEX.md, .chough/sessions/chough-113-missing-log.json, .chough/history/cycles/CYC-9b5095352ed99d70ff1d2f0b/, .chough/artifacts/issue-113/
- **Acceptance**: Find a remembered legacy adoption anchor independently of whether transitions.jsonl currently exists. With a valid adoption holder open, archived, shelved or in its existing supported vault location, deleting that frozen log must refuse a new session append before checkpoint/cursor/active-pointer or event bytes change. Preserve original adopting-session refusal and changed-byte mismatch refusal. Intact/restored logs permit new-session writes, and genuinely new projects with no adoption anchor/log still work with legacy:null. Preserve each session own independent lineage; no anchor copying, immutable record rewrite, new schema, broad archive scan or weakened bounds/path protections. Add meaningful committed regression before the production patch and prove same assertions RED then GREEN on an actually built exact-source artifact only under a separate root lease. Keep old tests/assertions and failed receipts. Current older-runtime private8 controls are qualified evidence only, not current-main execution. Update only the allocated per-change cases/changelog/INDEX and own records, freeze clean exact SHA, and return for different-author review; human acceptance/delivery remain separate.

## Evidence

- Verification commit: `9b081b8742ae7af03d0b097522f1e8ecd9905de4`
- Base commit: `4bbd5a6373d26c0ad069878f8f543842a4564a6f`
- Verified by: `specsync check --spec history`

## From the change's context.md

# Context

## Problem and current source

Leif's open Chough113 reports a new session appending after its project's adopted frozen legacy log is deleted. Current main4bbd5a6373d26c0ad069878f8f543842a4564a6f still calls adopted_anchor only for (None, Some(current)). anchor returns None when transitions.jsonl is absent; a new session intentionally has cursor.legacy None. Consequently the inspected branch can compare None to None without looking for remembered adoption proof. Existing REQ-history-002 says unexpected anchor changes refuse new writes; the history contract error table says anchor mismatch refuses. REQ-history-006/DRIVE-3 keeps sessions independent. This is a repair of that existing guard, with no canonical wording/schema/CLI or permanent HI/REQ ID change.

## Authority and ownership

Actor t221 prepares only under coordinator t19f's explicit113 queued allocation relaying Leif's request to work through all Chough issues/PRs. This records the relay source, not invented human words or implementation acceptance. Root retains actual scope receipt, implementation scheduling, genuine human review/acceptance and delivery. No production edit before actual root receipt; no compile/full/site until a separate root heavy lease. One isolated tree at current main, one lifecycle writer. Other196/eval/de31/153/147d/dirty187132/60 sources, gates and archives stay untouched.

## What actually ran

Live Fledge GitHub issue/PR and read-only main discovery: issue113 open,14 issues,0 PRs, main4bbd. Private eight-case CLI matrix on immutable older INT01 executable ef3289fefad932c05b8c0da0274990cc516fb6d701549f242c86e490a6942667: missing log/new session with open holder and with archived holder each returned0 and published a new event plus changed checkpoint. Original adopter and changed-byte controls refused1 without checkpoint/history mutation; intact, restored and never-had-log controls succeeded0. The same contract oracle returned1 for each missing-log witness and0 for six controls.

This executable is NOT a qualified current-main build. Selected history.rs and sessions.rs bytes equal retained INT01 source8a37, but five other production files differ; whole-source equivalence is false. Root supplied no qualified main artifact and explicitly disallowed compilation while another full lane holds the slot. Current-main runtime and the proposed Rust regression are NOT RUN. No product bug/failed observation is written as current-main execution. One unused archive-byte guard in the external probe was always true and is explicitly uncredited; a separate actual equality check confirms archived holder adoption cursor, not uncaptured pre-rename full bytes.

## From the change's design.md

# Design

## Existing behavior to preserve

Keep original adopting-session verification, changed-byte mismatches, independent cursor.legacy None in new sessions, valid never-had-log projects, current error family, existing bounds and safe local paths. Reuse the current supported holder locations: open/archive/shelved sessions and existing library attachment sessions. Do not copy the anchor into new sessions or rewrite legacy logs, checkpoints, events, native archives or retention proof.

## Proposed focused repair

Make remembered-anchor lookup reachable whenever the active cursor has no own legacy anchor, including when current log is absent. A bounded readable holder with a remembered anchor makes an absent current log a mismatch before event publication or checkpoint/cursor save. With a present log retain the existing matching-anchor behavior; with neither current log nor remembered anchor permit genuinely new projects. Reuse current supported directory enumeration and bounded_bytes/session cap, without adding a repository-wide scan or widening unreadable-holder policy. If implementation discovers a policy ambiguity about invalid/conflicting/unreadable holders, report it instead of inventing authoritative history or changing the scope.

An implementation may parameterize the existing adopted_anchor helper by optional current anchor, preserving matching lookup when present and returning a remembered anchor when absent. The acceptance behavior, not this proposed signature, binds scope. No all-records-invalid shortcut or schema migration is needed. Holder directories are test fixture arrangements only; real archived/source records remain immutable.

## Files

src/history.rs: verification and remembered-anchor lookup only. tests/sessions.rs: focused tests/helper only, all existing tests/assertions unchanged. .chough/cases/history.missing-log-113.cases.toml: new per-change definitions binding focused tests to existing REQ-history-002 and REQ-history-006, existing LIFE/DRIVE criteria. CHANGELOG.md: one accurate Unreleased bug entry. knowledge/INDEX.md: supported projection only; broader generation churn must be returned/raised. Own session, own exact history cycle and .chough/artifacts/issue-113/ hold only this change's records/results. Native workspace is tool-owned. No other production, policy, canonical requirements, HI, schemas or other issue source is included.

## From the change's testing.md

# Testing

## Actual committed RED and SAME GREEN

Tests-only04f2 was committed before production. Actual HCt2mv compiled unchanged main4bbd production: missing-log test0pass1fail because stageadvance unexpectedly succeeded after deleting the log; never-had-log1pass. The failure is meaningful behavior, not setup. Its later three holder/postrefusal checks were not reached. Initialt2ms pre-Cargo PATH reset failure remains separate.

Fixed950e preserves both original assertions/full old prefix; adds real earlier-null session ordering control. Firstt2nh run sessions38pass1oldfail (signal-sensitive inbox test), history25pass, declaredfmt0/lint0. Actualt2p3 same old test with identical argv/env/source/harness/DUT fails101 under Pythonexecv ignored signals and passes0 under restored defaults. Actualt2p5 corrected complete sessionsONCE39pass0fail0ignored0filtered/all39names including SAME2 allfourholders/postrefusal and ordering. The original38/1 is preserved, not relabelled.

## Retained receipts and limits

.chough/artifacts/issue-113/ contains byte-exact prior HC logs, named result proof, causal/fence proof and retained-run-provenance.json. New case definitions bind the exact three tests. Current evidence add records agent-assisted review/import of these automated logs; no new cargo-test adapter invocation or argv/exit/tests_run is fabricated. Historical RED remains04f2 rather than a synthetic failed current observation.

Distinct actual coverage: corrected sessions39 plus retained same-source history25, no differential/count inflation. History/fmt/lint retain their original execution profile; no unnecessary repeat lint because all source/tool inputs remain exact and only launcher signals changed. Full project/Trust/site/Linux/CI/hosted qualification and genuine human implementation acceptance/review/delivery remain NOT RUN/pending. Bookkeeping/native structural checks cannot substitute for product or human proof.

## Definition and source preservation

Only src/history.rs and tests/sessions.rs compiled inputs changed in source950e; bookkeeping preserves their full bytes and every other compiled input. Existing scope approvaldbed and deltas are preserved. No permanent HI/REQ allocation or canonical policy/schema change. Knowledge generation may retain only the approved INDEX projection; any additional path is returned to root, not silently kept.

## Where these lessons go

- `specs/history/context.md`
