# GitHub stage labels

This template is optional. Copying it is what turns the labels on. Chough does not call GitHub, and `chough init` does not install these files.

The labels show the seven main stages, left to right:

| Order | Label | Color |
|---|---|---|
| 1 | `chough-stage:intent` | `#FF0000` |
| 2 | `chough-stage:design` | `#FF4D00` |
| 3 | `chough-stage:plan` | `#FF9900` |
| 4 | `chough-stage:build` | `#FFEE00` |
| 5 | `chough-stage:test` | `#00CC44` |
| 6 | `chough-stage:accept` | `#00BBFF` |
| 7 | `chough-stage:done` | `#8C00FF` |

GitHub lists labels alphabetically. The color and the description ("step N of 7")
are the progress cue.

## Copy

From the project root:

```sh
mkdir -p .github/workflows tools
cp integrations/github/chough-labels.yml .github/workflows/chough-labels.yml
cp integrations/github/chough-label-prs.sh tools/chough-label-prs.sh
chmod +x tools/chough-label-prs.sh
```

A project that does not have this Chough checkout copies the same two files from
a Chough source tree into those two paths.

The workflow's `push` trigger names `main`. A project whose default branch has
another name changes that one line. The script asks GitHub for the pull
request's base branch, so the diff follows that branch.

## What gets a label

The script reads `stage:` from the session files the pull request introduces:
`.chough/sessions/*/session.md` and `.chough/archive/sessions/*.md`. It does not
use a session's `pr:` field, and it does not read a session path that is a symlink.

When any introduced file is still an open session, that open session sets the
label and archived stories are ignored. When no open session remains, the
archived stories set the label. One distinct main stage among the chosen
sessions applies `chough-stage:<stage>` and removes any other stage label.
Zero chosen sessions, several stages among them, or a stage outside the main
line (`release`, `bugs`, `validate`, `librarian`, or a name that is not a stage)
removes every `chough-stage:*` label and applies none. `implement` and `verify`
are not stages. The workflow deletes those two labels if an older copy created
them.
