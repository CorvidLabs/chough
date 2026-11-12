---
kind: session
id: 2026-11-07-issue-65-pr197-p1-corrections
aliases: [issue-65-pr197-p1-corrections]
objective: Correct both P1 capture-authority and unsafe-rollback defects in issue 65 / PR 197 without changing retention policy or old acceptance history
intent_scope: LIFE
modules: [cli, history]
stage: done
stage_status: active
iteration: 1
callers: []
native_change: correct-prune-capture-authority-and-unsafe-rollback-for-pr-197
pr: ""
criteria: [LIFE-2, LIFE-7, LIFE-8]
requirements: [REQ-history-004]
cases:
  - TC-history-prune-capture-byte-authority
  - TC-history-prune-unsafe-restore-continuation
  - TC-history-prune-protection-race
  - TC-history-prune-restore-preserves-recreated-file
  - TC-history-prune-restores-partial-removal
  - TC-history-prune-restores-earlier-sets
design: []
files: []
observations:
  - OBS-tc-history-prune-capture-byte-authority-21c076beb9fc717b19bb1aac
  - OBS-tc-history-prune-unsafe-restore-continuation-8b9bbae246ad0129e40aa181
  - OBS-product-test-97bec39c043841e26288e181
  - OBS-tc-history-prune-capture-byte-authority-0d8f9c5f414d1458c254a22a
  - OBS-tc-history-prune-unsafe-restore-continuation-c9640fdb1cd9bf652847ac4b
  - OBS-tc-history-prune-protection-race-0477fdc57b6276c5aaaf94ca
  - OBS-tc-history-prune-restore-preserves-recreated-file-a80c69539246aec8eff0d977
  - OBS-tc-history-prune-restores-partial-removal-09fafb2ff58f01d7100b5ae4
  - OBS-tc-history-prune-restores-earlier-sets-99ad7a5b8a6b46bcb2a6031a
  - OBS-product-test-80b95a49cc2ba0705ed1b2fb
  - OBS-product-test-4f7d63e468288e44cfd85c88
  - OBS-product-test-c667d4756cf1c183202f8b02
bugs:
  - BUG-prune-capture-accepts-replacement-bytes-as-deletion-authority-cdfb288128633d57b0c29089
  - BUG-prune-rollback-aborts-on-recreated-symlink-paths-b187eee3aee864159e01358c
  - BUG-full-macos-lane-checkpoint-writer-lock-timeout-ddac00a62f685f463c2b9f79
