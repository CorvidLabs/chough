---
change: fix-438
artifact: design
---

# Design

Treat a valid grouped reply for the selected session as one checkpoint update. Hold the existing writer lock before loading the session to be changed. Recheck the selected list, membership, open gates, digests and relay data on that snapshot. Build the closed decisions and Timeline entries in memory using the same rules as an individual checkpoint close. Preserve the existing order of gate entries and the next action when the last gate holding the current stage closes.

Run the ordinary checkpoint renderer against the whole result while still holding the lock. Include the native command and actor text and the direct-write entries that will be published. The rendered candidate must pass the same validation, leak checks, Timeline count and byte limits as a single save. Use the current atomic file replacement to publish one complete session after every check and any native step succeeds.

A valid group may contain one gate that runs a native step. Run that step as the configured human after local preflight and before publishing any Chough closure. A SpecSync refusal keeps all named Chough gates pending. A filesystem failure after native success can leave an already recorded native decision while all Chough gates remain pending. On retry, inspect the native state and record. Accept a fresh invocation only when the native step is still due, or when an authoritative matching result can be used safely. If the result is uncertain or recorded without a safe match, refuse before invoking the native step and give recovery that names the native record to inspect. The existing native relay forwards words, place and recording agent without the Chough summary digest. Treat the digest alone as insufficient proof of a native outcome.

Collect pre-existing direct native writes once. Stage their Timeline entries before the grouped closes. After publishing, use the existing checkpoint baseline settlement and report a settlement failure as a diagnostic on the committed result, consistent with a single close.
