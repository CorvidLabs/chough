---
change: correct-prune-capture-authority-and-unsafe-rollback-for-pr-197
artifact: research
---

# Research and executable failure-first evidence

Base for both disposable probes: `1fc4c67b59f05097cab4cafa446aa58198f8ee56`. Actual real-tree source remains frozen. The source copy and its target directory are private to this probe; no shared CARGO_TARGET_DIR, native approval/review probe or real-project retention apply was used.

The accepted source read agrees with both reports: the artifact capture currently accepts an unconstrained later fs::read as removal authority, and rollback calls local_path with an escaping `?` before its per-file result. The following executed tests demonstrate the consequences, not just matching helper names.

## capture

Thread: https://github.com/CorvidLabs/chough/pull/197#discussion_r4502629883
Exact argv: `["cargo", "test", "--locked", "--bin", "chough", "evidence::tests::pr197_captured_replacement_bytes_survive", "--", "--nocapture"]`
Working directory: `/private/tmp/sol-ch65-p1-czcds90c`
Exit: **101**; **1 executed test(s) failed**, no zero-test or compile-only failure.
Raw receipt: `/private/tmp/sol-ch65-p1-czcds90c/capture-red.log`; SHA-256 `43fb50e2daf6d21cb0928cc2eabef4b717883e986147edf5ed6da23eb0c8ed5f`.

```text
capture probe: hook=true; pruned=Ok(Array [String("target")]); writer_exists=false
assertion `left == right` failed: changed writer bytes must survive capture/removal
  left: None
 right: Some([115, 101, 99, 111, 110, 100, 32, 119, 114, 105, 116, 101, 114, 10])
test evidence::tests::pr197_captured_replacement_bytes_survive ... FAILED
test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 74 filtered out; finished in 0.04s
```

## unsafe-restore

Thread: https://github.com/CorvidLabs/chough/pull/197#discussion_r4502629890
Exact argv: `["cargo", "test", "--locked", "--bin", "chough", "evidence::tests::pr197_unsafe_restore_", "--", "--nocapture"]`
Working directory: `/private/tmp/sol-ch65-p1-czcds90c`
Exit: **101**; **2 executed test(s) failed**, no zero-test or compile-only failure.
Raw receipt: `/private/tmp/sol-ch65-p1-czcds90c/unsafe-restore-red.log`; SHA-256 `17d392cb1a85724ac271939ede132821a71a3c4b6a30b54b8522f2f0062a6ced`.

```text
unsafe restore probe: ancestor=false; hook=true; hold_exists=false; earlier_artifact=false; earlier_record=false; restored_events=0; error=symlink is not an allowed source: /var/folders/5k/616qy7sj28l0d53k4c6w_4x00000gn/T/.tmppk6GSN/.chough/artifacts/pr197-unsafe/b.log
assertion `left == right` failed: unsafe destination must preserve original bytes in the safe hold
  left: None
 right: Some([117, 110, 115, 97, 102, 101, 32, 111, 114, 105, 103, 105, 110, 97, 108, 10])
unsafe restore probe: ancestor=true; hook=true; hold_exists=false; earlier_artifact=false; earlier_record=false; restored_events=0; error=symlink is not an allowed source: /var/folders/5k/616qy7sj28l0d53k4c6w_4x00000gn/T/.tmpZGrfgF/.chough/artifacts/pr197-unsafe
assertion `left == right` failed: unsafe destination must preserve original bytes in the safe hold
  left: None
 right: Some([117, 110, 115, 97, 102, 101, 32, 111, 114, 105, 103, 105, 110, 97, 108, 10])
test evidence::tests::pr197_unsafe_restore_file_symlink ... FAILED
test evidence::tests::pr197_unsafe_restore_ancestor_symlink ... FAILED
test result: FAILED. 0 passed; 2 failed; 0 ignored; 0 measured; 73 filtered out; finished in 0.05s
```

The regression-only patch is retained at `/private/tmp/sol-ch65-p1-czcds90c/regressions.patch`, SHA-256 `c4f7252835683c3a20d6c5b1fe3a58d4429c5e41b975672442792eb5f68560cb`. It adds only three tests and cfg(test) instrumentation to the disposable source. No fix has been applied or claimed passing. These disposable reds establish the gate; real-tree failure-first commits/runs must still follow scope approval.
