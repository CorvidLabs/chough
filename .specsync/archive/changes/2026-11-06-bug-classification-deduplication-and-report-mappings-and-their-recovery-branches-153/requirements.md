---
change: bug-classification-deduplication-and-report-mappings-and-their-recovery-branches-153
artifact: requirements
---

# Requirements

ADDED REQ-cli-054 (see `deltas/cli.md`). It covers the bug value, deduplication and report mappings and their recovery branches:
- the allowed classifications, scopes and statuses;
- which bugs block leaving test;
- search scoring and its 0.2 and 0.5 thresholds, and `--distinct`;
- self and unknown links refused, naming `chough bug list --all`;
- reopening a duplicate clears `duplicate_of`;
- the verified refusal names `chough bug update <id> --observation <id>`;
- the report shows every stored field.

REQ-cli-012 is unchanged. Trace: FLOW-8, LIFE-5.
