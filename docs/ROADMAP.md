# Roadmap

Durable intent is in [HI](../hi/life.md). This roadmap records proposed work and
sequencing, not human acceptance or proof of delivery.

| Stage | Outcome | Evidence needed |
|---|---|---|
| First foundation | Current agent uses a discoverable skill; CLI preserves files, assembles context, reports native status, and records observations. | Isolated Git fixtures, Rust checks, native structural checks, feature/bug/resumption walkthrough, rendered diagrams. |
| First real project loop | Apply the skill to a real scoped feature and a reproduced bug. | Genuine scope decision, product test results, native change verification, human review, same-PR archive, final delivery observation. |
| Context refinement | Select less irrelevant context while retaining material decisions and lessons. | Cases demonstrating successful resumption and explicit missing/stale inputs; no silent source truncation. |
| Host portability | Validate supported host discovery and command mappings. | Real host-specific exercises; a command name alone is insufficient. |
| Native gate adapter | Reuse Trust where its policy is appropriate. | Version-compatible integration and tests for degraded, failed, unavailable, baseline, and proposed-commit outcomes. |
| Rune proof of concept | Optional execution/session bridge for an explicitly chosen use case. | Bounded session lifecycle, per-send output, human takeover, process-settled versus task-success distinction. |
| Context/gap adapter | Explore read-only Atlas input. | Current SpecSync compatibility and focused useful output established first. |

The next useful product exercise is a real feature/bug pair on a chosen small
project. It will test whether the skill asks the right questions, keeps enough
context, and makes the next action obvious. More commands should follow observed
friction from that exercise.
