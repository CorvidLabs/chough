# Releasing Chough

These steps are for a human after the release preparation change merges. Nothing in
Chough or its CI runs them, and each is a separate decision (see the optional release
stage in [the skill](../skills/chough-workflow/references/release.md)). Use the
toolchain from [Install the toolchain](../README.md#install-the-toolchain).

Below, `X.Y.Z` is the version being released.

1. Confirm that CI on main is green at the merge commit, including `MSRV / 1.88.0`.
   For 1.0, also run the [flake window](ci.md#flakes-and-the-clean-window) check:
   `python3 .github/scripts/flake-window.py --save-input <input> --require-clean > <receipt>`
   must exit 0 with the verdict `clean`. Record it with `chough evidence add
   --criterion LIFE-21 --kind structural --outcome passed --environment <env>
   --summary "<verdict, window start, input sha256>" --artifact <receipt>`. It shows CI
   history, not product behavior. Keep the input file out of Git.
2. Check the package contents: `cargo package --list --locked` lists only `src/`,
   `skills/chough-workflow/`, `Cargo.toml`, `Cargo.lock`, `README.md` and `LICENSE`
   (plus the files cargo generates).
3. Dry run: `cargo publish --dry-run --locked`. Also check that the `chough` name on
   crates.io still has the expected owner (0xLeif on 2026-11-06, from
   `https://crates.io/api/v1/crates/chough/owners`); the README's first install
   command depends on it. If someone else holds it, stop and decide the name before going on.
4. Check the version and the change log: `Cargo.toml` says `version = "X.Y.Z"`, and
   in [CHANGELOG.md](../CHANGELOG.md) the entries that ship in this release sit under
   `## [X.Y.Z] - <release date>`, with a `[X.Y.Z]` link to
   `https://github.com/CorvidLabs/chough/releases/tag/vX.Y.Z` at the end. The release
   preparation change normally does this (0.4.0's did, in #116); if it did not, do it
   in a small docs pull request and merge that first.
5. Tag that commit `vX.Y.Z` and push the tag.
6. Publish: `cargo publish --locked`. crates.io keeps the README as published, so
   check it reads correctly first.
7. Create the GitHub release for `vX.Y.Z` from the change log section.
8. In a clean environment run `cargo install chough --version X.Y.Z --locked` and
   `chough --version`, and record what you saw as an installation observation.
9. In a small docs pull request, add or mark the `X.Y.Z release` row of the
   [roadmap's delivered table](roadmap-1.0.md#delivered) published, with the date.

A merge, a tag, a publish, a GitHub release and a clean install are separate
observations; one does not imply another.
