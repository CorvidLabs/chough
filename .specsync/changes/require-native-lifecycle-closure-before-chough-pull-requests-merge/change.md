---
id: require-native-lifecycle-closure-before-chough-pull-requests-merge
state: approved
type: operations
base_commit: 57aa169588522ef44438236b9f03f409dc1254c2
---

# Require native lifecycle closure before Chough pull requests merge

## Intent

Require native lifecycle closure before Chough pull requests merge

## Affected Canonical Specs

- None

## Acceptance Criteria

- A required-check candidate named SpecSync Archive runs for every pull request, including drafts, without path or stage filters; it rejects every active native change and every unreadable or incomplete lifecycle package. Native SpecSync semantics authenticate archive closure and history; malformed, missing, ambiguous, stale, unknown, or unavailable inputs fail closed. CI performs no lifecycle mutation or human approval and explains that draft work can be expected red. Existing three-platform product tests/builds, HI, strict 100 percent SpecSync, formatting and Clippy remain present with consistent pinned tool versions. Isolated fixtures prove allowed no-active and authentic archived cases and rejected draft, in-progress, stale, incomplete, missing, malformed, omitted-record and unavailable-tool cases. The PR documents exact check contexts and read-only protection findings without changing branch rules or claiming human acceptance.

## No-spec Rationale

CI enforcement changes repository delivery gates; the current CLI contract and product behavior remain unchanged.
