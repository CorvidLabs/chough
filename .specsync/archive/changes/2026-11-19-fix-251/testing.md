---
change: fix-251
artifact: testing
---

# Testing

`tests/doctor.rs::a_whole_repository_specsync_gets_the_narrowing_advice`, red on main, has three steps:
1. With `source_dirs = ["."]` and no policy, `chough init` prints the narrowing advice, naming the setting, the overlap refusal, the skill bundle and `["src"]`. `doctor` reports no finding.
2. With a policy whose `meaningful_paths` holds `.`, the finding's fix starts with the narrowing, and its advice names both settings and then the list.
3. Once both are narrowed, the advice is the list again.

Both files stay byte-identical throughout. Case `TC-records-a-whole-repository-specsync-gets-the-narrowing-advice` binds the test.

`tests/doctor.rs::root_level_code_gets_the_exclude_advice_instead_of_narrowing` (#354), red before this addition: a spec covering `generate.py` and `config.yaml` at the root gets the exclude advice and spec-sync#1083, with no folder to narrow to. Once those folders are excluded, only the list's part is left.
