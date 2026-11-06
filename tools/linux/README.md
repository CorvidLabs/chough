# Linux test environment

Chough's tests and verify lane also run on Linux, in local Docker images built from
this directory (REQ-cli-058, DRIVE-22).

| Image | What it adds |
|---|---|
| `chough-linux:1` | The toolchain, as root: Rust, git, jq, python3, procps, rustfmt, clippy, and the pinned `hi` (human-intent), `fledge` and `specsync`. |
| `chough-linux:2` | A uid-501 `dev` user, matching the macOS account that owns the checkout. |
| `chough-linux:3` | rsync, for runners that sync a worktree into a volume. |

- `fledge run linux-image` builds all three. Run it after a Docker reset.
- `fledge run linux-verify` runs `fledge lanes run verify` on the current commit
  inside `chough-linux:3`. The commit is cloned from a Git bundle, so the container
  never reads the host's `.git`.

The tool versions come from the pins in `.github/workflows/ci.yml`, read by
`pins.sh`; the Dockerfile names none. `pins.sh` refuses when a pin is missing or the
workflow's specsync pins disagree. `linux-verify` exits 3 before verifying when a
tool in the image is not the pinned version, and names `fledge run linux-image`.
