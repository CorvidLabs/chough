---
change: move-the-two-192-cases-into-dedicated-196-files-without-changing-definitions-or-evidence
artifact: testing
---

# Testing

## Baseline and positive controls

Before editing, the actual immutable Chough helper returns a valid full cases list/digest catalog and cases check0. Save every TOML case by ID/module and every field, the count/ID set, raw moved blocks and evidence/accepted-record byte-mode map. After moving, require exact equality of the full semantic catalog and actual case digests; require exactly the planned two source-file mappings and no duplicates. No other case content may change. Compare src/tests/config/contracts/HI blobs to base unchanged.

## Nonvacuous negative controls

Use dedicated private copies of the baseline catalog/proposed data only: omit one named case and alter a named expected-outcome/binding field. The unchanged preservation oracle must reject each while unchanged and correctly relocated controls pass. No mutation touches original fixtures, case files or records. Actual current-ID/global uniqueness and module checks run through Chough, independently of the mechanical extraction.

## Commands and proof limits

Run pinned chough cases check and chough library index --check after supported regeneration. Both must actually return0; no empty selection or speculative pass. Capture exact argv/cwd/HEAD/tool paths/versions/hashes and raw failures inside named HC helpers. No additional repository tests are introduced for this low-impact data move. The existing cargo-test bindings and their exact assertions remain unchanged but are NOT rerun without a compile/heavy lease; original archived passes are historical and never relabelled as new execution. Full/site/Trust/hosted CI/human review/delivery remain separate. Optional native structural verification is not runtime proof.
