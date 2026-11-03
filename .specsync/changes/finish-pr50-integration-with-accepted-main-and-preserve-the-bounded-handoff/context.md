# PR50 final integration and bounded handoff

## Actual authority

Acting project agent: corvid-agent, root session. Actual human sources:0xLeif said "I already gave it some approval and told it can do any changes. It just needs a PR.", "Can you finish up all these?", "finish up ur prs and then check everyhting", explicitly "Merge approved, green PRs and resynchronize", then "get the 3 prs merged?" in this conversation on2026-11-02. Those authorize this bounded integration, proof-preserving cleanup, qualification and green reviewed merge. Agent GitHub comments coordinate ownership and do not create human authority. No specific new capacity values are approved or inferred from "2′". Existing through-PR implementation authority is distinct from the final independent review and green merge gates.

## Goal and approach

Import accepted main747a12cc6fc396f0e8646fee1f2801e02b7d3035 into PR50 parent6715944ff6c4d0423f59e38d76a9cba6ea89e708 with normal Git merge. Preserve current main report reads, identity hooks/tests and CI checker; preserve reviewed CLI fixes and their tests. Canonical main25 remains authoritative.

Use exact raw checkpoints from base16fe55d, parent671 and main747a. The owner-selected projection moves only the reviewed10obsolete coordination strings and41completed work notes verbatim into .chough/artifacts/pr50-checkpoint-coordination-index/policy.json, with full source indices/hashes and frozen raw inputs. Preserve every other decision, all human sources/limits, all200active observation IDs and all24existing direct record pins. The measured prototype is58567bytes; revalidate actual live cycle and final refs. Each genuine new observation gets its own exact .chough/evidence/<id>.json file reference, protecting its record and artifacts.

## Out of scope

PR51capacity/LIFE-11, changed reader/output/time/item limits, source evidence rewrites, archive edits, cleanup/prune application, new cold-start proof, release and deployment.

## Ownership and estimate

Root owns integration coordination, native artifacts, checkpoint/projection/evidence, review/ship and guarded publication. A worker may own only declared Rust/example/doc merge conflicts. All workers preserve other edits. Relative size: small source adaptation, moderate provenance/qualification work; native/hosted latency is uncertain and no target duration is claimed.

## Constraints consulted

AGENTS, INTENT, LIFE-1/2/3/6/7/8, FRD, cli contract/companions, workflow plan/lifecycle, context.md lesson about final verification covering Chough records, source-working-content freshness, and the prior raw/refused/selected projection proofs. Bare union and the decision-only projection are rejected as over-bound or without practical headroom.
