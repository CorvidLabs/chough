# Issue 32 warm benchmark

Result: PASS. Both commands meet DRIVE-15 on the final integrated 147-archive repository.

## Identity and inputs

- Source: `cfa2d10422259d677e9e6f28ae49e7380628de78` (including `origin/main` at `85260583`).
- Release binary SHA-256: `c63a4f635c194d699817d0eab64887beb06c17fad1d972058b9488cd13c53908`.
- Host: macOS 26.5.2 (25F84), arm64.
- SpecSync 6.2.0: `/opt/homebrew/Cellar/spec-sync/6.2.0/bin/specsync`, SHA-256 `b014d720569604682f8f0269520fbb022b5ae8313c688bd1e6bab782cbb81af4`.
- HI 0.8.0; local Fledge 1.7.2 (CI names 1.8.0).
- Full non-shallow clone with no alternates; 4,927 tracked files; 147 archived changes.
- Scratch session names archived change `read-requirements-with-crlf-line-endings-in-the-librarian-default-test-131-windows`.

## Prime

The ordinary bounded prime completed successfully in 5.35 seconds. Its live SpecSync status read took 4,290 ms and `next.assess` took 4,890 ms. The report says `read observed` and `terminal evidence authenticated-history`. The existing 120-second deadline was unchanged. See `final-prime-next.log`.

## Acceptance runs

Times are `/usr/bin/time -p` wall seconds, in execution order. Ten warm runs of `next` were followed by ten warm runs of `status`; no other benchmark ran concurrently.

| Run | next | status |
|---:|---:|---:|
| 1 | 0.66 | 0.62 |
| 2 | 0.60 | 0.67 |
| 3 | 0.64 | 0.61 |
| 4 | 0.61 | 0.63 |
| 5 | 0.62 | 0.59 |
| 6 | 0.61 | 0.63 |
| 7 | 0.61 | 0.66 |
| 8 | 0.62 | 0.63 |
| 9 | 0.60 | 0.63 |
| 10 | 0.64 | 0.64 |

Nearest-rank p95 for ten runs is the tenth sorted value. `next`: p95 0.66 s, maximum 0.66 s. `status`: p95 0.67 s, maximum 0.67 s. Required: p95 at most 2.00 s and every run at most 3.00 s.

All twenty traces omit a SpecSync `change status` invocation and report the archived response as cached. Representative traces:

- `next` run 1: assess 189 ms; Git enumeration 107 ms; snapshot hash 247 ms; Git status 54 ms; case suite 4 ms; references 23 ms; evidence load 18 ms; freshness 4 ms. See `final-next-1.log`.
- `status` run 2: assess 209 ms; Git enumeration 80 ms; snapshot hash 258 ms; Git status 49 ms; evidence load 20 ms; case suite 4 ms; observations 50 ms; references 16 ms; freshness 26 ms.

## Independent zero-call proof

A PATH wrapper logged every SpecSync argument and delegated to the exact binary above. This changed the resolved binary identity and therefore required its own prime; it did not reuse the acceptance-prime entry. After that prime, separate warm `next` and `status` runs recorded zero wrapper calls each and retained `read cached` plus `authenticated-history`.

- Prime calls: 2 (`final-instrumented-prime-calls.log`).
- Warm next calls: 0 (`final-instrumented-next-calls.log`).
- Warm status calls: 0 (`final-instrumented-status-calls.log`).
- Warm next product output and trace: `final-instrumented-warm-next.log`.

These instrumented runs prove call behavior only; their times are not part of the acceptance samples.

## Historical measurement

The earlier integrated run at `a5df1ead` on 129 archives also passed: next p95/max 1.02/1.02 s and status 1.11/1.11 s. Its raw logs remain beside this summary. The table above supersedes that measurement for the final product head.
