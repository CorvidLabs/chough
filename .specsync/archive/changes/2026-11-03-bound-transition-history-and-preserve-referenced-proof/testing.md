---
change: bound-transition-history-and-preserve-referenced-proof
artifact: testing
---

# Testing

These are planned checks, not observed passes. Use controlled native fixtures for deterministic behavior and real native integration for lifecycle qualification. All cleanup and migration tests use disposable roots, never the repository's recorded proof.

| Case proposed | Requirement | Observable assertions |
|---|---|---|
| TC-cli-history-adoption | REQ-cli-025 | Legacy preview changes no byte; adoption preserves log and schema-1 backup; repeats do not duplicate; changed anchors, unsupported envelopes and old-reader compatibility are explicit; failed save preserves checkpoint bytes and leaves new orphans unselected |
| TC-cli-history-branches | REQ-cli-025 | Real Git branches add event files at the same controlled timestamp, preserving both hashes after integration; compatible checkpoint changes merge; divergent cursors refuse as one current-cycle group; timing never spans independent forks; unrelated reset cannot affect current guidance |
| TC-cli-history-bounds | REQ-cli-026 | Exact 65,536-byte event boundary, 128-event/512-KiB recent bounds, 4,096-event/16-MiB cycle and timing bounds, and 16,384-source/64-MiB maintenance bounds; instrument read counts and bytes; cap refusal leaves checkpoint byte-identical and a reset can start a fresh cycle |
| TC-cli-history-retention | REQ-cli-026 | Preview is read-only with exact path/digest/byte accounting; newest three, 14-day, active, unresolved fork/orphan and every reference source protect; protected-only fixture is truthful no-op; authorized apply preserves anchor, migration backup and frozen proof; interrupted receipts distinguish present, pruned and unavailable |
| TC-cli-retention-revalidation | REQ-cli-027 | Reference added after preview protects the observation and cycle at apply; shared artifacts survive; changed/missing inputs, symlinks/traversal, malformed protection sources and interrupted removal never broaden deletion or claim completed cleanup |

Additional checks: force one random-source failure and one identical event/cycle candidate collision; assert one draw and no overwrite/fallback/retry. Inject failure before and after event publication and checkpoint replacement, and before/after receipt publication and each removal. Missing parents, clock ties, descending clocks, malformed envelopes and oversize plans must be partial/unknown or refused. Compare recursive byte/hash inventories for protected files.

The real Git fixture combines a supported history cleanup with a second branch's new evidence and immutable events, then rechecks every retained reference and artifact hash. It also covers unrelated current-cycle edits and leaves unsupported merges actionable instead of inventing a unified history.

Run focused history, stages, timing, next and evidence suites plus existing schema migration, package and identity regressions. Then Fledge fmt, test, lint and lanes run verify using HI 0.8.0, Fledge 1.8.0 and SpecSync 6.1.2. Run tests appropriate to an implementation; definition preparation does not claim product behavior. Observe hosted Ubuntu, macOS, Windows and MSRV checks only after they actually run. No live adopt/prune or cold-start exercise is authorized here.
