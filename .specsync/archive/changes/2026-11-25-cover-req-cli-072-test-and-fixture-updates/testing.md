## Testing

- `cargo test --locked` full suite green on the branch head (already run after the
  review fixes).
- Hosted Archive + verify lanes green after push.

## Requirement evidence

| Requirement | Evidence |
|---|---|
| (no-spec-change) | cargo test --locked: 1044 passed, 0 failed at the review-fix commit |
