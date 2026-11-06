---
id: bug-classification-deduplication-and-report-mappings-and-their-recovery-branches-153
state: implementing
type: bug_fix
base_commit: 615b587b33a1ef3a569f9557e2782a6be3df2f0f
---

# Bug classification, deduplication and report mappings, and their recovery branches (153)

## Intent

Bug classification, deduplication and report mappings, and their recovery branches (153)

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- REQ-cli-054 (new) states the bug value, deduplication and report mappings and their recovery branches; six tests in tests/bugs.rs assert its six criteria. Two fail first on defects found at 64aa77e0: a mistaken duplicate cannot be reopened, and a bug can relate to itself, with refusals naming no recovery. A small src/bugs.rs change fixes them: reopening clears duplicate_of, self-related links are refused, and refusals name chough bug list --all or chough bug update <id> --observation <id>. The four mapping tests each fail under a targeted mutation that the existing tests let pass. Every refusal's recovery command is run by a test. fledge lanes run verify passes on Linux.

## No-spec Rationale

Not applicable
