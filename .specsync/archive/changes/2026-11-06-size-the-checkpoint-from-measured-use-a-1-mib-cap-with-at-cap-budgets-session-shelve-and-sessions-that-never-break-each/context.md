---
change: size-the-checkpoint-from-measured-use-a-1-mib-cap-with-at-cap-budgets-session-shelve-and-sessions-that-never-break-each
artifact: context
---

# Context

## Intent and decisions

DRIVE-13, DRIVE-14 and KNOW-29 were captured on 2026-11-05 in Leif's words, relayed by the orc. Leif's decisions, via the orc:
- "figure out the good size for the checkpoints and resize... maybe one megabyte... we need to self test" (19:56 MST);
- the session-equals-change and keep-five intent (19:57);
- the 1 MiB cap and "Capture as written" (20:08);
- "Add shelve", including per-session robustness (20:12).

C3 (#125, audit D9) folds in here. At 1 MiB, REQ-cli-021 AC1's original shape is legal: 40 gates, 40 blockers and 50 decisions of 4 KiB each, about 380 KB. KNOW-29 (keep five archived sessions) is its own change and PR, after this one.

## Measured use

Measured across all CorvidLabs repositories and worktrees, with files deduplicated by content.

- **Per-change sessions on main:** 17 sessions of 985 to 1,813 bytes, with 1 to 3 decisions and 0 to 4 observations. One adopted legacy session is 73,231 bytes.
- **Legacy checkpoints:** 85 distinct files. The largest is 93,748 bytes (124 decisions, 388 observations, 110 completed). Others: jackdaw 29,308 bytes, raven 2,938 bytes.
- **Growth:** about 19 KB/day on the largest legacy checkpoint. The 200-entry list bound binds in about 8 days, before the byte cap does.
- **Ceiling:** the current list and entry bounds allow at most about 4 MiB.

## Benchmark

Release builds, warm median in ms. Cold times are within about 20% of warm. Every command exited 0.

| Checkpoint | macOS next | macOS status | macOS write | macOS context | Linux next | Linux status | Linux write | Linux context | Peak memory (MiB) |
|---|---|---|---|---|---|---|---|---|---|
| 128 KiB | 64 | 63 | 12 | 9 | 14 | 15 | 7 | 5 | 5–9 |
| 512 KiB | 68 | 68 | 15 | 9 | 15 | 16 | 10 | 7 | 8–9 |
| 1 MiB | 69 | 67 | 17 | 11 | 18 | 19 | 17 | 10 | 11 |
| 2 MiB | 74 | 142 | 23 | 13 | 23 | 24 | 28 | 18 | 13–19 |
| 4 MiB | 102 | 115 | 40 | 29 | 35 | 37 | 48 | 29 | 24–33 |

- **Debug build on Linux at 1 MiB:** next 151 ms, status 167 ms, write 289 to 553 ms.
- **Focused view at 4 MiB:** stays within 16 KiB (`next` text 16,270 bytes, JSON 16,239 bytes).
- **Full status report:** grows with the checkpoint, by design (3.28 MB at 4 MiB).

## The rule for the cap

The cap must be at least 10 times the per-change p99 (73,231 bytes, so at least 732,310) and at least 4 times the largest legacy checkpoint (93,748 bytes, so at least 374,992). 1,048,576 bytes is 14.3 times and 11.2 times those. At the cap, `next` and `status` are within 250 ms in release and within 2 s in debug CI.

## Recovery

Before this change, no command recovered from an over-cap checkpoint. In legacy mode, `next`, `session adopt` and `history adopt` all failed. In sessions mode, one oversized session broke `next`, `session new` and `session list` for every session. Leif chose "Add shelve".
