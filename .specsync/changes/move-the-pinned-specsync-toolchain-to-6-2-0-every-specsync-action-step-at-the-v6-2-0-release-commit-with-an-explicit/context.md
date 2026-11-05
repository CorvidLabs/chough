---
change: move-the-pinned-specsync-toolchain-to-6-2-0-every-specsync-action-step-at-the-v6-2-0-release-commit-with-an-explicit
artifact: context
---

# Context

SpecSync 6.2.0 was released (tag v6.2.0, commit
40dcf33a6b0f0b55dcb7f17e8e98be9d55fa2c56; crate version 6.2.0, rust-version 1.89).
Chough's toolchain pins, kept in agreement by REQ-cli-019 and D-019, still name 6.1.2
with the Action at the v6.1.2 commit d08d2746. The local `specsync --version` already
reports 6.2.0. The v6.2.0 tag's action.yml still defaults its `version` input to
6.1.2, so every Action step must pass `version: '6.2.0'` explicitly.

The task came from session t18 on 2026-11-05: move to 6.2.0 through a SpecSync change,
nothing else. Release notes list read-cache and finalize speed-ups (#840, #842, #858)
whose effect on the CI SpecSync steps is to be measured on the PR run against the
latest main run.

Out of scope: any other Chough behavior; HI and Fledge pins; historical lessons, test
comments and archived records that name the version they observed.

## Scope authority

Human authority: Leif (0xLeif) replied "approve scope" in the orc tab at 2026-11-05 08:46 MST, relayed by session t18. It covers intent coverage by LIFE-9, REQ-cli-019 and D-019 with no new HI criterion, and exactly the scope in this change (pins, docs, D-021, package test, CHANGELOG). Limits: no merge, tag or release; implementation review stays a separate human decision. Native actor: claude, the acting agent account.
