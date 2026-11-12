---
kind: session
id: 2026-11-06-closed-bugs-release-proof-key-items-move-into-the-vault-under
aliases:
  - closed-bugs-release-proof-key-items-move-into-the-vault-under-the-bug-and-the-rest-is-trimmed-when-cold-know-22
objective: "Closed bugs release proof: key items move into the vault under the bug, the rest is trimmed when cold (KNOW-22)"
intent_scope: KNOW
modules: [cli]
stage: done
stage_status: active
iteration: 1
callers: []
native_change: closed-bugs-release-proof-key-items-move-into-the-vault-under-the-bug-and-the-rest-is-trimmed-when-cold-know-22
pr: ""
criteria: [KNOW-22]
requirements: []
cases: []
design: []
files: []
observations:
  - OBS-tc-cli-closed-bug-key-items-move-to-vault-ed7a30c1fb6534c91d7288de
  - OBS-tc-cli-closed-bug-fixture-moves-to-vault-cd1562f5273fe864dd88f48c
  - OBS-tc-cli-open-bug-keeps-links-89538a43c3a4b93c529e6b4b
bugs: []
blockers: []
waiting_on_human: []
run_consents: []
next: Wait for Leif to approve the definition, then implement against the failing tests
closed: 2026-11-06
outcome: done
receipts:
  - id: 2026-11-06-tc-cli-closed-bug-key-items-move-to-vault
    aliases: OBS-tc-cli-closed-bug-key-items-move-to-vault-ed7a30c1fb6534c91d7288de
    claim: "Ran cargo test --locked --test reduce -- --exact a_closed_bug_moves_its_key_items_into_the_vault_and_releases_the_rest for TC-cli-closed-bug-key-items-move-to-vault: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test reduce -- --exact a_closed_bug_moves_its_key_items_into_the_vault_and_releases_the_rest
    revision: a5246bd819400f0e1858a7b7f0c0aa41c4fa73de
    environment: local-linux-aarch64
    content: 40533879906bd6e9954e4c54f6560471727ca1ea075ea5ec1e4c726dfada5b2a
    fingerprint: sha256:a9f65a5d04969ef96b5066721dc94edac99f802a3bbda075225e08546f520436
    case: TC-cli-closed-bug-key-items-move-to-vault
    case_digest: d89f255ba4518aba1b9099d306de0d9c6849b918914771df1efc63edfa3b353c
    observed: 2026-11-06T00:03:31Z
    duration_seconds: 0
  - id: 2026-11-06-tc-cli-closed-bug-fixture-moves-to-vault
    aliases: OBS-tc-cli-closed-bug-fixture-moves-to-vault-cd1562f5273fe864dd88f48c
    claim: "Ran cargo test --locked --test reduce -- --exact a_closed_bug_releases_its_loose_fixture for TC-cli-closed-bug-fixture-moves-to-vault: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test reduce -- --exact a_closed_bug_releases_its_loose_fixture
    revision: a5246bd819400f0e1858a7b7f0c0aa41c4fa73de
    environment: local-linux-aarch64
    content: 40533879906bd6e9954e4c54f6560471727ca1ea075ea5ec1e4c726dfada5b2a
    fingerprint: sha256:f6579433891d134f59d18ab4c12c4e61d07ce4e052faa4b3fe31bc947e27108f
    case: TC-cli-closed-bug-fixture-moves-to-vault
    case_digest: 2ab054f4f5649fc2a8622b064037c63f4f81d15fd1195f925f6fdd4ba4a038b8
    observed: 2026-11-06T00:03:34Z
    duration_seconds: 0
  - id: 2026-11-06-tc-cli-open-bug-keeps-links
    aliases: OBS-tc-cli-open-bug-keeps-links-89538a43c3a4b93c529e6b4b
    claim: "Ran cargo test --locked --test reduce -- --exact an_open_bug_keeps_every_linked_observation for TC-cli-open-bug-keeps-links: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test reduce -- --exact an_open_bug_keeps_every_linked_observation
    revision: a5246bd819400f0e1858a7b7f0c0aa41c4fa73de
    environment: local-linux-aarch64
    content: 40533879906bd6e9954e4c54f6560471727ca1ea075ea5ec1e4c726dfada5b2a
    fingerprint: sha256:439a9cc70965e381d4415097bdeef276b2f406ee83be10c043f6872537d1bbe9
    case: TC-cli-open-bug-keeps-links
    case_digest: 3f2f70e0ffce93702b6de5b9b98275138dfbce41846f5e9a7230c1feabbda6a8
    observed: 2026-11-06T00:03:37Z
    duration_seconds: 0
