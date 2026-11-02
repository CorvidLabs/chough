## ADDED

### REQUIREMENT REQ-cli-028

Request-local reporting reads. Within one report request, the helper SHALL structurally load the case-definition suite at most once, and SHALL not load it when no bound-case digest is requested. It SHALL reuse the existing case-digest representation and failure semantics across observation and reference-label checks, with an ephemeral context bound to one project root. No result SHALL be reused by a later request. Source, artifact, environment and case changes SHALL retain their freshness effects, and native reads and lifecycle gates SHALL remain authoritative and unchanged. Trace: LIFE-3, LIFE-6, LIFE-7, LIFE-10, FLOW-13.

Opt-in CHOUGH_TRACE=1 diagnostics SHALL report bounded stderr-only local phase durations and exact case-suite load/lookup counts while preserving standard output, record bytes and exit results. Measurements SHALL identify the revision, freshly built binary, environment, inputs and deadlines, distinguish first-process from repeated runs without assuming OS caches were purged, and distinguish completed native reads from timed-out or unavailable work. No persistent cache, new runtime or duplicate lifecycle state SHALL be introduced.

Acceptance Criteria
- Multiple observations and referenced labels in one next/status request cause at most one structural suite load; no bound cases cause zero loads. Legacy numeric and current slug digests remain unchanged.
- Malformed, duplicate, retired, missing and unsafe case definitions retain explicit existing outcomes. A later request observes changed source, case definitions, artifacts and environment rather than reusing an earlier result.
- Trace on and off preserve stdout, exit results and every project record, including refusal paths. Existing native call counts, deadlines and invalid-evidence gate refusals remain enforced.
- Before/after measurements use the same input root and explicit binary identities, report local work separately from native wait, and retain incomplete reads as partial or unknown. Native slowness is not reported fixed by a local optimization.
