---
change: raise-the-checkpoint-session-byte-limit-to-131072-and-the-observation-pointer-limit-to-400
artifact: testing
---

# Testing

- `tests/cli.rs`: an exactly-131072-byte valid checkpoint is read by `context`; 131,073 bytes is refused naming 131072 (was 65,537 / 65536). Init over the bound still refuses before writing.
- `tests/stages.rs`: merge operand over 131,072 bytes refused in each position, naming 131072; merged result over the bound refused with inputs byte-identical.
- `tests/cli.rs`: 400 observation pointers validate, 401 refuse; 201 decisions still refuse; an evidence or bug record over 65,536 bytes still refuses.
- Full `fledge lanes run verify` with SpecSync 6.1.2.
