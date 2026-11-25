## Testing

- The check script runs in CI (docs lane) and locally; its failure output names the
  offending file + line.
- Human pass over the four reference files: every `specsync change` fenced command is
  either one of the four labeled exceptions or a chough form.
- No product code changes; `cargo test` unaffected but run once as regression.

## Requirement evidence

| Requirement | Evidence |
|---|---|
| REQ-cli-072 | (to fill after implementation: check-script run output + grep walk) |
