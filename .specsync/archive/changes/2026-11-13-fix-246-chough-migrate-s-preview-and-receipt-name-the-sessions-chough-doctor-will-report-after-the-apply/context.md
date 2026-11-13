---
change: fix-246-chough-migrate-s-preview-and-receipt-name-the-sessions-chough-doctor-will-report-after-the-apply
artifact: context
---

# Context

corvid-agent, adopting Chough 0.6.0 in CorvidLabs/spec-sync and CorvidLabs/raven, found
that `chough migrate`'s preview listed three follow-ups (`records_in_verification`,
the merge-driver lines and `chough skill update`) and nothing else. Right after the
apply, though, `chough doctor` reported `finished_session` and `stale_gates`
findings. A stale gate's withdrawal is a decision, so the person consenting saw it
only after the apply, and the adoption PR had to go back to the human. Filed as GitHub
#246. t38b reproduced it with a 0.5.x fixture holding a finished session and an
archived-change session waiting on two gates (red at 129a4dbb).

On 2026-11-13T04:37Z Leif approved the scope ("Chough #246 migrate preview", relayed
by t370): the migration preview and receipt also list the sessions doctor will flag
right after the apply, as a small migration wording change. A docs-only line was the
fallback, and it isn't needed.