blockers: []
waiting_on_human: []
run_consents: []
next: Await Leif fresh exact-head corrected implementation review via orc t19f after final native check and push. No commit or .chough/knowledge/contract writes after that check; root alone records human review; do not archive, merge or release.
closed: 2026-11-07
outcome: done
receipts:
  - id: 2026-11-07-tc-history-prune-capture-byte-authority
    aliases: OBS-tc-history-prune-capture-byte-authority-21c076beb9fc717b19bb1aac
    claim: "Ran cargo test --locked --bin chough evidence::tests::pr197_captured_replacement_bytes_survive -- --nocapture for TC-history-prune-capture-byte-authority: failed (exit 101)."
    result: failed
    command: cargo test --locked --bin chough evidence::tests::pr197_captured_replacement_bytes_survive -- --nocapture
    revision: 8f7574956f2f8b8789022e68d6a02e9ef0567062
    environment: local-macos-arm64
    content: 5fff028745dd4b812f2c44ac362e5e5a280224bb9d24c534e611c5a9d923fcae
    fingerprint: sha256:6f0894aee6b6a94e155c0cc19e8bf9b2fb72d46c9a1373133438bf9f1bef4fd0
    case: TC-history-prune-capture-byte-authority
    case_digest: 7daf701f1a779f5139332e20511f3e5f5fc18b2bd9f52d23d64269ddd16fc8b3
    observed: 2026-11-07T17:05:34Z
    duration_seconds: 3
  - id: 2026-11-07-tc-history-prune-unsafe-restore-continuation
    aliases: OBS-tc-history-prune-unsafe-restore-continuation-8b9bbae246ad0129e40aa181
    claim: "Ran cargo test --locked --bin chough evidence::tests::pr197_unsafe_restore_ -- --nocapture for TC-history-prune-unsafe-restore-continuation: failed (exit 101)."
    result: failed
    command: cargo test --locked --bin chough evidence::tests::pr197_unsafe_restore_ -- --nocapture
    revision: 8f7574956f2f8b8789022e68d6a02e9ef0567062
    environment: local-macos-arm64
    content: 5fff028745dd4b812f2c44ac362e5e5a280224bb9d24c534e611c5a9d923fcae
    fingerprint: sha256:f54ae55a321a6672cceb79048b72467b6a259f38c2d6de5fbf72929bd9c86cc1
    case: TC-history-prune-unsafe-restore-continuation
    case_digest: 46ccbcac16f0273f7c6bde7355d161553fb06ab0b0b89d17a40b40780b5a5670
    observed: 2026-11-07T17:05:37Z
    duration_seconds: 0
  - id: 2026-11-07-correction-to-obs-product-test-e1797a0960e7bd92560cca96-prose
    aliases: OBS-product-test-97bec39c043841e26288e181
    claim: "Correction to OBS-product-test-e1797a0960e7bd92560cca96 prose counts: executed focused evidence unit14, evidence integration30, history25, spec_split4 passed (all exit0). Two corrective bindings passed; disposable baseline3 passed and capture1/unsafe2 mutants failed exit101 as expected. Attached JSON contains exact actual counts and argv."
    result: passed
    revision: 6748c3d30a005e46abe468fc00eb43900eda374f
    environment: local-macos-arm64
    content: e71b8b6ed7758d1321609e1f534c702307429f0664fc85405e469508db66a807
    fingerprint: sha256:afa689eb4c691bb80697b036dd75173e718bf4bd43185f1b46aaabe419c535fc
    observed: 2026-11-07T17:09:06Z
  - id: 2026-11-07-tc-history-prune-capture-byte-authority-617828
    aliases: OBS-tc-history-prune-capture-byte-authority-0d8f9c5f414d1458c254a22a
    claim: "Ran cargo test --locked --bin chough evidence::tests::pr197_captured_replacement_bytes_survive -- --nocapture for TC-history-prune-capture-byte-authority: passed (exit 0)."
    result: passed
    command: cargo test --locked --bin chough evidence::tests::pr197_captured_replacement_bytes_survive -- --nocapture
    revision: 6748c3d30a005e46abe468fc00eb43900eda374f
    environment: local-macos-arm64
    content: e71b8b6ed7758d1321609e1f534c702307429f0664fc85405e469508db66a807
    fingerprint: sha256:01806fb141cafb6d2df44a90fcd48ef37de96637dd1a29a6c7ddc1def0848a24
    case: TC-history-prune-capture-byte-authority
    case_digest: 7daf701f1a779f5139332e20511f3e5f5fc18b2bd9f52d23d64269ddd16fc8b3
    observed: 2026-11-07T17:07:15Z
    duration_seconds: 2
  - id: 2026-11-07-tc-history-prune-unsafe-restore-continuation-8b8526
    aliases: OBS-tc-history-prune-unsafe-restore-continuation-c9640fdb1cd9bf652847ac4b
    claim: "Ran cargo test --locked --bin chough evidence::tests::pr197_unsafe_restore_ -- --nocapture for TC-history-prune-unsafe-restore-continuation: passed (exit 0)."
    result: passed
    command: cargo test --locked --bin chough evidence::tests::pr197_unsafe_restore_ -- --nocapture
    revision: 6748c3d30a005e46abe468fc00eb43900eda374f
    environment: local-macos-arm64
    content: e71b8b6ed7758d1321609e1f534c702307429f0664fc85405e469508db66a807
    fingerprint: sha256:154b0327874b093eb494daf054915d3ee966cb4456e113c57a2fe8d231339290
    case: TC-history-prune-unsafe-restore-continuation
    case_digest: 46ccbcac16f0273f7c6bde7355d161553fb06ab0b0b89d17a40b40780b5a5670
    observed: 2026-11-07T17:07:17Z
    duration_seconds: 0
  - id: 2026-11-07-tc-history-prune-protection-race-411dc5
    aliases: OBS-tc-history-prune-protection-race-0477fdc57b6276c5aaaf94ca
    claim: "Ran cargo test --locked --bin chough -- --exact evidence::tests::prune_protection_race_restores_removed_bytes for TC-history-prune-protection-race: passed (exit 0)."
    result: passed
    command: cargo test --locked --bin chough -- --exact evidence::tests::prune_protection_race_restores_removed_bytes
    revision: 6748c3d30a005e46abe468fc00eb43900eda374f
    environment: local-macos-arm64
    content: e71b8b6ed7758d1321609e1f534c702307429f0664fc85405e469508db66a807
    fingerprint: sha256:7e8840d0fb410170816e092b7bedc9943204f7b7752d1bf76bae4d1c472b2aab
    case: TC-history-prune-protection-race
    case_digest: 763949fbada75217dc6f170254ae8a0313bff28996820116e8b9097159f1df00
    observed: 2026-11-07T17:08:40Z
    duration_seconds: 0
  - id: 2026-11-07-tc-history-prune-restore-preserves-recreated-file-4c5a98
    aliases: OBS-tc-history-prune-restore-preserves-recreated-file-a80c69539246aec8eff0d977
    claim: "Ran cargo test --locked --bin chough -- --exact evidence::tests::prune_restore_preserves_recreated_file_and_uncommitted_original for TC-history-prune-restore-preserves-recreated-file: passed (exit 0)."
    result: passed
    command: cargo test --locked --bin chough -- --exact evidence::tests::prune_restore_preserves_recreated_file_and_uncommitted_original
    revision: 6748c3d30a005e46abe468fc00eb43900eda374f
    environment: local-macos-arm64
    content: e71b8b6ed7758d1321609e1f534c702307429f0664fc85405e469508db66a807
    fingerprint: sha256:b4cc565107721363d0dc525918279b08da26a5af588633b0c28536001786adf4
    case: TC-history-prune-restore-preserves-recreated-file
    case_digest: 76355e1278f22f81d35d5f85c718e29fe9383839cec1a2ceb403279df2252a85
    observed: 2026-11-07T17:08:47Z
    duration_seconds: 0
  - id: 2026-11-07-tc-history-prune-restores-partial-removal-d1aac5
    aliases: OBS-tc-history-prune-restores-partial-removal-09fafb2ff58f01d7100b5ae4
    claim: "Ran cargo test --locked --test evidence -- --exact evidence_prune_restores_partial_removal for TC-history-prune-restores-partial-removal: passed (exit 0)."
    result: passed
    command: cargo test --locked --test evidence -- --exact evidence_prune_restores_partial_removal
    revision: 6748c3d30a005e46abe468fc00eb43900eda374f
    environment: local-macos-arm64
    content: e71b8b6ed7758d1321609e1f534c702307429f0664fc85405e469508db66a807
    fingerprint: sha256:766b60cfbd092db071e1b152d1358de7af61830f96213b17a00ac2d6844f1b54
    case: TC-history-prune-restores-partial-removal
    case_digest: 7293ab7ec230fc592298ec9915eaab8b78054de3265729abcff9834a7d4e2af2
    observed: 2026-11-07T17:08:43Z
    duration_seconds: 0
  - id: 2026-11-07-tc-history-prune-restores-earlier-sets-6744fe
    aliases: OBS-tc-history-prune-restores-earlier-sets-99ad7a5b8a6b46bcb2a6031a
    claim: "Ran cargo test --locked --bin chough -- --exact evidence::tests::prune_restores_earlier_sets_when_a_later_set_becomes_protected for TC-history-prune-restores-earlier-sets: passed (exit 0)."
    result: passed
    command: cargo test --locked --bin chough -- --exact evidence::tests::prune_restores_earlier_sets_when_a_later_set_becomes_protected
    revision: 6748c3d30a005e46abe468fc00eb43900eda374f
    environment: local-macos-arm64
    content: e71b8b6ed7758d1321609e1f534c702307429f0664fc85405e469508db66a807
    fingerprint: sha256:9806e2d8523183c57ca396495f4b32d862b52bf9d2bde50517fec86ea7629655
    case: TC-history-prune-restores-earlier-sets
    case_digest: 171ed7d2d36db96fd78c5d34b7ffa93e16bf5c0403fb267924b90bd1d0ab61fd
    observed: 2026-11-07T17:08:45Z
    duration_seconds: 0
  - id: 2026-11-07-initial-full-verify-at
    aliases: OBS-product-test-80b95a49cc2ba0705ed1b2fb
    claim: "Initial full verify at 3b9d008d6c48399f8ac4f7c62dee9fcb1b9cc4e9 failed step3: checkpoint_writers_in_parallel_keep_every_update hit checkpoint-busy after30s; Fledge exit1/cargo101. Cause unverified; raw receipt retained."
    result: failed
    revision: 3b9d008d6c48399f8ac4f7c62dee9fcb1b9cc4e9
    environment: local-macos-arm64
    content: 92f2383e07014789609a16a6d4944473a925a62695daa8440da9f3fe33ac06c1
    fingerprint: sha256:902c37bc572b3bc586471b857b5cc781ed8256a1b9a6a688b254703c216e61cb
    observed: 2026-11-07T17:19:40Z
  - id: 2026-11-07-sequential-full-verify-at
    aliases: OBS-product-test-4f7d63e468288e44cfd85c88
    claim: "Sequential full verify at 3b9d008d6c48399f8ac4f7c62dee9fcb1b9cc4e9: all10steps exit0, 596 product tests passed. Isolated prior failed checkpoint test passed exit0/1test in1.20s with unchanged timeout/assertions. Initial failure retained."
    result: passed
    revision: 3b9d008d6c48399f8ac4f7c62dee9fcb1b9cc4e9
    environment: local-macos-arm64
    content: 92f2383e07014789609a16a6d4944473a925a62695daa8440da9f3fe33ac06c1
    fingerprint: sha256:97da695b9691372e36913cccd34f7b0fde2a3e3a6fe8e0ede8a823a79adc1f84
    observed: 2026-11-07T17:19:42Z
  - id: 2026-11-07-declared-pinned-fledge-run-linux-verify-at
    aliases: OBS-product-test-c667d4756cf1c183202f8b02
    claim: "Declared pinned fledge run linux-verify at 3b9d008d6c48399f8ac4f7c62dee9fcb1b9cc4e9: all10steps exit0, 600 product tests passed. Linux clone/image matches Fledge1.8.0,HI0.8.0,SpecSync6.2.0."
    result: passed
    revision: 3b9d008d6c48399f8ac4f7c62dee9fcb1b9cc4e9
    environment: local-linux-aarch64
    content: 92f2383e07014789609a16a6d4944473a925a62695daa8440da9f3fe33ac06c1
    observed: 2026-11-07T17:19:44Z
