- Chough's change log adds one fragment per change, `changelog.d/<change-id>.md`, holding
  that change's bullets, instead of editing the `CHANGELOG.md` Unreleased section, and a
  release task that assembles the fragments into the release heading and removes them, so
  two open changes no longer edit the same `CHANGELOG.md` section (#290, LIFE-16).
