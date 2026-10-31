# Exercise: one feature and one bug through the stages

Requirement links: FLOW-2, FLOW-5, FLOW-6, FLOW-7, FLOW-8, FLOW-9, FLOW-10;
REQ-cli-008 to REQ-cli-013. Run with `fledge run exercise` (part of the `verify` and
`ci` lanes) or directly:

```sh
cargo build --locked
CHOUGH="$PWD/target/debug/chough" sh examples/feature-bug-loop.sh [empty-dir]
```

It needs `hi`, `specsync`, `cargo`, `git` and `jq`. It copies `examples/shipping`
into an empty directory and runs the real tools. Human gates in the sample are
SIMULATED with labelled actors; nothing is pushed or posted. Every expected outcome
is an assertion, so the script either prints `EXERCISE PASSED` or names the step
that diverged.

## Sequence and observed result

Observed on macOS arm64 with hi 0.8.0, specsync 6.1.1 and the branch build of chough.

| Stage | What happens | Observed |
|---|---|---|
| intent | Sample criteria CHECKOUT-1 and CHECKOUT-2 already in `hi/`; `hi check` clean | passed |
| design | `chough stage skip --reason "Fee calculation only; …"` | `design -> plan (skip)` |
| plan | `specsync change new`, interview, artifacts, delta adding REQ-shipping-002 | draft change |
| plan → build | `chough stage advance` before approval | refused: `refused:native-draft` |
| plan → build | after `specsync change approve --actor SIMULATED-sample-approver-not-a-human` | gate `native:approved` |
| build | Threshold feature with a deliberate boundary defect; `specsync change check --commit` | materialized REQ-shipping-002 |
| test | `chough cases select --requirement REQ-shipping-002 --path src/lib.rs` | TC-shipping-0001 to 0003 selected; copy case 0004 omitted with reason |
| test | `chough cases run` for each | 0001 failed (`left: 499, right: 0`), 0002 and 0003 passed |
| bugs | `bug search`, then `bug add --scope blocking --observation <failed>` | BUG-0001 `reproduced`; duplicate report refused |
| test → accept | with BUG-0001 open | refused: `refused:blocking-bugs` |
| validate | `chough validate run` under the standing `repeat-validation off` | refused: not authorized |
| build | `stage back --to build`, fix `>` to `>=`, `change check --commit` | refresh list printed |
| test | rerun the three cases | all passed; BUG-0001 `verified`, `reproduced-and-later-passed` |
| accept → done | before a human review and native archive | refused: `refused:native-verifying` |

## What the first run taught

The first run had no `Cargo.lock`, so `cargo test --locked` stopped before compiling
and the adapter recorded `failed`, as if the planted assertion had failed. That was an
environment failure, not evidence about the behavior. The adapter now records
`not-run` with a limitation when cargo exits without a test result, the sample ships a
lockfile, and a regression test covers it (`fix(cases)` commit 97410b8). The exercise
now asserts the real failure: one test ran and `left: 499, right: 0`.

## Prepare a cold start

`sh examples/feature-bug-loop.sh <dir> prepare-cold-start` stops after BUG-0001 is
recorded at stage test, which is the starting point of the
[cold-start exercise](cold-start.md).
