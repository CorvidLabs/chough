---
change: raise-the-checkpoint-session-byte-limit-to-131072-and-the-observation-pointer-limit-to-400
artifact: testing
---

# Testing

- `tests/cli.rs`: an exactly-131072-byte valid checkpoint is read by `context`; 131,073 bytes is refused naming 131072 (was 65,537 / 65536). Init over the bound still refuses before writing.
- `tests/stages.rs`: merge operand over 131,072 bytes refused in each position, naming 131072; merged result over the bound refused with inputs byte-identical.
- `tests/cli.rs`: 400 observation pointers validate, 401 refuse; 201 decisions still refuse; an evidence or bug record over 65,536 bytes still refuses.
- Full `fledge lanes run verify` with SpecSync 6.1.2.

## Requirement evidence

| Requirement | Test |
|---|---|
| REQ-cli-032, TC-cli-checkpoint-byte-bound-is-131072 | `tests/cli.rs::checkpoint_byte_bound_is_exactly_131072`: a valid checkpoint padded to exactly 131,072 bytes is read by `context`; 131,073 bytes is refused naming 131072 with the file byte-identical; a checkpoint whose content exceeds 131,072 bytes is refused. `oversized_records_fail_explicitly` (131,073 bytes, names 131072) and `init_rejects_unreadable_session_sizes_before_writing` (objective serialized over 131,072 bytes) keep their assertions with only the bound updated. |
| REQ-cli-032, TC-cli-observation-pointer-bound-is-400 | `tests/cli.rs::observation_pointer_bound_is_400_and_other_lists_stay_at_200`: 400 observation pointers validate, 401 are refused, and 201 `decisions`, `completed` or `bugs` entries are still refused. |
| REQ-cli-032 | `tests/cli.rs::records_other_than_the_checkpoint_keep_the_65536_byte_bound`: a preferences record over 65,536 bytes is still refused naming 65536. |
| REQ-cli-032, REQ-cli-007, TC-cli-checkpoint-merge-enforces-the-raised-bounds | `tests/stages.rs::checkpoint_merge_enforces_input_collection_and_output_byte_bounds`: an input over 131,072 bytes is refused in each position naming 131072; an input of exactly 131,072 bytes merges in each position; a merged result over 131,072 bytes from two in-bound inputs is refused with inputs byte-identical; 200 + 200 added observations merge to 400 and 200 + 201 are refused; the 300-decision and 201-consent refusals are unchanged. |

Each new or updated test fails with the bounds set back to 65,536 bytes and 200 observations (checked locally).
