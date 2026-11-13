---
change: fix-246-chough-migrate-s-preview-and-receipt-name-the-sessions-chough-doctor-will-report-after-the-apply
artifact: design
---

# Design

Doctor's per-session check in `src/doctor.rs` becomes one function that takes a
parsed session and its unfolded inbox count. Doctor calls it on disk, with no change
in what it reports. `chough migrate`'s plan in `src/migrate.rs` calls it on each
planned `.chough/sessions/<key>/session.md`, parsed from the planned bytes, with the
planned inbox count, and turns each finding into a follow-up. The archived native
changes it reads are already present before the apply. So the preview, the receipt
and doctor share one computation. `docs/cli.md`'s Migration section and the
CHANGELOG say so.
