---
change: give-real-assertions-to-the-nine-criteria-the-audit-found-with-no-selected-assertion-138
artifact: testing
---

# Testing

Each task names its test. The failing test first is `measure_report_pairs_two_binaries_on_one_root_and_keeps_native_wait_apart`: it fails until examples/measure-report.sh exists. The other new tests assert behavior and text that exist today, so each is shown real by one targeted mutation that makes it fail, then restored:

| Requirement | Mutation that must fail the new test |
|---|---|
| REQ-cli-002 | Drop "always outranks a lesson" from accept.md; copy the lesson sentence into the generated module note. |
| REQ-cli-007 | Have `checkpoint` also write a second checkpoint copy. |
| REQ-cli-012 | Drop the upgrade sentence from docs/cli.md. |
| REQ-cli-029 authority | Move plan.md's authority sentence after the approve example. |
| REQ-cli-029 cold start | Let `cases record` accept a missing verdict; claim a passed cold start in ROADMAP. |
| REQ-cli-031 | Move "## New project" after "## Install". |
| REQ-cli-045 | Drop the librarian-off skip; drop observations when no reset time is read. |
| REQ-reporting-003 | In the script, count a timed-out read as completed. |

## Requirement evidence

| Requirement | Evidence |
|---|---|
| REQ-cli-002 | tests/cli.rs (new lesson-source tests) |
| REQ-cli-007 | tests/stages.rs (new single-store test) |
| REQ-cli-012 | tests/bugs.rs (new upgrade-note test) |
| REQ-cli-029 | tests/workflow_authority.rs, tests/cases.rs (new authority and cold-start tests) |
| REQ-cli-031 | tests/cli.rs (new walkthrough-opening test) |
| REQ-cli-045 | tests/reduce.rs (new librarian-off and no-reset tests) |
| REQ-reporting-003 | tests/timing.rs `measure_report_pairs_two_binaries_on_one_root_and_keeps_native_wait_apart` |

## Lanes

`fledge lanes run verify` passes on Linux.
