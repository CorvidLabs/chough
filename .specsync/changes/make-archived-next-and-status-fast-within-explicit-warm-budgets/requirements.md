---
change: make-archived-next-and-status-fast-within-explicit-warm-budgets
artifact: requirements
---

# Requirements

Proposed REQ-cli-053, traced to DRIVE-2, LIFE-3, LIFE-6, LIFE-7 and FLOW-2:

1. On the measured 121-archive macOS arm64 Chough repository, release `next` and `status` for a session naming an archived change each complete at or below 2000 ms at p95 and 3000 ms for every run across ten warm runs after one priming run.
2. A warm report with a valid entry reuses only a live SpecSync `authenticated-history` verdict and makes zero SpecSync status calls. The output labels the read as cached and retains the native summary fields and deterministic output bounds.
3. Cold, missing, mismatched, malformed or unreadable entries fall back to one existing bounded live status read. Failed, timed-out, malformed and unknown responses remain explicit and are never recorded as authenticated.
4. Cache identity covers the change ID, archive bytes and Git tree/carrying history, resolved SpecSync binary bytes and recorded version, native configuration/policy/baseline, and Git shallow, replace, graft and alternate-object inputs. Any mismatch or check failure goes live.
5. Cached verdicts serve only `next` and `status`. Stage transitions, timing, retention, native verification and every other lifecycle mutation keep their live reads and existing gate, output, process and timeout behavior.
6. Snapshot optimization preserves the exact legacy and content fingerprint streams, observation freshness, error visibility and input bounds. No persistent evidence-fingerprint cache is introduced.
7. Controlled failing-first tests cover reuse, every invalidation class, fallback, anti-spoofing and live-gate call counts. Real before/after release measurements and the full Fledge verification lane establish the result; structural checks alone are insufficient.
