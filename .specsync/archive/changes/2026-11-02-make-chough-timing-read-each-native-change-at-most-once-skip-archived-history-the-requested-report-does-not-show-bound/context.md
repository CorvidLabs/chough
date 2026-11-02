---
change: make-chough-timing-read-each-native-change-at-most-once-skip-archived-history-the-requested-report-does-not-show-bound
artifact: context
---

# Context

Constraints consulted: AGENTS.md, INTENT.md, hi/life.md (LIFE-3, LIFE-6, LIFE-7,
LIFE-10), docs/FRD.md, specs/cli requirements REQ-cli-003, 020, 021, issue #32 and
its siblings #30 and #31, and the open draft PR #29 (Kimi; checkpoint merge; not
touched).

Measured on b35acaf in a temporary full-history clone, macOS arm64, SpecSync 6.1.2,
fresh debug and release builds (sha256 50ae8210… and 44e990d7…); raw results are kept
outside the repository with the handoff report. One sample each on a machine
running other agents' processes, so durations are indicative, not benchmarks.

- `specsync change status <archived checkpoint change> --json` alone completed in
  49.954 s (exit 0, 9,912 bytes). A shimmed run cut off at 25 s had logged one
  SpecSync start and 104 Git starts (history provenance: `git show <commit>:<archived
  record>` for every archived change, `rev-list`, `log --diff-filter=A`). Shim
  per-call durations include two interpreter start-ups each and are inflated.
  `specsync change status --json` with no ID returned in 0.049 s. The slowness is in
  SpecSync's archived-change provenance path; the internal root cause is not
  established.
- Native call counts per Chough command (logging shims): next 1, status 1, prune
  preview 1, timing 11 (checkpoint change plus ten archived, the checkpoint change
  read twice), cases check 0 SpecSync and 52 `hi`.
- With every native read capped at 1 s, release build: next 3.3–4.0 s, status
  3.2–3.4 s, timing (current cycle, test stage) 11.3–11.4 s, prune 1.1–1.5 s. Debug
  build: next 5.1–6.3 s, status 11.4–15.6 s (507 KB output), timing 11.5–11.8 s.
  Timing time is almost entirely its eleven sequential native reads; next and status
  have a 2–3 s local cost (release) not yet split by phase.
- At the default 120 s per-read deadline, timing's worst case is eleven reads × 120 s.
  If every archived read costs about what the one measured read cost, that is roughly
  nine minutes. This is an extrapolation, not a measurement.
