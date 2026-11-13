---
change: fix-249-chough-passes-a-relayed-decision-s-words-place-and-recording-agent-to-specsync-s-review-and-approve
artifact: context
---

# Context

t384's reviews of #242 and #244 found that SpecSync's `review.json` for a review Chough
ran names the reviewer (0xLeif) and the verdict, but not the relay: Leif's words, where
he said them, and the agent that recorded them. `src/humans.rs` passes only
`--reviewer` and `--verdict` to `specsync change review`, and `--actor` and `--note`
to `specsync change approve`, where the relay lives only in the note's text, as
REQ-records-008 words it. SpecSync 6.3.0, which Chough installs, accepts `--words`,
`--where` and `--recorded-by` on both commands. t370 reported it; it is recorded as
`.chough/bugs/2026-11-13-chough-change-ship-s-native-review-drops-the-relay-review-json.md`
and filed as GitHub #249.

On 2026-11-13T03:44Z Leif chose "Reviews and approvals (Recommended)" (relayed by
t370): both records carry his words, where he said them and who relayed them; t38b
builds it, and an issue gets filed. The note on approvals stays as it is.
