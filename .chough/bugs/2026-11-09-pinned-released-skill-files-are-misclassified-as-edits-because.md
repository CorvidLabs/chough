---
kind: bug
id: 2026-11-09-pinned-released-skill-files-are-misclassified-as-edits-because
aliases:
  - BUG-pinned-released-skill-files-are-misclassified-as-edits-because-six-pred-2448f259d21efce039cd3e9b
title: Pinned released skill files are misclassified as edits because six predecessor pairs are missing
status: verified
classification: product-defect
scope: in-scope
impact: Existing installs retain stale pristine guidance; source update must keep all real edits intact.
criteria: [FLOW-1, LIFE-4]
requirements: [REQ-cli-016]
cases: [TC-cli-release-pristine-status, TC-cli-release-pristine-update]
evidence:
  - OBS-tc-cli-release-pristine-status-82d60293ee2debd19108f65b
  - OBS-tc-cli-release-pristine-update-15820c959f00e33dec5a751e
  - OBS-tc-cli-release-pristine-status-3d85d0c9a5a4d2c1afd63fe7
  - OBS-tc-cli-release-pristine-update-0fe36d77c0cce7dc9abb0619
receipts:
  - id: 2026-11-09-tc-cli-release-pristine-status
    aliases: OBS-tc-cli-release-pristine-status-82d60293ee2debd19108f65b
    claim: Retained actual t2ns 2026-11-09T04:46:29.660928+00:00; source6bb513b66e488359a7995c4f1eb764d6496a555e; /usr/bin/nice -n10 Fledge1.8 run test -- --locked --test skill_releases ran7,5pass/2fail. every_pinned_release_file_is_recognized_before_update FAILED at54 pristine host/file recognition/update assertions (six missing predecessor pairs). Recorder t2nr pre-Cargo setup failure is separate. Original evidence/chough102-compiled-red/MANIFEST.json and actual-red.log retained; no re-execution now.
    result: failed
    revision: d9a9d92459a611eeca6a587fb83e130b95c91dee
    environment: local-macos-arm64
    content: 06c26c73395526350b28c0d3d3b313c73e0e82af8ebd55c21c007c5f065bea8f
    case: TC-cli-release-pristine-status
    case_digest: 23399f86eb9d87156f73ceab34cc0643dceca757bb00a5bc27d456d96a13fa8d
    observed: 2026-11-09T05:14:12Z
  - id: 2026-11-09-tc-cli-release-pristine-update
    aliases: OBS-tc-cli-release-pristine-update-15820c959f00e33dec5a751e
    claim: Retained actual t2ns 2026-11-09T04:46:29.660928+00:00; source6bb513b66e488359a7995c4f1eb764d6496a555e; /usr/bin/nice -n10 Fledge1.8 run test -- --locked --test skill_releases ran7,5pass/2fail. every_pristine_release_updates_instead_of_staying_a_preserved_edit FAILED at54 pristine host/file recognition/update assertions (six missing predecessor pairs). Recorder t2nr pre-Cargo setup failure is separate. Original evidence/chough102-compiled-red/MANIFEST.json and actual-red.log retained; no re-execution now.
    result: failed
    revision: d9a9d92459a611eeca6a587fb83e130b95c91dee
    environment: local-macos-arm64
    content: 06c26c73395526350b28c0d3d3b313c73e0e82af8ebd55c21c007c5f065bea8f
    case: TC-cli-release-pristine-update
    case_digest: 8e6776b6769ec0f675efcef3544a458350f8a86a9032189a678d49744cb79370
    observed: 2026-11-09T05:14:14Z
  - id: 2026-11-09-tc-cli-release-pristine-status-4d921c
    aliases: OBS-tc-cli-release-pristine-status-3d85d0c9a5a4d2c1afd63fe7
    claim: Retained actual t2p7 at2026-11-09T05:04:53.416163+00:00; sourced9a9d92459a611eeca6a587fb83e130b95c91dee; SAME Fledge1.8 run test -- --locked --test skill_releases:7/7 PASS, every_pinned_release_file_is_recognized_before_update passed. Exact Rust1.95/source/test/corpus identities in evidence/chough102-focused-green;720 edited rows/modes and all filename/host/current/missing/legacy boundaries retain assertions. Additional existing4+3+16, fmt/lint passed; no fresh execution while recording.
    result: passed
    revision: d9a9d92459a611eeca6a587fb83e130b95c91dee
    environment: local-macos-arm64
    content: 06c26c73395526350b28c0d3d3b313c73e0e82af8ebd55c21c007c5f065bea8f
    case: TC-cli-release-pristine-status
    case_digest: 23399f86eb9d87156f73ceab34cc0643dceca757bb00a5bc27d456d96a13fa8d
    observed: 2026-11-09T05:14:18Z
  - id: 2026-11-09-tc-cli-release-pristine-update-55048e
    aliases: OBS-tc-cli-release-pristine-update-0fe36d77c0cce7dc9abb0619
    claim: Retained actual t2p7 at2026-11-09T05:04:53.416163+00:00; sourced9a9d92459a611eeca6a587fb83e130b95c91dee; SAME Fledge1.8 run test -- --locked --test skill_releases:7/7 PASS, every_pristine_release_updates_instead_of_staying_a_preserved_edit passed. Exact Rust1.95/source/test/corpus identities in evidence/chough102-focused-green;720 edited rows/modes and all filename/host/current/missing/legacy boundaries retain assertions. Additional existing4+3+16, fmt/lint passed; no fresh execution while recording.
    result: passed
    revision: d9a9d92459a611eeca6a587fb83e130b95c91dee
    environment: local-macos-arm64
    content: 06c26c73395526350b28c0d3d3b313c73e0e82af8ebd55c21c007c5f065bea8f
    case: TC-cli-release-pristine-update
    case_digest: 8e6776b6769ec0f675efcef3544a458350f8a86a9032189a678d49744cb79370
    observed: 2026-11-09T05:14:20Z
