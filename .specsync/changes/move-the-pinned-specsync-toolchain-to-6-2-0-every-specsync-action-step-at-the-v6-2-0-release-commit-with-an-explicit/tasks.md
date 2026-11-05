---
change: move-the-pinned-specsync-toolchain-to-6-2-0-every-specsync-action-step-at-the-v6-2-0-release-commit-with-an-explicit
artifact: tasks
---

# Tasks

- [x] Move the four ci.yml SpecSync Action steps to the v6.2.0 commit with version '6.2.0'.
- [x] Move the archive script and fixture expectations to specsync 6.2.0.
- [x] Move README.md, AGENTS.md, docs/ci.md, docs/integrations.md and .specsync/config.toml to 6.2.0; add D-021 and a CHANGELOG entry.
- [x] Move the tests/package.rs pin-agreement cases to 6.2.0.
- [x] Run the pin test and the CI gate fixtures locally with SpecSync 6.2.0.

After `specsync change check` (plain text, per the specs/cli/context.md closeout-order
lesson): run `fledge lanes run verify` with HI 0.8.0, Fledge 1.8.0 and SpecSync 6.2.0;
its audit step needs the change implementing.

Human gates (plain text): scope approval before build is recorded natively; human
implementation review before finalize. Hosted PR checks and the duration comparison
are observed after push.
