# Releasing Chough

These steps are for a human after the release preparation change merges. Nothing in
Chough or its CI runs them, and each is a separate decision (see the optional release
stage in [the skill](../skills/chough-workflow/references/release.md)). Use the
toolchain from [Install the toolchain](../README.md#install-the-toolchain).

1. Confirm that CI on main is green at the merge commit, including `MSRV / 1.88.0`.
2. Check the package contents: `cargo package --list --locked` lists only `src/`,
   `skills/chough-workflow/`, `Cargo.toml`, `Cargo.lock`, `README.md` and `LICENSE`
   (plus the files cargo generates).
3. Dry run: `cargo publish --dry-run --locked`.
4. Date the change log: replace `Unreleased` in the `0.1.0` heading of
   [CHANGELOG.md](../CHANGELOG.md) with the release date, in a small docs pull
   request, and merge it.
5. Tag that commit `v0.1.0` and push the tag.
6. Publish: `cargo publish --locked`. crates.io keeps the README as published, so
   check it reads correctly first.
7. Create the GitHub release for `v0.1.0` from the change log section.
8. In a clean environment run `cargo install chough --locked` and
   `chough --version`, and record what you saw as an installation observation.

A merge, a tag, a publish, a GitHub release and a clean install are separate
observations; one does not imply another.
