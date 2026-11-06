---
change: pin-every-history-and-inbox-bound-at-exactly-its-value-154-ch-19
artifact: testing
---

# Testing

## tests/history.rs

- `recent_traversal_stops_at_exactly_512_kib`:
  - with 100 events of 8,192 bytes, `status` reads exactly 64 events (524,288 bytes);
  - with the 64th event read one byte larger, it reads 63 (516,096 bytes).
- `a_cycle_of_exactly_16_mib_is_full_and_one_byte_more_is_over_its_bound`:
  - a cycle of exactly 16,777,216 bytes is read whole, and `stage iterate` refuses with "capacity reached";
  - one byte more refuses with "partial history";
  - the checkpoint is unchanged either way.
- `a_receipt_of_exactly_1_mib_is_read_and_one_byte_more_is_refused`:
  - a receipt of exactly 1,048,576 bytes lets an interrupted prune resume;
  - one byte more refuses, and the remaining files stay.

## tests/sessions.rs

- `an_inbox_entry_of_exactly_16_kib_is_read_and_one_byte_more_is_refused`: an entry file of 16,384 bytes is listed; at 16,385 bytes, listing refuses and names 16384.
- `an_inbox_holds_200_unfolded_entries_and_refuses_the_201st`: the 201st add refuses, naming 200 unfolded entries and the fold.

## Mutation check

Each test failed against its loosened bound. After these tests, all 11 mutations are caught.

## Lanes

`fledge lanes run verify` passes on Linux.
