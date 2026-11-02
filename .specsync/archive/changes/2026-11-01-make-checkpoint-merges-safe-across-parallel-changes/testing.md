---
change: make-checkpoint-merges-safe-across-parallel-changes
artifact: testing
---

# Testing

This is the verification plan, not a record that these checks have run.

## Failure first and later pass

1. Before any `src/` implementation edit, add the checkpoint merge regressions to `tests/stages.rs`. Include a real Git fixture with a committed schema 2 base, two branches and `.chough/session.json merge=chough-session`. Configure the same per-clone driver that the documentation gives, with the built `chough` on Git's PATH. Give ours and theirs compatible collection edits, including a one-side removal and an identical addition, while leaving the current-cycle group unchanged. Assert the exact merged bytes and clean Git status.
2. At the unmodified tip, first run the same branch fixture with no Chough driver configured and retain the current ordinary Git conflict or incorrect merged bytes. Then run the new regression with its driver configured and retain its failed assertion or unsupported `--merge` failure. Put both commands, exit statuses, stderr and resulting checkpoint bytes in `.chough/artifacts/checkpoint-merge-real-git-pre-fix.log`. The ordinary Git reproduction is required; an unsupported flag alone is not evidence of the current conflicting behavior. Record the actual failure with the existing observation schema and no case link:

   ```sh
   chough evidence add checkpoint-merge-real-git-pre-fix \
     --criterion FLOW-13 --criterion LIFE-3 --requirement REQ-cli-007 \
     --contract specs/cli/cli.spec.md --kind product-test --outcome failed \
     --environment local-macos-arm64 --method automated --track functional \
     --artifact .chough/artifacts/checkpoint-merge-real-git-pre-fix.log \
     --summary "Real Git cannot yet merge compatible parallel schema 2 checkpoint changes"
   ```

3. Run `chough bug search "checkpoint parallel merge"`. Unless an existing record is the same defect, use `chough bug add` with title, symptom, trigger, expected and actual text from this reproduction, `--environment local-macos-arm64`, both criteria, REQ-cli-007, `--classification product-defect`, `--scope in-scope` and `--observation checkpoint-merge-real-git-pre-fix`. Link the observation before implementation and use the returned bug ID. Do not allocate a legacy numeric ID or a new case ID.
4. After implementation, rerun the same unchanged real Git regression and save its log as `.chough/artifacts/checkpoint-merge-real-git-post-fix.log`. Record `checkpoint-merge-real-git-post-fix` with the same links and method but outcome `passed` and a summary of the clean compatible merge. Append it to the same bug with `chough bug update <returned-id> --observation checkpoint-merge-real-git-post-fix --status verified --note "The unchanged real Git regression passed after the merge implementation"`. The pass must be observed strictly later than the latest linked failure. Never rewrite the failed observation or infer a pass from a structural check.

## Requirement evidence

