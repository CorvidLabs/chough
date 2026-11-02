---
change: read-report-case-definitions-once-per-request-and-expose-local-phase-costs
artifact: design
---

# Design

Introduce a root-bound, request-local case-digest index. It lazily calls the existing
structural loader once, preserving its bounds, global duplicate validation and errors.
It reuses the existing Located::digest representation, including legacy numeric case
digests and environment omission semantics. Requested digests may be memoized inside
this context; parse failures are retained only for that request. No disk cache,
process-global cache, invalidation watcher or second checkpoint is introduced.

Share one index between a status request's observation and reference-label passes.
Use one index for next's reference pass and one for each existing Labels owner.
The context is bound to one root and dropped at request end. Native reads and gates
receive no cached response from this context. Writers keep their existing validation.

Add bounded opt-in stderr phases around Git enumeration, content hashing, evidence
parsing, artifact checks, case loading, report reference work and rendering. Emit
case-suite load and lookup counts from the request context. Nested phase durations
are attribution intervals and must not be added as independent totals. Keep the
existing native trace state/duration and timing phases intact.

Existing native latency remains independent. Published SpecSync 6.1.2 source appears
to authenticate the terminal roster for an archived status request, but that is a
source hypothesis; this change does not alter or bypass native authentication.
