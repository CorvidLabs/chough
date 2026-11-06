## ADDED

### REQUIREMENT REQ-cli-052

Responsive archived reports with bounded authenticated reuse. For a checkpoint whose native change is archived, `chough next` and `chough status` MAY reuse a native JSON response only after a live `specsync change status <id> --json` reported the same archived change with terminal evidence validity `authenticated-history`. The cached response SHALL be labelled as cached, SHALL retain the existing native summary and deterministic output bounds, and SHALL be used only by these read-only reports. Stage transitions, timing, retention, native verification and every lifecycle mutation SHALL continue to make their authoritative live native reads with the existing gate predicates, deadlines and output bounds. Active-change responses SHALL NOT be cached. Trace: DRIVE-2, LIFE-3, LIFE-6, LIFE-7, FLOW-2.

The cache identity SHALL cover the change ID, complete archive bytes and Git tree/carrying history, the resolved SpecSync executable bytes and recorded version, native policy/configuration/baseline bytes, and Git shallow, replace, graft and alternate-object inputs. A missing, malformed, unreadable, mismatched or unproven entry SHALL fall back to the existing single bounded live status read. Failed, timed-out, malformed and unknown live responses SHALL remain explicit and SHALL NOT be stored as authenticated. A live response cannot supply its own cached marker.

On the measured macOS arm64 Chough repository with 121 archives, a freshly built release `next` and `status` naming an archived change SHALL each finish at or below 2000 ms at p95 and 3000 ms for every run across ten warm runs after one priming run. A valid warm run SHALL make zero SpecSync status calls. Measurements SHALL identify source/binary/input identity and every run, and SHALL distinguish cold, warm, completed, failed and timed-out native reads.

Snapshot performance work SHALL preserve the exact legacy and content fingerprint streams, observation freshness, error visibility and existing bounds. The two ordered digest passes MAY execute concurrently; no persistent evidence-fingerprint cache or metadata-only freshness shortcut SHALL be introduced.

Acceptance Criteria
- Controlled tests prove cold recording, warm zero-status reuse, every named invalidation/fallback class, anti-spoofing, explicit live failures, and unchanged live native calls from stage, timing, retention and verification.
- Existing native gate predicates, deadlines, output bounds and refusal behavior pass unchanged, including when a report cache is warm.
- Legacy and content digest known answers and observation freshness results are identical before and after the snapshot change; worker failures remain visible.
- The real release benchmark records ten warm runs per command after one priming run and meets both numeric limits, with source, binary, input and native-call identities retained. A separate cold archived native measurement remains explicit.
- Failing-first logs, focused tests, mutation checks and the complete Fledge verification lane are recorded; no test, gate, bound or timeout is weakened.
