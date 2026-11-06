# Issue #32 next/status profile

## Identity and inputs

- Source: `e99dc7961323e5c4e2aff1a2cf13333b0c47037e`, branch `claude/perf/32-next-status-latency`.
- Platform: macOS 26.5.2, arm64. Chough 0.4.0 freshly built with `fledge run build` and `fledge run build -- --release`.
- Tools: HI 0.8.0, SpecSync 6.2.0 (`/opt/homebrew/bin/specsync`), Fledge 1.7.2. CI names Fledge 1.8.0, so these local runs do not establish version-parity with CI.
- Input tree before profiling artifacts: 121 native archives, 369 evidence records, 3,799 tracked files; `.chough` 5.2 MiB, `.specsync` 18 MiB, `knowledge` 12 MiB.
- All commands used environment `local`. `CHOUGH_TRACE=1` supplied phase timings. Runs were repeated with warm filesystem caches; no claim is made that OS caches were purged. Sibling agents were active on the machine, so wall-time outliers are recorded rather than hidden.

## End-to-end timings

The first resume, using the installed Chough and the previous session's archived native change, took 15.895 s. That single uncontrolled observation is not a release benchmark.

A fresh binary with the new session and no active native change produced:

| Build | Command | Run 1 | Run 2 | Run 3 |
|---|---|---:|---:|---:|
| debug | `next` | 5.60 s | 3.73 s | 3.48 s |
| debug | `status` | 3.87 s | 3.17 s | 5.69 s |
| release | `next` | 7.22 s | 2.02 s | 4.20 s |
| release | `status` | 3.15 s | 1.39 s | 1.40 s |

Direct SpecSync 6.2.0 status reads produced:

| Native state | Unwrapped warm runs | Git children in separately instrumented run |
|---|---:|---:|
| no active change | 0.006-0.020 s inside Chough | 0 |
| active #32 draft | 2.25 s, 1.80 s | 18 |
| archived #131 change | 4.36 s, 4.84 s | 261 |

The archived instrumented run took 20.77 s; wrapper overhead was substantial, so it is evidence of call count and shape, not a wall-time baseline. Its Git calls included 112 `ls-files`, 36 `check-attr`, 36 `-c ... diff-files`, 28 `rev-parse`, 12 `show`, 10 `config`, and smaller numbers of history/object queries. The original audit's approximately 175 Git processes and this 261-process result are consistent with native archived-status reconstruction growing with repository/proof inputs.

## Chough phase attribution

With no active native change, Chough itself made exactly four Git calls: two `rev-parse`, one `ls-files`, and one `status`.

- Debug `next`: `snapshot.hash` took 3.108-4.067 s; Git enumeration 0.120-0.352 s; reference labels 0.102-0.138 s.
- Debug `status`: `snapshot.hash` took 2.519-4.834 s; observations 0.179-0.468 s; references 0.097-0.116 s.
- Release `next`, runs 2-3: `snapshot.hash` took 1.168-1.913 s; Git enumeration 0.405-0.669 s; Git status 0.265-1.073 s.
- Release `status`, runs 2-3: `snapshot.hash` took 0.513-0.577 s; Git enumeration 0.362-0.378 s; observations 0.059-0.096 s.

`src/evidence.rs` currently makes separate ordered legacy and content fingerprint passes over the working inputs. Any optimization must preserve both digest byte streams and existing freshness behavior.

## Interpretation and limits

1. The audit's archived-change 3-6 s native cost reproduced directly in SpecSync. It is not caused by Chough formatting.
2. Chough adds independent local cost, dominated by snapshot hashing and Git filesystem enumeration. Removing only native wait does not support a tight warm budget on every run.
3. Active native changes have a smaller but real native cost (18 Git calls here). Reusing an archived verdict does not address that regime; this proposal does not hide it behind stale active-state output.
4. There was no isolated cold-cache run and no controlled before/after archived end-to-end release run yet. The after implementation must measure ten warm archived `next` and `status` runs on the same input tree and retain the native cold read separately.
5. Existing PR #69 is prior art for an authenticated archive cache, but it is conflicting and its review identified missing Git-history inputs. It also allows cached stage transitions, which this proposal excludes.

Raw phase traces are under `runs/`. Large command outputs and PATH-wrapper call logs were diagnostic scratch and are not retained in the definition commit.
