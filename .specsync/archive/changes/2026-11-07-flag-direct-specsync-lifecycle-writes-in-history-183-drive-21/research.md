---
change: flag-direct-specsync-lifecycle-writes-in-history-183-drive-21
artifact: research
---

# Research

What was checked before choosing this design:

- **History compatibility.** `stage::Transition` is `#[serde(deny_unknown_fields)]`, and `history::validate_payload` refuses any kind outside the stage kinds, `gate-opened` and `gate-closed` ("unknown transition kind"). So a new kind or field cannot be read by Chough 0.4.0 or by any main build before this change. The relay field from #140 is precedent: the 0.5.0 release proposal records that a binary without that change refuses a history event carrying `relayed_by`.
- **The verification window.** The accept reference says: "Between the check in step 5 and finalize, nothing writes to `.chough/` and nothing is committed", because SpecSync's verification digest covers `.chough/`. So a finding cannot be recorded when it is seen. It is held in `.chough/local/`, which `.chough/local/.gitignore` (`*`) keeps out of Git and so out of verification, and recorded with the next stage move.
- **Which records SpecSync writes.** A change's workspace in `.specsync/changes/<id>/`, as seen on #174 and #182:
  - `change new` writes `state.json`, `change.md`, `approvals.json` and the artifact templates;
  - `approve` writes `approvals.json` and `state.json`;
  - `answer` writes `state.json`;
  - `check` writes `verification.json`, `verification-attempts.json` and `state.json`;
  - `review` writes `review.json` and `review-attempts.json`;
  - `ship` moves the directory to `.specsync/archive/changes/`.

  The six JSON records are SpecSync's lifecycle writes. The Markdown artifacts are authored by agents and are left out.
- **Who makes these writes today.** After slices 1 and 2, approve and review are the direct calls left in a normal closeout. They are the findings this slice will show until slice 5.
