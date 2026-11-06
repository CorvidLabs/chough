# Issue 32 warm benchmark

Result: PASS. Both commands meet DRIVE-15 on the integrated 129-archive repository.

## Identity and inputs

- Source: `a5df1eadb4396ed7447e0f1173f8d0b9d4db9c62` (including `origin/main` at `64aa77e01d2763395d8af7566e62743ede37cc32`).
- Release binary SHA-256: `45bf534eedcb3dfe04dc495fdc3ba8753d41a3f9402043ed25629f0283d358f7`.
- Host: macOS 26.5.2 (25F84), arm64.
- SpecSync 6.2.0: `/opt/homebrew/Cellar/spec-sync/6.2.0/bin/specsync`, SHA-256 `b014d720569604682f8f0269520fbb022b5ae8313c688bd1e6bab782cbb81af4`.
- HI 0.8.0; local Fledge 1.7.2 (CI names 1.8.0).
- Full non-shallow clone with no alternates; 4,186 tracked files; 129 archived changes.
- Scratch session names archived change `read-requirements-with-crlf-line-endings-in-the-librarian-default-test-131-windows`.

## Prime

The ordinary bounded prime completed successfully in 9.87 seconds. Its live SpecSync status read took 7,661 ms and `next.assess` took 8,166 ms. The report says `read observed` and `terminal evidence authenticated-history`. The existing 120-second deadline was unchanged. See `prime-a5df1ead-next.log`.

A separate instrumented prime completed in 6.62 seconds. Its wrapper recorded exactly two calls: `change status ... --json` and `--version`.

## Acceptance runs

Times are `/usr/bin/time -p` wall seconds, in execution order. Ten warm runs of `next` were followed by ten warm runs of `status`; no other benchmark ran concurrently.

| Run | next | status |
|---:|---:|---:|
| 1 | 0.70 | 1.11 |
| 2 | 0.76 | 0.73 |
| 3 | 0.88 | 0.64 |
| 4 | 0.91 | 0.75 |
| 5 | 1.02 | 0.65 |
| 6 | 0.92 | 0.67 |
| 7 | 0.80 | 0.68 |
| 8 | 0.86 | 0.92 |
| 9 | 0.88 | 0.74 |
| 10 | 0.89 | 0.63 |

Nearest-rank p95 for ten runs is the tenth sorted value. `next`: p95 1.02 s, maximum 1.02 s. `status`: p95 1.11 s, maximum 1.11 s. Required: p95 at most 2.00 s and every run at most 3.00 s.

All twenty traces omit a SpecSync `change status` invocation and report the archived response as cached. Representative traces:

- `next` run 5: assess 335 ms; Git enumeration 117 ms; snapshot hash 357 ms; Git status 121 ms; case suite 3 ms; references 32 ms; evidence load 33 ms; freshness 7 ms. See `next-a5df1ead-5.log`.
- `status` run 1: assess 373 ms; Git enumeration 137 ms; snapshot hash 386 ms; Git status 79 ms; evidence load 21 ms; case suite 3 ms; observations 101 ms; references 19 ms; freshness 76 ms.

## Independent zero-call proof

A PATH wrapper logged every SpecSync argument and delegated to the exact binary above. This changed the resolved binary identity and therefore required its own prime; it did not reuse the acceptance-prime entry. After that prime, separate warm `next` and `status` runs recorded zero wrapper calls each and retained `read cached` plus `authenticated-history`.

- Prime calls: 2 (`instrumented-prime-calls.log`).
- Warm next calls: 0 (`instrumented-next-calls.log`).
- Warm status calls: 0 (`instrumented-status-calls.log`).
- Warm next product output and trace: `instrumented-warm-next.log`.

These instrumented runs prove call behavior only; their times are not part of the acceptance samples.
