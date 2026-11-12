---
kind: bug
id: 2026-10-31-case-deadline-and-output-bound-fail-with-descendants-and-large
aliases: [BUG-0010]
title: Case deadline and output bound fail with descendants and large output
status: verified
classification: product-defect
scope: in-scope
impact: A case can hang chough or exhaust memory
criteria: [FLOW-7]
requirements: [REQ-cli-011, REQ-cli-006]
cases: []
evidence:
  - bug-0010-repro-1
  - bug-0010-fixed-1
  - tc-cli-0045-1793482731
  - tc-cli-0046-1793482734
  - tc-cli-0047-1793482738
receipts:
  - id: 2026-10-31-a-1-s-case-deadline-returned-after-20-s-and-a-3-s-deadline
    aliases: bug-0010-repro-1
    claim: A 1 s case deadline returned after 20 s and a 3 s deadline after 21 s while a descendant held the pipes; 300 MB of output peaked at 578 MiB before truncation to 1 MiB
    result: failed
    revision: f19823d4287debe65cf8c5a78fa4a2f72d83b9b7
    environment: local-macos-arm64
    fingerprint: sha256:9f72d9499cd0b33377ea2c865df69428f2810a6d4639372628f7ea73074099b6
    observed: 2026-10-31T20:44:40Z
  - id: 2026-10-31-tc-cli-0045-bb994e
    aliases: bug-0010-fixed-1
    claim: "Fixed binary from bbd9f04 on the recorded reproduction: the 1 s deadline with a pipe-holding sleep descendant returned failed in 1.17 s with a timeout limitation; the 3 s case whose descendant held the pipes after exit returned in 2.21 s with the descendants-stopped limitation; no sleep descendant survived; 300 MB of output peaked at 10 MiB RSS with a 1048697-byte artifact noting 298951439 discarded bytes."
    result: passed
    revision: bbd9f041fb58f14ed1447420c183984c1a602446
    environment: local-macos-arm64
    fingerprint: sha256:1187579ed785432571ba470bb9f377b7ee5c797a7a65e5563bd3720e9eb1481a
    case: TC-cli-0045
    case_digest: e846351fbad4c1fe856c44109c72f1486c94aeb56fbf5bf0b8950970c15d1794
    observed: 2026-10-31T21:45:01Z
  - id: 2026-10-31-tc-cli-0045
    aliases: tc-cli-0045-1793482731
    claim: "Ran cargo test --locked --test cases -- --exact a_descendant_holding_the_pipes_cannot_outlast_the_case_deadline for TC-cli-0045: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test cases -- --exact a_descendant_holding_the_pipes_cannot_outlast_the_case_deadline
    revision: bbd9f041fb58f14ed1447420c183984c1a602446
    environment: local-macos-arm64
    fingerprint: sha256:4d1da9964419da7e70c02a77beceb38eac6649dc54cc3e5c0b61f624fc0ba33c
    case: TC-cli-0045
    case_digest: e846351fbad4c1fe856c44109c72f1486c94aeb56fbf5bf0b8950970c15d1794
    observed: 2026-10-31T21:38:53Z
  - id: 2026-10-31-tc-cli-0046
    aliases: tc-cli-0046-1793482734
    claim: "Ran cargo test --locked --test cases -- --exact a_descendant_holding_the_pipes_after_exit_is_stopped_by_the_deadline for TC-cli-0046: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test cases -- --exact a_descendant_holding_the_pipes_after_exit_is_stopped_by_the_deadline
    revision: bbd9f041fb58f14ed1447420c183984c1a602446
    environment: local-macos-arm64
    fingerprint: sha256:7ae6c314e4f450368e8b2c474ccbb627849e7324fa23e7a5c00b18afa2cd402a
    case: TC-cli-0046
    case_digest: 393432e604ab7cf6c612bde2c750dc1f7e70fcdfedae40da5c3c73ece38b81c9
    observed: 2026-10-31T21:38:58Z
  - id: 2026-10-31-tc-cli-0047
    aliases: tc-cli-0047-1793482738
    claim: "Ran cargo test --locked --test cases -- --exact output_beyond_the_per_stream_bound_is_counted_not_retained for TC-cli-0047: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test cases -- --exact output_beyond_the_per_stream_bound_is_counted_not_retained
    revision: bbd9f041fb58f14ed1447420c183984c1a602446
    environment: local-macos-arm64
    fingerprint: sha256:8800e37f75848d891930d8b6fc6c2f2713a0eddd32db5de7d4f2f41ca4c2c350
    case: TC-cli-0047
    case_digest: 03b6a7c063f31235add5fc8416c448b4d06164270c3f70d131770cfdbf846497
    observed: 2026-10-31T21:39:00Z
environment: local-macos-arm64
revision: f19823d4287debe65cf8c5a78fa4a2f72d83b9b7
found_by: Review finding E, reproduced 2026-10-31 against main f19823d
related: []
created: 2026-10-31T20:45:00Z
updated: 2026-10-31T21:45:11Z
---
# Case deadline and output bound fail with descendants and large output

**What happened:** 1 s and 3 s deadlines returned after 20-21 s; 300 MB output peaked at 578 MiB
Actual: Only the direct child is killed; readers wait for EOF and collect everything before truncating

**Steps:** A bound command whose descendant keeps stdout or stderr open, or that writes a lot
1. Bind argv [sh,-c,'sleep 20 & sleep 20'] with timeout_seconds 1
2. chough cases run <case>

**Expected:** The run returns by its deadline, descendants are stopped, and retained output stays within the bound while reading

**Why it matters:** A case can hang chough or exhaust memory

**Fix:** Recorded before 0.6.0; the Log keeps its history.

## Log

- 2026-10-31T20:45:00Z · Reported (Review finding E, reproduced 2026-10-31 against main f19823d); evidence: bug-0010-repro-1
- 2026-10-31T21:45:11Z · status: open -> verified. Fixed in bbd9f04 (bounded runner: per-stream caps while reading, process-group stop at the deadline or 2 s after exit, abandoned readers); regression cases TC-cli-0045 to TC-cli-0047 and the recorded reproduction rerun pass on macOS arm64.; evidence: bug-0010-fixed-1, tc-cli-0045-1793482731, tc-cli-0046-1793482734, tc-cli-0047-1793482738
