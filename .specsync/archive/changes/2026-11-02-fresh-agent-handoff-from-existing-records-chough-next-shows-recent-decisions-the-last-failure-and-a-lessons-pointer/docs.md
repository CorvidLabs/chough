---
change: fresh-agent-handoff-from-existing-records-chough-next-shows-recent-decisions-the-last-failure-and-a-lessons-pointer
artifact: docs
---

# Docs

- docs/cli.md, "The next view": the `decisions`, `last_failure` and `lessons` fields
  and their text lines, the prefix convention (read best-effort, latest-N fallback,
  no schema change), ordering by recorded fields only, byte-identical output, and the
  archived-not-done rule in the next-valid-action order.
- skills/chough-workflow: references/accept.md (Compounding: certainty labels, source
  and limits, decisions outrank lessons; `verification_ignored_paths` wording),
  references/lifecycle.md (the same wording), references/checkpoint.md (prefix
  convention and the new blocks in Resume cold), references/bugs.md (a failure fixed
  in the same pull request still gets a bug or lesson), references/librarian.md
  (Closeout), SKILL.md only if needed; host copies refreshed.
- docs/exercises/handoff-cold-start/: seed.sh, rubric.md and, after the runs, a dated
  record of the baseline, change and real-repository runs with host, model, version,
  observation IDs and 0xLeif's grades, saying what each run cannot show.
- specs/cli/context.md: lessons from this change, labelled, written by the closeout
  pass inside this change.
- CHANGELOG.md: an Unreleased entry for the handoff blocks, the archived-not-done fix
  and the skill wording.
- docs/ROADMAP.md: list this slice only; the Kimi row and other housekeeping wait for
  the separate pull request.
- Pull request description: the deferrals, that PR #26 owns REQ-cli-012, TC-cli-0125
  and BUG-0026, and the byte counts of both next forms here.
