## MODIFIED

### SPEC SECTION Error Cases

| Error | When | Behavior |
|-------|------|----------|
| Missing retention authority | Apply runs without a run consent or standing retention preference | Refuse without removing anything |
| Changed candidate | A candidate file changed, vanished or became referenced after preview | Retain or refuse it with an explicit outcome |
| Bound exceeded | An event, cycle, traversal or scan exceeds its bound | Refuse before checkpoint mutation, or report partial history |
| Anchor mismatch | The adopted legacy anchor no longer matches | Refuse new writes |
| Captured artifact identity changed | Bytes captured after eligibility validation do not match the eligible artifact digest | Refuse before this set's ledger write/removal; retain the replacement and restore any earlier removals |
| Unsafe restoration destination | A removed path or ancestor is recreated as a symlink | Keep its identity and target untouched; preserve original bytes in a safe named hold, continue remaining held-file restores and record honest restored accounting for every ledgered set |
