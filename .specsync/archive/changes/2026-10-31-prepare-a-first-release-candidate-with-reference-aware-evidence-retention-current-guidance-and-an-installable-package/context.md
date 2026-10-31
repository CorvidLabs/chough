---
change: prepare-a-first-release-candidate-with-reference-aware-evidence-retention-current-guidance-and-an-installable-package
artifact: context
---

# Context

The human asked for a usable first release they can review, acceptance-test, install
and use in another project, with no new integrations or runtime. Queued behind the
approved fixes on PRs #3, #7 and #8; implementation starts after those merge, because
it touches the same files.

Observed on main `f19823d` (2026-10-31): 45 observations and 46 artifacts (about 87 KB
of content) in `.chough/`; every case has one run per environment, so today's volume
comes from one run per case per change plus bug evidence, not from reruns. Nothing
prunes, and repeated validation (`validate run --times N`) writes N observation and
log pairs. `.specsync/archive` holds 520 KB of frozen native records, which this change
does not touch. A disposable test showed that removing a manifest-listed observation
keeps the archive `authenticated-history` and `change audit` green, so retention can act
on Chough's working records without altering native archives.

Live guidance drift: README calls the public repository private; ROADMAP says the
stage workflow awaits review although it merged and archived; `cargo package` would
ship 300 files including Chough's own backlog, evidence and archives.
