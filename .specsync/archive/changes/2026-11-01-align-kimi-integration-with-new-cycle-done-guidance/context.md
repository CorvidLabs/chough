---
change: align-kimi-integration-with-new-cycle-done-guidance
artifact: context
---

# Context

PR #24 archived a native-next-action fix before main advanced. Main then merged PR #23 with REQ-cli-021, which requires a new-cycle action for an archived done checkpoint, and PR #25 with a distinct open BUG-0020.

Merge commit `b2e2f19d904aec2a5acd73569c8439f5583c01f9` contains the mechanical conflict resolution. It preserves both histories and moves the Kimi archived-resume defect to BUG-0024. The integrated code still has the earlier native-routing override, so it contradicts the newer approved REQ-cli-021. BUG-0025 records that integration defect.

0xLeif explicitly selected main's new-cycle behavior. This change implements that decision without a canonical contract delta. The earlier observations and frozen archives remain unchanged. The one coached Kimi run is still the only run; no fresh post-fix run is claimed.

The `.chough/` directory is deliberately excluded from affected paths so record writes do not stale native verification. Slug-based bug identity is a separate change. No new integration, runtime, service or duplicate lifecycle state is in scope.
