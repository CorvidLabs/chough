---
id: move-the-two-192-cases-into-dedicated-196-files-without-changing-definitions-or-evidence
state: draft
type: refactor
base_commit: 4bbd5a6373d26c0ad069878f8f543842a4564a6f
---

# Move the two #192 cases into dedicated #196 files without changing definitions or evidence

## Intent

Move the two #192 cases into dedicated #196 files without changing definitions or evidence

## Affected Canonical Specs

- None

## Acceptance Criteria

- Move only TC-cli-change-new-no-spec-change from .chough/cases/cli.change-steps.cases.toml to .chough/cases/cli.case-split-196.cases.toml and TC-history-timing-skips-direct-writes from .chough/cases/history.direct-writes.cases.toml to .chough/cases/history.case-split-196.cases.toml. Preserve both raw case blocks, all fields/IDs/bindings/links and the entire catalog count, ID set, normalized definitions and actual case digests. Keep every other case, runtime/test source, canonical contract/requirements, HI, policy, accepted/inherited records and evidence files unchanged. Actual pinned chough cases check and library index --check pass after only supported own knowledge regeneration. Prove the preservation oracle discriminates missing/altered case data in private controls. Finish a clean committed local source and honest exact-tool results; no Rust binding/full/site execution is claimed without a separate lease, and no approval/review/finalize/delivery is performed by this author.

## No-spec Rationale

Only relocate existing TOML case blocks and preserve all fields, stable IDs, bindings, case digests and evidence records. Existing DRIVE-12 per-change-file contract already requires this layout; runtime, canonical requirements, HI and policy remain unchanged.