---
# Correct both P1 capture-authority and unsafe-rollback defects in issue 65 / PR 197 without changing retention policy or old acceptance history

## Goal

Correct both P1 capture-authority and unsafe-rollback defects in issue 65 / PR 197 without changing retention policy or old acceptance history

## Where it stands

- Disposable base 1fc4c67b59f05097cab4cafa446aa58198f8ee56 reproduced both P1s: capture filter 1 executed failure; unsafe rollback filter 2 executed failures (file and ancestor symlinks). Actual argv, raw hashes and compact receipts are in native research.md.
- Failing tests: 8f7574956f2f8b8789022e68d6a02e9ef0567062; both exact case bindings executed cargo exit101 (capture 1 failed, unsafe symlink 2 failed)
- Both P1 red cases now pass at 6748c3d30a005e46abe468fc00eb43900eda374f; focused unit14/evidence30/history25/split4; independent capture/unsafe mutants fail 1/2 tests. Prior focused OBS prose counts corrected by OBS-product-test-97bec39c043841e26288e181
- Full frozen qualification at 3b9d008d6c48399f8ac4f7c62dee9fcb1b9cc4e9: Linux600/Mac596 tests and all10steps passed; initial <redacted:host-name> lock timeout retained in OBS-product-test-80b95a49cc2ba0705ed1b2fb, local BUG-full-macos-lane-checkpoint-writer-lock-timeout-ddac00a62f685f463c2b9f79; cause unverified; no test/timeout changes.
- Preservation check: 4412 historical Chough/archive/policy/split files match 1fc4c67b59f05097cab4cafa446aa58198f8ee56; predecessor archive/acceptance unchanged.

