---
change: pin-every-history-and-inbox-bound-at-exactly-its-value-154-ch-19
artifact: design
---

# Design

## Approach

The history tests build exact layouts with the padded synthetic event chain. Inbox entries are padded with JSON whitespace. Each test asserts the observable result at the bound and one past it: events read, the refusal text, or listing.

## Risks

The 16 MiB test writes about 16 MiB of event files, which takes about 0.4 s.

## Constraints consulted

REQ-history-003 and REQ-history-009. The rule against changing assertions or bounds.
