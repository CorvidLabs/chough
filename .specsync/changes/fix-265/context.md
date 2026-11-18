---
change: fix-265
artifact: context
---

# Context

GitHub #265, from the 0.6.0 swarm report (s4). Conflict markers in a record's body (`<<<<<<<`, `=======`, `>>>>>>>`) are accepted and committed silently: every reader and `chough doctor` treat a half-merged record as healthy.

Leif approved the scope on 2026-11-18, relayed by hc t4c ("Approve the scope"):
- a record body may not hold git conflict markers;
- writes are refused naming the file, line and marker;
- `chough doctor` reports committed records that hold them;
- a new sentence in REQ-records-001.

This change is new, on main. It builds on two earlier attempts, read but not reused as records:
- the local team's in `chough-nemo-265` (018a059, d9012c6, 74defb4): correct code and tests, but wrong records, per Leif's decision on chough#220;
- an older fix on branch `agent/265-conflict-markers` in `chough-265`.

Ripple's minor findings on the local team's attempt are addressed:
- the doctor scan is limited to records;
- the cases' expectations don't overlap;
- `src/migrate.rs` is declared, with its guard tested.

Two choices go beyond those attempts:
- A `=======` line counts only between a `<<<<<<<` and a `>>>>>>>` line, so a Markdown setext heading underline is not refused. Both earlier attempts refused any line of seven `=`.
- The migration checks every file it would write before writing any, so its refusal writes nothing.

`src/records.rs` and `src/doctor.rs` belong to `cli`, so REQ-cli-055 (the doctor) names the new finding beside REQ-history-012's.