## Decisions and why

- Existing LIFE-2/7/8 and REQ-history-004 express the corrective intent; no new HI/REQ IDs. Public failure cases are clarified through CLI Purpose/history Error Cases; requirement bodies and split digest stay unchanged.
- Rejected reusing the old acceptance or editing/reopening its archive: corrected code needs this new scope and a fresh exact-head implementation review through orc t19f.
- Only corrective definition/intent bookkeeping is authorized before scope approval; real production, contracts and test/case files stay at the accepted archive head.
- Leif decided "Scope decision of correct-prune-capture-authority-and-unsafe-rollback-for-pr-197": "Yes to all" (direct Leif chat reply to orc t19f ONE-item #197 corrective scope question on 2026-11-07; existing approval relayed by orc t19f); recorded by sol-ch65; a relayed claim, not authenticated
- Leif scope approval recorded natively by orc-t19f at c763d13d41edae6ec09c76b10333189118e1e668; approval_valid and correction_valid true. This authorizes the six corrective paths only, not corrected implementation acceptance.
- Leif decided "Implementation review of correct-prune-capture-authority-and-unsafe-rollback-for-pr-197": "approve" (Direct Leif reply to orc t19f ONE-item fresh corrected #197 implementation acceptance question on 2026-11-07; existing decision relayed by orc t19f); recorded by sol-ch65; a relayed claim, not authenticated

## What was verified

- OBS-tc-history-prune-capture-byte-authority-21c076beb9fc717b19bb1aac
- OBS-tc-history-prune-unsafe-restore-continuation-8b9bbae246ad0129e40aa181
- OBS-product-test-97bec39c043841e26288e181
- OBS-tc-history-prune-capture-byte-authority-0d8f9c5f414d1458c254a22a
- OBS-tc-history-prune-unsafe-restore-continuation-c9640fdb1cd9bf652847ac4b
- OBS-tc-history-prune-protection-race-0477fdc57b6276c5aaaf94ca
- OBS-tc-history-prune-restore-preserves-recreated-file-a80c69539246aec8eff0d977
- OBS-tc-history-prune-restores-partial-removal-09fafb2ff58f01d7100b5ae4
- OBS-tc-history-prune-restores-earlier-sets-99ad7a5b8a6b46bcb2a6031a
- OBS-product-test-80b95a49cc2ba0705ed1b2fb
- OBS-product-test-4f7d63e468288e44cfd85c88
- OBS-product-test-c667d4756cf1c183202f8b02

## Open threads

## Timeline

- 2026-11-07T16:28:04Z · direct-native-write · intent → intent ·  ^t-19f7f7
  - native_change: correct-prune-capture-authority-and-unsafe-rollback-for-pr-197
  - revision: 1fc4c67b59f05097cab4cafa446aa58198f8ee56
  - direct_write: {"change":"correct-prune-capture-authority-and-unsafe-rollback-for-pr-197","files":[{"path":"state.json","change":"changed"}],"before_state":"draft","after_state":"draft"}
- 2026-11-07T16:28:04Z · advance · intent → design ·  ^t-2ef587
  - native_change: correct-prune-capture-authority-and-unsafe-rollback-for-pr-197
  - revision: 1fc4c67b59f05097cab4cafa446aa58198f8ee56
- 2026-11-07T16:28:04Z · skip · design → plan · No visual or interaction design change: this corrective scope only specifies capture authority and filesystem rollback failure handling under existing intent ^t-d1b09f
  - native_change: correct-prune-capture-authority-and-unsafe-rollback-for-pr-197
  - revision: 1fc4c67b59f05097cab4cafa446aa58198f8ee56
- 2026-11-07T16:28:04Z · gate-opened · plan → plan · Scope decision of correct-prune-capture-authority-and-unsafe-rollback-for-pr-197 ^t-7b09f9
  - native_change: correct-prune-capture-authority-and-unsafe-rollback-for-pr-197
  - revision: 1fc4c67b59f05097cab4cafa446aa58198f8ee56
- 2026-11-07T17:04:14Z · gate-closed · plan → plan · Scope decision of correct-prune-capture-authority-and-unsafe-rollback-for-pr-197 ^t-af3054
  - native_change: correct-prune-capture-authority-and-unsafe-rollback-for-pr-197
  - revision: c763d13d41edae6ec09c76b10333189118e1e668
  - relayed_by: {"decided_by":"Leif","words":"Yes to all","where":"direct Leif chat reply to orc t19f ONE-item #197 corrective scope question on 2026-11-07; existing approval relayed by orc t19f","recorded_by":"sol-ch65"}
- 2026-11-07T17:04:15Z · direct-native-write · plan → plan ·  ^t-db0584
  - native_change: correct-prune-capture-authority-and-unsafe-rollback-for-pr-197
  - revision: c763d13d41edae6ec09c76b10333189118e1e668
  - direct_write: {"change":"correct-prune-capture-authority-and-unsafe-rollback-for-pr-197","files":[{"path":"state.json","change":"changed"},{"path":"approvals.json","change":"changed"}],"before_state":"draft","after_state":"approved"}
- 2026-11-07T17:04:15Z · advance · plan → build · native:approved ^t-45cbe3
  - native_change: correct-prune-capture-authority-and-unsafe-rollback-for-pr-197
  - gate: native:approved
  - revision: c763d13d41edae6ec09c76b10333189118e1e668
- 2026-11-07T17:07:52Z · advance · build → test · native:approved ^t-499bb5
  - native_change: correct-prune-capture-authority-and-unsafe-rollback-for-pr-197
  - gate: native:approved
  - revision: 6748c3d30a005e46abe468fc00eb43900eda374f
- 2026-11-07T17:19:50Z · advance · test → accept · native:verifying ^t-da1c7c
  - native_change: correct-prune-capture-authority-and-unsafe-rollback-for-pr-197
  - gate: native:verifying
  - revision: 3b9d008d6c48399f8ac4f7c62dee9fcb1b9cc4e9
- 2026-11-07T17:19:50Z · gate-opened · accept → accept · Implementation review of correct-prune-capture-authority-and-unsafe-rollback-for-pr-197 ^t-8e18d4
  - native_change: correct-prune-capture-authority-and-unsafe-rollback-for-pr-197
  - revision: 3b9d008d6c48399f8ac4f7c62dee9fcb1b9cc4e9
- 2026-11-07T17:31:42Z · gate-closed · accept → accept · Implementation review of correct-prune-capture-authority-and-unsafe-rollback-for-pr-197 ^t-9b439c
  - native_change: correct-prune-capture-authority-and-unsafe-rollback-for-pr-197
  - revision: 9825f69ab0df457705f383033879710e14393f01
  - relayed_by: {"decided_by":"Leif","words":"approve","where":"Direct Leif reply to orc t19f ONE-item fresh corrected #197 implementation acceptance question on 2026-11-07; existing decision relayed by orc t19f","recorded_by":"sol-ch65"}
- 2026-11-07T17:31:47Z · direct-native-write · accept → accept ·  ^t-bbe33e
  - native_change: correct-prune-capture-authority-and-unsafe-rollback-for-pr-197
  - revision: 9825f69ab0df457705f383033879710e14393f01
  - direct_write: {"change":"correct-prune-capture-authority-and-unsafe-rollback-for-pr-197","files":[{"path":"review.json","change":"added"},{"path":"review-attempts.json","change":"added"}],"before_state":"verifying","after_state":"verifying"}
- 2026-11-07T17:31:47Z · advance · accept → done · native:archived ^t-5af34f
  - native_change: correct-prune-capture-authority-and-unsafe-rollback-for-pr-197
  - gate: native:archived
  - revision: 9825f69ab0df457705f383033879710e14393f01


Closed 2026-11-07: done.
