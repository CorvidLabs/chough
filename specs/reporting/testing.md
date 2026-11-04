# Reporting testing

Product checks run through `fledge run test`; structural checks through `fledge run
spec`; case definitions through `fledge run cases`. Stable case definitions for this
module are in `.chough/cases/reporting.cases.toml`. TC-reporting-0080 to
TC-reporting-0104 bind the timing integration tests in tests/timing.rs; the
`TC-reporting-` slug cases bind the request-local read regressions in tests/next.rs.
tests/spec_split.rs proves the 2026-11-04 split from cli kept every moved
requirement's wording (TC-reporting-split-keeps-requirement-wording).

| Requirement | Verification responsibility | Cases |
|-------------|-----------------------------|-------|
| REQ-reporting-001 (formerly REQ-cli-020) | Mixed-ID timing from timestamps and stored durations, never lexical IDs or random suffixes; read-only validation resume accounting | TC-reporting-0080 to 0089, 0100, 0104; existing timing regressions and mixed-ID fixtures |
| REQ-reporting-002 (formerly REQ-cli-024) | Bounded, deduplicated timing native reads with explicit excluded/unknown history | TC-reporting-0089, 0100; tests/timing.rs native read and budget regressions |
| REQ-reporting-003 (formerly REQ-cli-028) | One lazy case-suite load across observation/reference passes, unchanged legacy/slug digests, new-request source/case/artifact/environment/native-artifact invalidation, missing/retired/duplicate outcomes and exact trace stdout/error/record parity | TC-reporting-case-read-budget, TC-reporting-read-invalidation, TC-reporting-trace-parity; portable tests/next.rs regressions |
| REQ-reporting-004 | Native lifecycle records over 64 KiB still give their times; only SpecSync's 4 MiB bound applies | TC-reporting-approval-ledger-over-record-bound |

Observations and current IDs live in the Chough checkpoint and evidence records;
this plan does not invent a passing grade or hosted result.

## Case ID mapping (2026-11-04 split)

`TC-cli-<rest>` became `TC-reporting-<rest>`, and a repeated `report-` was dropped.

Moved from cli.cases.toml with their bindings, paths, environments and expectations
unchanged: TC-cli-0081 and TC-cli-0083 to TC-cli-0087 are TC-reporting-0081 and
TC-reporting-0083 to TC-reporting-0087; TC-cli-report-case-read-budget,
-read-invalidation and -trace-parity are TC-reporting-case-read-budget,
TC-reporting-read-invalidation and TC-reporting-trace-parity.

Twinned, because a case links only requirements in its own module: TC-cli-0080,
0082, 0088, 0089, 0100 and 0104 also link REQ-cli-003, 004, 006, 007 or 008, which
stay in cli. Each keeps its cli ID and those links, and TC-reporting-0080, 0082,
0088, 0089, 0100 and 0104 carry the identical binding, paths, environments and
expectations for the moved requirement. No requirement lost a case binding. Earlier
observations and bug records keep the old IDs as history.
