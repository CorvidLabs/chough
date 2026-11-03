---
change: integrate-the-approved-case-ordering-and-native-authority-workflow-changes
artifact: testing
---

# Verification

Before merge, record refs and all tracked archive, cold-start, observation and
artifact Git blob identities from both parents. After resolution every existing
blob must match its corresponding parent and all additive checkpoint collections
and transition occurrences must survive. No protected conflict is resolved by
rewriting its content.

Run the workflow authority/host-copy and shipped case-order regressions, relevant
bound cases, the full nine-step Fledge verify lane and all tests under both
Rust1.95 and Rust1.88. Do not exclude the prior flaky timing regression. A failure
is captured before corrective source edits or reruns, with its limits retained.

Cases execute only after required canonical materialization. No task checkbox
requires running/recording Chough cases before that check. Qualification outcomes
are observations, separate from native structural verification, human acceptance,
hosted CI, merge, release or deployment. No cold-start run or grade is added.
