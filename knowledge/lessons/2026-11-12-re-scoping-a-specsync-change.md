---
id: 2026-11-12-re-scoping-a-specsync-change
title: "Re-scoping a SpecSync change: clear no_spec_change by hand, one delta per spec"
type: lesson
tags: [lesson, specsync, change-workflow]
date: 2026-11-12
applies_to: [SpecSync 6.3.0, re-scoping a native change]
source: https://github.com/CorvidLabs/spec-sync/issues/1070
---
# Re-scoping a SpecSync change: clear no_spec_change by hand, one delta per spec

A change created with `no_spec_change` that later needs a contract change cannot be
approved: SpecSync refuses `no_spec_change` together with a public contract change, and
no command clears it. Set `no_spec_change` to false and its rationale to null in the
change's `state.json` by hand, record that as a direct lifecycle write with the reason,
and rehearse `change approve` and `change check --commit` in a throwaway clone before
the real approval. This gap is filed as spec-sync #1070.

The change's deltas must also name exactly the specs it affects, and a spec is affected
when it owns a file the change touches. A change that touches files of three modules
needs a real delta in each of the three, even when only one requirement changes.

Seen in Chough 0.6.0: the release preparation's first re-scope (D2 and the migrate
preview warning) needed both, and the session's decisions record them.
