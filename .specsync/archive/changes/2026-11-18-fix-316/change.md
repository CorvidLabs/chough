---
id: fix-316
state: archived
type: bug_fix
base_commit: 66543c2bf509627ec2a165414acae0bc8ab59c4a
---

# Fix 316

## Intent

fix 316

## Affected Canonical Specs

- `records`

## Acceptance Criteria

- A hyphen before sk- continues a word again, as in 0.6.0: chough bug add --title 'Fix flaky sk-learn tests in CI pipeline' writes its bug, a committed 0.6.0 bug whose id holds -sk- followed by 20 or more letters, digits and hyphens accepts chough bug update, and chough doctor --leaks reports no leak in it. sk- followed by 20 or more such bytes at the start of a value or after a space is still refused as a token, and a GitHub, GitLab, Slack or AWS token form directly after a hyphen, such as x-ghp_<token>, is still refused.

## No-spec Rationale

REQ-records-006 names model-provider token forms without fixing the byte before the prefix; only the matcher's boundary for sk- and its tests change.
