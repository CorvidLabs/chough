---
change: one-librarian-default-in-the-requirements-req-cli-013-defers-to-req-cli-015-on-124-d8
artifact: testing
---

# Testing

- tests/optional.rs `the_requirements_give_the_librarian_one_default_and_it_is_on`: any sentence in specs/cli/requirements.md that names the librarian, a default and `off` must say it is on by default. It fails first, on REQ-cli-013's sentence ending "...capture-only and off respectively".
- `standing_preferences_need_a_source_and_stay_separate_from_consent` still asserts that the runtime reports `librarian: on`, with origin `default`.

## Requirement evidence

| Requirement | Evidence |
|---|---|
| REQ-cli-013 | tests/optional.rs `the_requirements_give_the_librarian_one_default_and_it_is_on`, `standing_preferences_need_a_source_and_stay_separate_from_consent` |

## Lanes

`fledge lanes run verify` passes on Linux.
