---
change: this-repository-routes-spec-files-to-specsync-s-merge-driver-291
artifact: context
---

# Context

Issue #291. Two PRs touching one module both bump its spec's version and add the same numbered Change Log row, so they always conflict (#261, #253 and #252 did on 2026-11-14). SpecSync ships an ancestor-aware driver for this, `specsync merge-file`, which this repository did not use. Leif chose to start this in chat on 2026-11-14 ("Yes, start #291"). Configuring the driver for every Chough project through `chough doctor` and `chough init` is a follow-up with its own spec wording.
