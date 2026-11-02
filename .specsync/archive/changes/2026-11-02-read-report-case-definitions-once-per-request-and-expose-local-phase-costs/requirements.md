---
change: read-report-case-definitions-once-per-request-and-expose-local-phase-costs
artifact: requirements
---

# Requirements

REQ-cli-028 adds request-local case-definition reuse and opt-in local attribution,
traced to LIFE-3, LIFE-6, LIFE-7, LIFE-10 and FLOW-13. The semantic delta is the
source of proposed canonical wording.

Observable invariants include one structural case-suite load at most per report,
zero when no bound case is queried, shared reuse between status observations and
reference labels, and fresh contexts for later requests. Missing, retired, changed,
malformed, duplicate and unsafe definitions retain their existing outcomes. Source,
artifact and environment checks and native lifecycle gates remain authoritative.

Trace output stays on stderr only under CHOUGH_TRACE=1 and gives load/lookup counts
and named elapsed phases. It changes no stdout schema, record or exit result.
Measurements identify inputs, binary, environment, deadlines and incomplete reads.
No wall-time threshold or native speedup is promised.
