- This repository routes `specs/**/*.spec.md` and `specs/**/requirements.md` to
  SpecSync's ancestor-aware merge driver, so two branches that each bump a module's
  version and Change Log merge locally without a hand-resolved conflict. `AGENTS.md`
  names the one-time `specsync merge --install` (#291, DRIVE-12).
