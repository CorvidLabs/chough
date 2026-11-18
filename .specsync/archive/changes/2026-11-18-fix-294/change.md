---
id: fix-294
state: archived
type: bug_fix
base_commit: 66543c2bf509627ec2a165414acae0bc8ab59c4a
---

# Fix 294

## Intent

fix 294

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- A test opens a gate on a session whose native change is recorded and asserts that chough gate list prints the summary line Change: <id> - <title> (<state>) and that the summary holds no em dash; it fails when the em dash returns to src/gates.rs. src/gates.rs is unchanged.

## No-spec Rationale

REQ-cli-061 already says a gate's summary names its change as Change: <id> - <title> (<state>) with no em dash; only a test, its case and a changelog fragment are added, and src/gates.rs is unchanged.
