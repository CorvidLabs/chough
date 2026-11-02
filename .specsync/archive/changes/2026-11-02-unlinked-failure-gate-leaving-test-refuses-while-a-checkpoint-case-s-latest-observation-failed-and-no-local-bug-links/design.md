---
change: unlinked-failure-gate-leaving-test-refuses-while-a-checkpoint-case-s-latest-observation-failed-and-no-local-bug-links
artifact: design
---

# Design

## Rule

`unlinked_failures(root, session)`, in `src/stage.rs`: for each case ID in
`session.references.cases`, take the observations from `evidence::load_all` whose
`cases` name it, pick the greatest `(observed_at_unix_seconds, id)`, and keep it when
its outcome is `failed` and no entry of `bugs::observation_references` contains its
ID. The result is sorted by case ID. Pruned observations are not loaded, as
elsewhere. One evidence read and one bug read per call; no native read.

## Where it applies

- `move_forward`, beside the blocking-bug check, when `from == "test"` and `to ==
  "accept"` (covers `advance` and `skip`): refuse with
  `{kind} refused (refused:unlinked-failure): case <case> latest observation <obs>
  failed and no bug links it; rerun the case until it passes, or link it with chough
  bug add ... --observation <obs> or chough bug update <bug> --observation <obs>`
  (the first failure is named, with the count of others).
- `assess`, at stage test after the blocking-bug branch: the next valid action names
  the same refusal and the remedy command, so `next` and `status` agree.

Blocking bugs are checked first, so their refusal is unchanged.

## Not changed

`src/main.rs`, the next view fields and bounds, evidence and bug record formats,
the skill bundle and `examples/`. The repository's own checkpoint has no
referenced case whose latest observation failed unlinked (checked at 825a380), so
its flow is unaffected.

## Estimate

Small: one helper, one prerequisite and one `assess` branch, about 60 lines, plus
tests. Main uncertainty: a textual merge with PR #37 in `src/stage.rs`.
