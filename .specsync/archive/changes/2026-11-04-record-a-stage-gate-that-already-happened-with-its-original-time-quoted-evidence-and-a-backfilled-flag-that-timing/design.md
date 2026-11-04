---
change: record-a-stage-gate-that-already-happened-with-its-original-time-quoted-evidence-and-a-backfilled-flag-that-timing
artifact: design
---

# Design

- `chough stage backfill --at <time> --commit <rev> --quote <text> [--note <text>]`.
  `--at` takes `YYYY-MM-DDTHH:MM:SSZ` or Unix seconds.
- It runs `move_forward(kind = "advance")` with every existing refusal, so the
  destination's native prerequisite is read now. A gate passed earlier but no longer
  supported is refused, and backfill never asserts a past native state.
- The transition is an ordinary `advance` with `at_unix_seconds` set to the given time
  and `backfill: {recorded_at_unix_seconds, commit, quote}`. The field is omitted when
  absent, so other records keep their bytes. `commit` is the full SHA of a commit that
  is an ancestor of HEAD. `quote` is the human's or the record's words, kept verbatim
  as evidence only.
- Ordering: `--at` must be at least the latest recorded transition's time and at most
  now. An unreadable latest record refuses.
- `release` to `done` cannot be backfilled.
- `timing` reads the `backfill` field. Segments a backfilled move starts or ends carry
  `backfilled: true`, each cycle's stage totals add `backfilled`, a top-level
  `backfilled` list names every backfilled move with its times, commit and quote, and
  stage estimate samples skip stage totals with any backfilled segment.
