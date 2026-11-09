---
id: recognize-pristine-skills-from-pinned-released-bundles-without-replacing-edits
state: implementing
type: bug_fix
base_commit: 4bbd5a6373d26c0ad069878f8f543842a4564a6f
---

# Recognize pristine skills from pinned released bundles without replacing edits

## Intent

Recognize pristine skills from pinned released bundles without replacing edits

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- For the exact pinned v0.1.0, v0.2.0, v0.3.0, v0.4.0 and v0.5.0 source bundles, every one of 24 files reads current or known-outdated and skill update refreshes pristine copies; edited bytes and modes remain exact. Match file name plus exact SHA256 only; preserve missing/current handling, all host routing and legacy removal policy. Offline manifest120 mappings/51 exact blobs; no moving-network tests, future-release promise, blanket replacement, normalization or hash ignore. Six KNOWN_PREVIOUS pairs only; meaningful compiled RED then SAME GREEN requires separate lease; no implementation acceptance or delivery implied.

## No-spec Rationale

Not applicable
