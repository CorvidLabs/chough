---
change: optional-github-template-labels-a-pull-request-with-the-seven-main-chough-stages
artifact: context
---

# Context

Jackdaw labeled pull requests from Chough session files with a workflow and a script that lived only in that repository. The label list was handwritten, so it grew `implement` and `verify`, which are SpecSync change statuses, not Chough stages. The seven main stages are `intent`, `design`, `plan`, `build`, `test`, `accept`, `done`, in that order, from `const ORDER` in `src/stage.rs`.

The user asked, on 2026-11-21, to add this to Chough after agreeing the shape: an optional template a project copies in, `chough init` unchanged, and the `chough` binary does not call GitHub. Labels stay a view of `stage:` in the session files the pull request introduces. SpecSync still decides whether a stage move is allowed.

Ruled out: installing the workflow from `chough init`, and putting GitHub calls in `src/`. Release and the side stages (`bugs`, `validate`, `librarian`) stay off the progress strip.