---
# Closed bugs release proof: key items move into the vault under the bug, the rest is trimmed when cold (KNOW-22)

## Goal

Closed bugs release proof: key items move into the vault under the bug, the rest is trimmed when cold (KNOW-22)

## Where it stands

## Decisions and why

- PR #121 (KNOW-22) accepted by Leif (0xLeif), 2026-11-05 17:07 MST through the orc: Record and merge; his native review (pass) recorded at 3d270936.

## What was verified

- OBS-tc-cli-closed-bug-key-items-move-to-vault-ed7a30c1fb6534c91d7288de
- OBS-tc-cli-closed-bug-fixture-moves-to-vault-cd1562f5273fe864dd88f48c
- OBS-tc-cli-open-bug-keeps-links-89538a43c3a4b93c529e6b4b

## Open threads

## Timeline

- 2026-11-06T00:02:30Z · advance · intent → design · Plan approved natively (0xLeif, Approve scope, 2026-11-05 17:01 MST via the orc) ^t-7270cd
  - native_change: closed-bugs-release-proof-key-items-move-into-the-vault-under-the-bug-and-the-rest-is-trimmed-when-cold-know-22
  - revision: bb0bfe438264bb3e1abe1d0e9d6e59f1dfd70ac0
- 2026-11-06T00:02:30Z · advance · design → plan · Plan approved natively (0xLeif, Approve scope, 2026-11-05 17:01 MST via the orc) ^t-bb6973
  - native_change: closed-bugs-release-proof-key-items-move-into-the-vault-under-the-bug-and-the-rest-is-trimmed-when-cold-know-22
  - revision: bb0bfe438264bb3e1abe1d0e9d6e59f1dfd70ac0
- 2026-11-06T00:02:30Z · advance · plan → build · Plan approved natively (0xLeif, Approve scope, 2026-11-05 17:01 MST via the orc) ^t-e60d53
  - native_change: closed-bugs-release-proof-key-items-move-into-the-vault-under-the-bug-and-the-rest-is-trimmed-when-cold-know-22
  - gate: native:approved
  - revision: bb0bfe438264bb3e1abe1d0e9d6e59f1dfd70ac0
- 2026-11-06T00:03:28Z · advance · build → test · Native change verified (specsync change check --commit) ^t-10f7d6
  - native_change: closed-bugs-release-proof-key-items-move-into-the-vault-under-the-bug-and-the-rest-is-trimmed-when-cold-know-22
  - gate: native:verifying
  - revision: a5246bd819400f0e1858a7b7f0c0aa41c4fa73de
- 2026-11-06T00:03:38Z · advance · test → accept · The three KNOW-22 cases passed on Linux (local-linux-aarch64); the two closed-bug tests failed first on main ^t-f79e57
  - native_change: closed-bugs-release-proof-key-items-move-into-the-vault-under-the-bug-and-the-rest-is-trimmed-when-cold-know-22
  - gate: native:verifying
  - revision: a5246bd819400f0e1858a7b7f0c0aa41c4fa73de
- 2026-11-06T00:03:38Z · gate-opened · accept → accept · Native implementation review of this change by Leif (FLOW-10) ^t-47d82b
  - native_change: closed-bugs-release-proof-key-items-move-into-the-vault-under-the-bug-and-the-rest-is-trimmed-when-cold-know-22
  - revision: a5246bd819400f0e1858a7b7f0c0aa41c4fa73de
- 2026-11-06T00:07:39Z · gate-closed · accept → accept · Native implementation review of this change by Leif (FLOW-10) ^t-0b59c4
  - native_change: closed-bugs-release-proof-key-items-move-into-the-vault-under-the-bug-and-the-rest-is-trimmed-when-cold-know-22
  - revision: ebe90cadbccd3f656a642b38205f4c70f3e677ca
- 2026-11-06T00:07:40Z · advance · accept → done · Native change archived; Leif recorded his review ^t-95cce0
  - native_change: closed-bugs-release-proof-key-items-move-into-the-vault-under-the-bug-and-the-rest-is-trimmed-when-cold-know-22
  - gate: native:archived
  - revision: ebe90cadbccd3f656a642b38205f4c70f3e677ca


Closed 2026-11-06: done.
