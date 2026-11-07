---
change: prepare-the-0-5-0-release-set-the-package-version-to-0-5-0-and-date-the-change-log-section-for-0-5-0
artifact: testing
---

# Testing

- **New check:** `the_package_version_has_a_dated_section_and_a_release_link` in tests/changelog.rs. It requires the `CARGO_PKG_VERSION` section to be dated `YYYY-MM-DD`, to list at least one entry, and to have a `[X.Y.Z]: https://github.com/CorvidLabs/chough/releases/tag/vX.Y.Z` link. It passes at 0.4.0 and fails first at 0.5.0, before the change log is dated.
- **Release steps from docs/releasing.md:** `cargo package --list --locked` lists only `src/`, `skills/chough-workflow/`, the Cargo files, `README.md` and `LICENSE`. `cargo publish --dry-run --locked` succeeds. The crates.io owner is checked.
- **Lanes:** `fledge lanes run verify` and `fledge run linux-verify` pass.
