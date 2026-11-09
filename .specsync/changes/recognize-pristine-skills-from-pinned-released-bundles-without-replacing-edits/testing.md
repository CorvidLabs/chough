---
change: recognize-pristine-skills-from-pinned-released-bundles-without-replacing-edits
artifact: testing
---

# Testing

## Requirement evidence

| Requirement | Planned check | Evidence boundary |
|---|---|---|
| REQ-cli-016 | New tests/skill_releases.rs; offline5tags×24files, all6host routes; byte/mode-edited and wrong-filename digest controls; current/missing/legacy original controls | Future compiled RED/SAME GREEN required; existing private skill control has54 failed pristine rows and720 preserved edited rows, not implementation proof |

## Runtime regression plan

Load only the pinned120-mapping manifest and51 exact byte blobs, verify SHA/length/tag commit identity before invoking the tested binary in private roots. Assert every pristine released file is current/known-outdated, then byte-equals the embedded current copy after update. Single-byte/appended-text/CRLF mutations and known bytes at the wrong filename remain edited and preserve bytes/modes; no digest-only or newline normalization loophole. Preserve shared Claude/OpenAI and otherhost routing, current/missing behavior and explicit legacy handling. Original focused skill controls remain unchanged.

## Commands and limits

Future leased Fledge run test -- --locked --test skill_releases, followed by existing focused skill tests and declared fmt/lint. Not run during preparation. Fledge/native structural checks are not product assertions. No moving-network test, script execution, global install, fresh host discovery, Windows, full/Trust/site/hosted acceptance claim. All failures/raw identities retained.

## Retained actual execution results

t2ns at6bb513b: seven ran,5pass/2fail,54 pristine failures in each intended recognition/update test. t2p7 atd9a9d924: SAME7/7, existing optional4/4, cli3/3, package16/16, then fmt0/lint0. Actual final compiler end2026-11-09T05:05:12.506196Z; lease released before bookkeeping. Complete source/input equality established. No new test or case run occurs while recording these retained receipts. Original setup probe failure remains separate from product RED; normal subprocess restore_signals controls verified. Full/Trust/site/hosted/human gates remain unrun.
