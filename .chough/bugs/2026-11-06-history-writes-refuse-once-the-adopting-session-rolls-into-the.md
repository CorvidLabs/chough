---
kind: bug
id: 2026-11-06-history-writes-refuse-once-the-adopting-session-rolls-into-the
aliases:
  - BUG-history-writes-refuse-once-the-adopting-session-rolls-into-the-vault-03573996cc41022e6d12572f
title: history writes refuse once the adopting session rolls into the vault
status: verified
classification: product-defect
scope: out-of-scope
impact: Every open session in this repository is blocked from gate and stage writes, including closeout moves into done; a refusal names a recovery that does not recover
criteria: []
requirements: [REQ-history-006]
cases: []
evidence:
  - OBS-product-test-e8962e75988bcde24fcf3164
  - OBS-product-test-5628640eada77b4185bb7389
  - OBS-tc-records-migrate-vaulted-adopter-df0611549a339327a5e28e1d
receipts:
  - id: 2026-11-11-claude-code-claude-opus-5-5-hc-t382-ran-tests-fixtures-migrate-afcb69
    aliases: OBS-product-test-e8962e75988bcde24fcf3164
    claim: "Claude Code (claude-opus-5-5, hc t382) ran tests/fixtures/migrate/observe-vaulted-adopter.sh with the last build before #142 (chough 0.4.0 at 260ecc94 = 90e5de6f^, a scratch build with nothing installed) on a fresh copy of tests/fixtures/migrate/vaulted-adopter (the adopting session rolled into the vault, transitions.jsonl committed). checkpoint --awaiting-human Review exited 1: 'legacy history anchor changed; restore the recorded legacy bytes before writing events'; checkpoint --clear-awaiting-human exited 0 (that build has no --close-gate); stage advance exited 1 with the same error; transitions.jsonl unchanged."
    result: failed
    revision: d8219edbb8e255659071fa73b25f1d7c9b71ca5f
    environment: local-macos-arm64
    content: e287cef970a2e70b5d716af6bc20f127df4d7275ee92871b760314f7393345e9
    observed: 2026-11-11T20:34:41Z
  - id: 2026-11-11-claude-code-claude-opus-5-5-hc-t382-ran-tests-fixtures-migrate
    aliases: OBS-product-test-5628640eada77b4185bb7389
    claim: "Claude Code (claude-opus-5-5, hc t382) ran tests/fixtures/migrate/observe-vaulted-adopter.sh with the chough 0.5.0 release build on the same vaulted-adopter fixture: checkpoint --awaiting-human Review, checkpoint --close-gate Review with the four relay values, and stage advance each exited 0; transitions.jsonl unchanged. 0.5.0 already holds #142's fix, so the 0.5.x tree is writable before migration."
    result: passed
    revision: 7e6b58f37ea721ec46d1f00466daa1efcb12cb08
    environment: local-macos-arm64
    content: e287cef970a2e70b5d716af6bc20f127df4d7275ee92871b760314f7393345e9
    observed: 2026-11-11T20:34:17Z
  - id: 2026-11-11-tc-records-migrate-vaulted-adopter
    aliases: OBS-tc-records-migrate-vaulted-adopter-df0611549a339327a5e28e1d
    claim: "Ran cargo test --locked --test migrate -- --exact a_vaulted_adopter_migrates_and_writes_work for TC-records-migrate-vaulted-adopter: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test migrate -- --exact a_vaulted_adopter_migrates_and_writes_work
    revision: 7b4cd3eb827f70c821255c0efb0326f17d7ca23a
    environment: local-macos-arm64
    content: e287cef970a2e70b5d716af6bc20f127df4d7275ee92871b760314f7393345e9
    fingerprint: sha256:dc4fd5147092894b24d264fa247de1af6be8c096f3bcf8e65eb16b4d0ec13a10
    case: TC-records-migrate-vaulted-adopter
    case_digest: 545fd7deb75e96856c2e955dfc44002d311b1df9055b32cc44d6fb5f318be8c1
    observed: 2026-11-11T20:39:27Z
    duration_seconds: 5
environment: local-macos-arm64
revision: 5656b70c79b668d6226087b33e3a487ff79c4825
found_by: "ch-d7 closeout of #140, 2026-11-05 21:50 MST; reproduced on a pristine origin/main 260ecc94 build"
related: []
created: 2026-11-06T04:51:21Z
updated: 2026-11-11T20:39:52Z
---
# history writes refuse once the adopting session rolls into the vault

**What happened:** chough checkpoint --awaiting-human, --close-gate and chough stage moves refuse with 'legacy history anchor changed; restore the recorded legacy bytes before writing events' although .chough/transitions.jsonl is byte-identical to the anchor
Actual: adopted_anchor (src/history.rs) scans only .chough/sessions and .chough/sessions/archive; KNOW-29 moved the adopting session to knowledge/attachments/adopt-event-c467f940/sessions/, so no adopter is found and verify_anchor refuses; the named recovery (restore the legacy bytes) cannot work because the bytes are unchanged. Introduced by #135; the same write worked at ec830cfb

**Steps:** On main 260ecc94 (after #135, KNOW-29), any session whose cursor has no legacy anchor writes a history event
1. git worktree add <dir> origin/main; cargo build
2. chough session use keep-the-five-most-recent-archived-sessions-and-roll-older-ones-into-the-knowledge-vault-know-29
3. chough checkpoint --awaiting-human probe

**Expected:** The write succeeds: the legacy log is unchanged (digest ab8cf9bf4b314c6b3374339164ef302cb1c129dab8dd6df2133276b73407fdf5, 219674 bytes, 602 lines) and was adopted by session adopt-event-history-and-per-change-sessions-in-this-repository-and-union-merge-the-retention-ledger-and-changelog

**Why it matters:** Every open session in this repository is blocked from gate and stage writes, including closeout moves into done; a refusal names a recovery that does not recover

**Fix:** Recorded before 0.6.0; the Log keeps its history.

## Log

- 2026-11-06T04:51:21Z · Reported (ch-d7 closeout of #140, 2026-11-05 21:50 MST; reproduced on a pristine origin/main 260ecc94 build)
- 2026-11-11T20:39:43Z · Additional evidence.; evidence: OBS-product-test-e8962e75988bcde24fcf3164, OBS-product-test-5628640eada77b4185bb7389
- 2026-11-11T20:39:52Z · status: open -> verified. Refused on the last build before #142 (OBS-product-test-e8962e75988bcde24fcf3164), writable on 0.5.0 (OBS-product-test-5628640eada77b4185bb7389); in 0.6.0 the vaulted adopter migrates and the writes succeed (TC-records-migrate-vaulted-adopter passed).; evidence: OBS-tc-records-migrate-vaulted-adopter-df0611549a339327a5e28e1d
