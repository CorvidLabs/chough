---
change: change-leaks
artifact: plan
---

# Plan

## Approach

Check the three free-text values Chough hands SpecSync for its committed records as every other write is checked, widen doctor's scan to the open changes without touching their records, and name the last echo sites by what they hold.

## Out of scope

- `.specsync/archive/`: SpecSync's authenticated history, not scanned.
- Redacting a finding under `.specsync/changes/` in place.
- The IDs' refusal, which stays as #413 approved.

## Steps

1. Failing tests: the refusals and `--redact` in `tests/change.rs`, the scan and the skip in `tests/leaks.rs`, the echo sweep with FOR_412 removed and the evidence ID in `tests/output_safety.rs`.
2. The checks in `src/change.rs`.
3. The scan in `src/doctor.rs`.
4. The echo sites and the evidence ID.
5. Docs, changelog, cases, verify, and the run on the real repositories (rule 3).

## Risks

- Pix and site would see their leak scans exit 1 on two matches in open changes' artifacts until the text is reworded.
- A redaction count line on standard error is the one difference from SpecSync's own output under `--redact`.

## Constraints consulted

LIFE-26; rule 3 (a new refusal is run on the real repositories and names a way out); #413's approved ID refusals; declare every owning module and one cases file per module; `pub(super)` helpers; no new dependency.
