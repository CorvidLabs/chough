---
change: move-the-pinned-specsync-toolchain-to-6-2-0-every-specsync-action-step-at-the-v6-2-0-release-commit-with-an-explicit
artifact: plan
---

# Plan

## Approach

Replace each current SpecSync 6.1.2 pin with 6.2.0 and each `CorvidLabs/spec-sync`
Action reference with the v6.2.0 commit, passing `version: '6.2.0'` on all four steps.

## Out of scope

Other Chough behavior, HI and Fledge pins, and history that records observed versions
(specs/*/context.md lessons, archived changes, test comments, README lines on what
6.1.1 and 6.1.2 did).

## Steps

1. ci.yml: four Action steps to 40dcf33a6b0f0b55dcb7f17e8e98be9d55fa2c56 # v6.2.0 with
   `version: '6.2.0'`; update the comments.
2. check-specsync-archive.py `VERSION` and test-specsync-archive.py expectations.
3. README.md install command and Rust note, AGENTS.md, docs/ci.md, docs/integrations.md,
   .specsync/config.toml header comment; docs/DECISIONS.md new row D-021; CHANGELOG.md.
4. tests/package.rs pin-agreement cases.
5. Verify: grep for current 6.1.x pins, the pin test, CI gate fixtures, `fledge lanes
   run verify`; hosted PR checks and step-duration comparison.

## Risks

SpecSync 6.2.0 could read or authenticate archived changes differently from 6.1.2;
the CI gate fixtures and the archive script run against real 6.2.0 locally to catch it.

## Constraints consulted

REQ-cli-019, D-019, specs/cli/context.md lessons (plain-text human gates, change check
refuses while tasks are open, recorded exercises keep observed versions).
