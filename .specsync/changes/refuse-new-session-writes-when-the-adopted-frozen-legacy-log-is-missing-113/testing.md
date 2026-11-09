---
change: refuse-new-session-writes-when-the-adopted-frozen-legacy-log-is-missing-113
artifact: testing
---

# Testing

## Completed private evidence

Eight cases on exact older INT01 artifact, source association limits in external runtime-source-identity.json. Open-holder missing log raw0 and archived-holder missing log raw0 each visibly changed checkpoint and added an event; unchanged contract oracle raw1 for each. Original adopter missing log and changed log open/archived raw1, unchanged checkpoint/history files and anchor diagnostic; their oracle raw0. Intact/restored/never-had-log positives raw0 and new event/checkpoint, oracle0. These are separate bounded older-runtime findings, NOT current-main test passes or failures.

## Proposed current-source regression (not executed)

External proposed-sessions-regression.rs contains a test that creates real fixture adoption, then a new session with legacy:null, arranges the holder open/archive/shelved/vault, asserts holder bytes retained, deletes only the disposable frozen log, and asserts stage advance refuses with an anchor reason. A file snapshot compares all session and history bytes plus active pointer, catching both selected and orphan event changes. A positive new-session fixture that never had a legacy log must actually write one event and keep legacy:null. Existing intact/changed-log and vaulted/shelved tests remain byte-exact. This proposed file has not been inserted, formatted with Rust, compiled or executed; its syntax and behavior require the scheduled lease.

Future smallest Fledge test invocation is discovered task test with --locked --test sessions and focused test filter, then complete sessions/history targets as allocated; fmt/lint afterward. Assert actual Fledge/Cargo/rustc paths, versions/hashes and bounded jobs/threads/own target inside the named executing HC helper, retain HEAD/input identity and raw results. First commit tests-only, observe same assertions fail for real missing-log behavior, then patch and observe GREEN. Never count unrelated compile/setup failure as RED. Main-source execution, Rust bindings, full/site/Trust/hosted CI remain NOT RUN in this preparation. Native structural completeness/currentness is not runtime proof, and independent review/human acceptance are separate.