| Requirement | Test |
|---|---|
| REQ-cli-007 | For every exact-string history and reference collection, use a base with retained and removed entries, ours with a removal plus additions, and theirs with different additions. Assert surviving base order, then ours-only order, then theirs-only order. An identical addition on both sides occurs once, a one-side removal stays absent, and strings that differ by case, whitespace or punctuation remain distinct. Cover references to criteria, requirements, cases, design and files, plus decisions, open questions, completed work, observations and bugs. |
| REQ-cli-007 | Exercise the current-cycle group as a unit. A one-side change to each of objective, intent scope, modules, stage, native change and next action selects that side's whole group. Identical two-side changes pass. Disjoint edits on the two sides, even to different members, refuse with a current-cycle conflict diagnostic naming the differing members rather than combining fields. Additive collections still merge when the selected current-cycle group is unchanged. |
| REQ-cli-007, REQ-cli-013 | One-side and identical changes to `run_consents`, including complete consent fields and timestamps, are retained exactly with the selected group. Divergent consent changes refuse. One-side and identical blocker or awaiting-human changes behave the same way, and conflicting gate changes refuse without appending gate-opened or gate-closed transitions. Merge never grants a new consent, writes a standing preference or authenticates a human decision. |
| REQ-cli-007 | For every refusal, hash base, ours and theirs before and after and snapshot the surrounding `.chough/` records. All input hashes stay equal and no transition, evidence, bug, preference, approval, review or consent record appears. Successful explicit merges replace only `ours`; base, theirs and unrelated records stay byte-identical. The canonical live checkpoint remains `.chough/session.json`, with no second store or duplicate native lifecycle state. |
| REQ-cli-006, REQ-cli-007 | Put schema 1 in each input position and mix schemas 1 and 2 in both directions; every attempt refuses without migration or a schema 1 backup. Also refuse unsupported versions, malformed JSON and unknown top-level or nested fields in each position. Existing schema 1 context and status reads, explicit migration backup preservation and stage-command migration refusal remain green. |
| REQ-cli-006, REQ-cli-007 | Refuse an input over 65536 bytes in each position. Separately use individually valid inputs whose additions would exceed 200 entries or whose pretty-serialized merged result would exceed 65536 bytes. Refusal occurs before any write. Valid results at the existing bounds pass, and existing module, caller, ID, path and stage validation remains enforced. |
| REQ-cli-007 | In the real Git fixture, the compatible merge uses `.gitattributes` and the actual configured driver, produces the same result as the explicit command and leaves no Chough side-effect record. A second real merge with divergent current-cycle groups exits nonzero, leaves the working checkpoint equal to ours and reports the path unresolved. Git's own merge metadata is expected, but Chough writes no record. After `git merge --abort`, invoke `chough checkpoint --merge <base> <ours> <theirs>` directly on the same three fixture copies to prove the no-driver fallback and its refusal atomicity. |
| REQ-cli-001, REQ-cli-007 | In `tests/cli.rs`, initialize a real Git repository with the attributes declaration, hash `.git/config`, run `chough init`, and assert the config bytes are unchanged and local `merge.chough-session.driver` remains unset. The test separately configures the driver for the real merge, so documentation setup is not confused with init behavior. |
| REQ-cli-016, REQ-cli-019 | In `tests/package.rs`, install the retained pre-change `references/checkpoint.md`, assert its recorded LF SHA-256 digest, require `known-outdated`, update it to the new bundle and verify an edited old copy remains byte-identical. `chough skill status` reports the canonical `.agents/` and `.claude/` copies current after refresh, and the existing no-dash and complete-bundle checks pass. |

## Focused, full and CI-matching checks

Run `cargo test --locked --test stages`, `cargo test --locked --test cli` and `cargo test --locked --test package`. The stages invocation includes both real Git tests and must not use a simulated three-way text merge. Run the pre-fix and post-fix observations in the order above, then `cargo test --locked` and `cargo build --locked --release`. Where the declared minimum toolchain is installed, also run `cargo +1.88.0 test --locked`; otherwise leave the hosted MSRV result unclaimed.

With HI 0.8.0, Fledge 1.8.0 and SpecSync 6.1.2, run `fledge run fmt`, `fledge run lint` (Clippy over all targets with warnings denied), `fledge run test`, `fledge lanes run verify` and `fledge lanes run pre-push`. Verify includes the configured format, lint, test, HI, strict SpecSync, native audit, case-definition, exercise and build steps. Report hosted Ubuntu, macOS, Windows and MSRV checks only from their actual runs.

## Evidence boundaries

The two readable observation IDs are instances of the existing evidence scheme. No TC-cli number, case definition, observation schema or other identity format is added or changed. The pre-fix failure may become historical after implementation and remains reproduction evidence. Only the later real pass can verify the linked bug. Structural success, a local bug status and an actor label remain distinct from human acceptance, native finalization, merge, release and deployment.