fixture: tests/fixtures/skill-releases/manifest.json;51 exact blobs,120 pinned source mappings
environment: local-macos-arm64
revision: d9a9d92459a611eeca6a587fb83e130b95c91dee
found_by: Issue102/direct all-PR task; retained actual t2ns product RED and t2p7 SAME GREEN, agent record only
related: []
created: 2026-11-09T05:14:16Z
updated: 2026-11-09T05:14:31Z
---
# Pinned released skill files are misclassified as edits because six predecessor pairs are missing

**What happened:** Exact pristine released bytes remain edited/preserved-edit after skill update.
Actual: t2ns actual compiled seven tests5pass/2fail; six missing pairs misclassify54 rows in each recognition/update group. t2p7 after only six pairs passes SAME7 plus23 existing controls, fmt/lint.

**Steps:** Run committed seven-test offline source corpus at6bb513b across5tags/6hosts; two assertion groups each expose54 pristine rows.
1. Validate complete corpus identities before runtime.
2. Run Fledge test --locked --test skill_releases on unchanged68-pair table; retain recognition/update assertions.
3. Add only approved six pairs then run SAME7 and existing focused controls.

**Expected:** Pinned pristine release files are current or known-outdated and update to current bytes; true edits/modes and filename matching stay exact.

**Why it matters:** Existing installs retain stale pristine guidance; source update must keep all real edits intact.

**Fix:** Recorded before 0.6.0; the Log keeps its history.

## Log

- 2026-11-09T05:14:16Z · Reported (Issue102/direct all-PR task; retained actual t2ns product RED and t2p7 SAME GREEN, agent record only); evidence: OBS-tc-cli-release-pristine-status-82d60293ee2debd19108f65b, OBS-tc-cli-release-pristine-update-15820c959f00e33dec5a751e
- 2026-11-09T05:14:31Z · status: open -> verified. Retained actual compiled RED t2ns ended2026-11-09T04:46:29.660928Z on6bb; SAME GREEN t2p7 after approved six-row d9 sourcefix, plus23 old controls/fmtlint. Current record timestamps are receipt-capture times, not retroactively fabricated execution timestamps; immutable original logs preserve execution chronology. Human acceptance/full/Trust/hosted remain separate.; evidence: OBS-tc-cli-release-pristine-status-3d85d0c9a5a4d2c1afd63fe7, OBS-tc-cli-release-pristine-update-0fe36d77c0cce7dc9abb0619
