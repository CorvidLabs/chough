# Lesson bundle: recognize-pristine-skills-from-pinned-released-bundles-without-replacing-edits

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Recognize pristine skills from pinned released bundles without replacing edits
- **Kind**: BugFix
- **Specs**: cli
- **Paths**: src/skill.rs, tests/skill_releases.rs, tests/fixtures/skill-releases/manifest.json, tests/fixtures/skill-releases/blobs/00b3c3f4ffaaeeaaca6d829ff0277c9eb354ce0911f66f41a55639d27fc7d35d.txt, tests/fixtures/skill-releases/blobs/1022db57d0c114effc8669ede9ebb453f932109f608f883712725e57ba53551d.txt, tests/fixtures/skill-releases/blobs/10cd3cbe76a8a56e6e931bed15a42d4ec1a10ccc8a9091fac3f60ca507856069.txt, tests/fixtures/skill-releases/blobs/1492e0c5368df8762f09b2bb590a049f6dc88649c6003686951dddf6fe34b2bf.txt, tests/fixtures/skill-releases/blobs/18299d9d1adb30fb87a167a1a59ecd8595d1f53ec73522ccee257ea817641701.txt, tests/fixtures/skill-releases/blobs/187a887f2999e2d54350a56059a739f8379a7d77305ab53652da3b3eed8caa8d.txt, tests/fixtures/skill-releases/blobs/1a0a324a83658a1466ee6fe295ccfe9e980275f7abca809ef2927e71fb912663.txt, tests/fixtures/skill-releases/blobs/1c9ef93185390ee98aa4b4f2f9fbf2c442a003ad22097d0e74e4676b42404909.txt, tests/fixtures/skill-releases/blobs/23dcd8140f0d5bbbf77ea9a20ce37c43fde91345bf1cddf40b9336797a920a57.txt, tests/fixtures/skill-releases/blobs/28eccd903a89ff815e71e0cc3f81250e6c63bb986d93b0ee76b323180c4d92bf.txt, tests/fixtures/skill-releases/blobs/2b1aabe936ec051c040820625b7665b5d26706fa5e9396ddb4bfba8425ede78a.txt, tests/fixtures/skill-releases/blobs/2e26c47b349c4138d50eb78aa557822909c586802a9dbb12a46a466438d89b3b.txt, tests/fixtures/skill-releases/blobs/366259957ea1289ac4c9046a4c65b42cb4a7228ab93006ca9396bc5840d612e3.txt, tests/fixtures/skill-releases/blobs/367a9873293e79ca40c9634e2b2a12c3223074aaec161c7444690c2d02e12199.txt, tests/fixtures/skill-releases/blobs/370386488c8e5ff6c974ae9d3bd51998de31183868dfe48af44c0935a3019170.txt, tests/fixtures/skill-releases/blobs/3da19999ee4830a1fe1e0af63b13efc934f90d05678bbf70c3d79f39103c1150.txt, tests/fixtures/skill-releases/blobs/4013dc5ac116f3f32cffeb89ffcef957c3b402236000ea54c130ae41a02a96b5.txt, tests/fixtures/skill-releases/blobs/472396403a4f716f0183ec2c3495ac0ecf0dc412c1b61bb850cee767765e1f6a.txt, tests/fixtures/skill-releases/blobs/4995658517849e80e567a73eaf0399ec51123549a7e94649a17afa35ec40f20d.txt, tests/fixtures/skill-releases/blobs/4d0bf24ddd5e55493658339c76750a639a15997c4290a492cacda3550874f035.txt, tests/fixtures/skill-releases/blobs/4e085b88eaa66a59fd6c8ce7d3acb1201b46292603a2cf13ab5333fc5ea85d86.txt, tests/fixtures/skill-releases/blobs/51281ff6c9af4b939105cdf6ecf80d9dd11080dc8d682ecbba9d5b5c17d75c42.txt, tests/fixtures/skill-releases/blobs/5d7f85aa72b3bba8e5d83b593556bcdee244939b31038ed3e6000783aac5c4ea.txt, tests/fixtures/skill-releases/blobs/5f839a81742b9dc3df3b1278e340061a08e95e2a00e203f0f2554eab3aa3fed9.txt, tests/fixtures/skill-releases/blobs/658ce798826c4d685753af0735e5fc6a53685f1205b9b9902d625c63c71d277f.txt, tests/fixtures/skill-releases/blobs/733346e29c7a57461926a6c7d297face74fc291a2e5c7f17d4d57dcf3945ec99.txt, tests/fixtures/skill-releases/blobs/772b6900528a84ad0c76638ad91536865ab7d62883d05eb49a8f7841896c2740.txt, tests/fixtures/skill-releases/blobs/816e40ecd47f39ac62e8f0f5e84a9e6277e3470221df1d9c80fff9d6bccb378e.txt, tests/fixtures/skill-releases/blobs/86eb8769f03a556ce4e198b13a028821961c3f686cfbf639afda5651de696102.txt, tests/fixtures/skill-releases/blobs/8a50dd8a1ea1afadffaf8b56f86d0031ada23ecc1f72569df3c3ff24642448b4.txt, tests/fixtures/skill-releases/blobs/8b9b582018094745034c033327df0093787c23c1b092c8cd706863012f748421.txt, tests/fixtures/skill-releases/blobs/8ca2b70f36ee162638cafd63bcc97e8c9ed688b59c2b67b98d61ba3be1d28f37.txt, tests/fixtures/skill-releases/blobs/92a9e61abe6011473f526868e731dc003b81835db720f6f4d61d704d92c22d73.txt, tests/fixtures/skill-releases/blobs/970d5a96be20ac737f8e1c9650bc327355792b3ee4b2e525acbbd7dc14c7ad30.txt, tests/fixtures/skill-releases/blobs/a159d2c4846ca0441337be0716fdd7c7fd96da27e7aea0b0b6e3dfb3b0f9696f.txt, tests/fixtures/skill-releases/blobs/a44f6812276d05332b69bac0cff237485c6aefc1973cddd77226c0ff97c66e83.txt, tests/fixtures/skill-releases/blobs/ac787118e30b11a8b5eb1429108bf1ac42911767b1ff33c56d68e925c84f11a3.txt, tests/fixtures/skill-releases/blobs/b3edaa0e8a58e944ab9ba82455750f8d27018ae6ca9b4b2d738a013e13ef3eed.txt, tests/fixtures/skill-releases/blobs/b7371621899a8926447873b1f80db3f64cc51361544251099793a1e1f79760e3.txt, tests/fixtures/skill-releases/blobs/ba9e89ee55dd51f7ac477a52c27ce1d641272a1fabc9be3fce150fa6321f47b8.txt, tests/fixtures/skill-releases/blobs/bb5d91c18199599e10195d457da0157ebcbfcb9595568b3cad743c8fc1f00908.txt, tests/fixtures/skill-releases/blobs/be351d5c3a8c65bd46f4784875b0118f82e5ba4a5b97a7d364d9856636d3b711.txt, tests/fixtures/skill-releases/blobs/ce8f63741058cf79afa275e01fb7c6ab0c38dc242c35c87a8628c808bf386239.txt, tests/fixtures/skill-releases/blobs/d2a185514ebe6f7428ed2d31c946126e6201bfd1e0feec5e0dccd1250474aa9c.txt, tests/fixtures/skill-releases/blobs/df33dfa9aa3db1355b47ef47a653e75ff4ae42a8c1de096bcd1718f6a4287c17.txt, tests/fixtures/skill-releases/blobs/df42d472ae25b0ed07ad3beb1257ce77c3fe4e159feefe8e70e7c18ec03a6945.txt, tests/fixtures/skill-releases/blobs/e232af339e12254fcc735ffa87b72c36d0e80a8b3efc585cb7be627973d42235.txt, tests/fixtures/skill-releases/blobs/e61ad96fe5e2933c538d1ac775e7541a025d81fcd02804dc7c4d0fd215ed10f0.txt, tests/fixtures/skill-releases/blobs/e92e5a8fdc2e7b02ac6e01f34e15ee66501f7e3a312970069abb9e409aca167a.txt, tests/fixtures/skill-releases/blobs/ef831afa1094da91e5766c2d9597d6c933abe11b4d1a50913893415bd0d511a4.txt, tests/fixtures/skill-releases/blobs/fcbc759ab4605a6dd8fffd4af4498783186e308c6d771284bb9a2626f6e0b4e8.txt, specs/cli/cli.spec.md, specs/cli/requirements.md, specs/cli/tasks.md, specs/cli/testing.md, specs/cli/cli.3md, CHANGELOG.md, knowledge/INDEX.md, .chough/cases/cli.released-skills-102.cases.toml, .chough/sessions/issue-102-released-skills.json
- **Acceptance**: For the exact pinned v0.1.0, v0.2.0, v0.3.0, v0.4.0 and v0.5.0 source bundles, every one of 24 files reads current or known-outdated and skill update refreshes pristine copies; edited bytes and modes remain exact. Match file name plus exact SHA256 only; preserve missing/current handling, all host routing and legacy removal policy. Offline manifest120 mappings/51 exact blobs; no moving-network tests, future-release promise, blanket replacement, normalization or hash ignore. Six KNOWN_PREVIOUS pairs only; meaningful compiled RED then SAME GREEN requires separate lease; no implementation acceptance or delivery implied.

## Evidence

- Verification commit: `02a0a7052220c8afa21b72e0bc0c951938fd6f90`
- Base commit: `4bbd5a6373d26c0ad069878f8f543842a4564a6f`
- Verified by: `specsync check --spec cli`

## From the change's context.md

# Context

Issue #102 reports pristine 0.2.0 accept/plan/testing files treated as edited. Exact main4bbd and five published source tags establish six missing path/digest pairs. Discovery report CHOUGH102-DISCOVERY.md/immutable323-receipt manifest is retained externally. Trusted existing control ef3289 has exact current skill module and24 bundle bytes; no new main build. Actual statuses/updates yielded54 failed pristine-recognition assertions across6hosts and720 byte/mode-preserved edit controls. Source archive assets are data only, no bundled scripts executed. The original CLI commands exit0 while describing edited files; external semantic assertion exits1 with54 failures.

Authority: direct Leif all-PR/issues task relayed by orc-t19f; root grants this preparation only. No worker/human implementation acceptance, scope approval, review, archive, merge/release/install record is inferred. Old evaluation/113/196/187/132/60 trees stay immutable. New own session only; inherited typed snapshot/restoration proof retained. cli.3md does not exist at base: its future creation records only this module's lesson without replacing or migrating inherited context.md.

## Exact production pairs

- `templates/brief.md` SHA256 `fcbc759ab4605a6dd8fffd4af4498783186e308c6d771284bb9a2626f6e0b4e8`, pinned releases v0.1.0, v0.2.0, v0.3.0, v0.4.0.
- `references/accept.md` SHA256 `d2a185514ebe6f7428ed2d31c946126e6201bfd1e0feec5e0dccd1250474aa9c`, pinned releases v0.2.0.
- `references/plan.md` SHA256 `a159d2c4846ca0441337be0716fdd7c7fd96da27e7aea0b0b6e3dfb3b0f9696f`, pinned releases v0.2.0.
- `references/testing.md` SHA256 `ef831afa1094da91e5766c2d9597d6c933abe11b4d1a50913893415bd0d511a4`, pinned releases v0.2.0.
- `references/accept.md` SHA256 `18299d9d1adb30fb87a167a1a59ecd8595d1f53ec73522ccee257ea817641701`, pinned releases v0.5.0.
- `references/lifecycle.md` SHA256 `ac787118e30b11a8b5eb1429108bf1ac42911767b1ff33c56d68e925c84f11a3`, pinned releases v0.5.0.

## Exact affected paths

- `src/skill.rs`
- `tests/skill_releases.rs`
- `tests/fixtures/skill-releases/manifest.json`
- `tests/fixtures/skill-releases/blobs/00b3c3f4ffaaeeaaca6d829ff0277c9eb354ce0911f66f41a55639d27fc7d35d.txt`
- `tests/fixtures/skill-releases/blobs/1022db57d0c114effc8669ede9ebb453f932109f608f883712725e57ba53551d.txt`
- `tests/fixtures/skill-releases/blobs/10cd3cbe76a8a56e6e931bed15a42d4ec1a10ccc8a9091fac3f60ca507856069.txt`
- `tests/fixtures/skill-releases/blobs/1492e0c5368df8762f09b2bb590a049f6dc88649c6003686951dddf6fe34b2bf.txt`
- `tests/fixtures/skill-releases/blobs/18299d9d1adb30fb87a167a1a59ecd8595d1f53ec73522ccee257ea817641701.txt`
- `tests/fixtures/skill-releases/blobs/187a887f2999e2d54350a56059a739f8379a7d77305ab53652da3b3eed8caa8d.txt`
- `tests/fixtures/skill-releases/blobs/1a0a324a83658a1466ee6fe295ccfe9e980275f7abca809ef2927e71fb912663.txt`
- `tests/fixtures/skill-releases/blobs/1c9ef93185390ee98aa4b4f2f9fbf2c442a003ad22097d0e74e4676b42404909.txt`
- `tests/fixtures/skill-releases/blobs/23dcd8140f0d5bbbf77ea9a20ce37c43fde91345bf1cddf40b9336797a920a57.txt`
- `tests/fixtures/skill-releases/blobs/28eccd903a89ff815e71e0cc3f81250e6c63bb986d93b0ee76b323180c4d92bf.txt`
- `tests/fixtures/skill-releases/blobs/2b1aabe936ec051c040820625b7665b5d26706fa5e9396ddb4bfba8425ede78a.txt`
- `tests/fixtures/skill-releases/blobs/2e26c47b349c4138d50eb78aa557822909c586802a9dbb12a46a466438d89b3b.txt`
- `tests/fixtures/skill-releases/blobs/366259957ea1289ac4c9046a4c65b42cb4a7228ab93006ca9396bc5840d612e3.txt`
- `tests/fixtures/skill-releases/blobs/367a9873293e79ca40c9634e2b2a12c3223074aaec161c7444690c2d02e12199.txt`
- `tests/fixtures/skill-releases/blobs/370386488c8e5ff6c974ae9d3bd51998de31183868dfe48af44c0935a3019170.txt`
- `tests/fixtures/skill-releases/blobs/3da19999ee4830a1fe1e0af63b13efc934f90d05678bbf70c3d79f39103c1150.txt`
- `tests/fixtures/skill-releases/blobs/4013dc5ac116f3f32cffeb89ffcef957c3b402236000ea54c130ae41a02a96b5.txt`
- `tests/fixtures/skill-releases/blobs/472396403a4f716f0183ec2c3495ac0ecf0dc412c1b61bb850cee767765e1f6a.txt`
- `tests/fixtures/skill-releases/blobs/4995658517849e80e567a73eaf0399ec51123549a7e94649a17afa35ec40f20d.txt`
- `tests/fixtures/skill-releases/blobs/4d0bf24ddd5e55493658339c76750a639a15997c4290a492cacda3550874f035.txt`
- `tests/fixtures/skill-releases/blobs/4e085b88eaa66a59fd6c8ce7d3acb1201b46292603a2cf13ab5333fc5ea85d86.txt`
- `tests/fixtures/skill-releases/blobs/51281ff6c9af4b939105cdf6ecf80d9dd11080dc8d682ecbba9d5b5c17d75c42.txt`
- `tests/fixtures/skill-releases/blobs/5d7f85aa72b3bba8e5d83b593556bcdee244939b31038ed3e6000783aac5c4ea.txt`
- `tests/fixtures/skill-releases/blobs/5f839a81742b9dc3df3b1278e340061a08e95e2a00e203f0f2554eab3aa3fed9.txt`
- `tests/fixtures/skill-releases/blobs/658ce798826c4d685753af0735e5fc6a53685f1205b9b9902d625c63c71d277f.txt`
- `tests/fixtures/skill-releases/blobs/733346e29c7a57461926a6c7d297face74fc291a2e5c7f17d4d57dcf3945ec99.txt`
- `tests/fixtures/skill-releases/blobs/772b6900528a84ad0c76638ad91536865ab7d62883d05eb49a8f7841896c2740.txt`
- `tests/fixtures/skill-releases/blobs/816e40ecd47f39ac62e8f0f5e84a9e6277e3470221df1d9c80fff9d6bccb378e.txt`
- `tests/fixtures/skill-releases/blobs/86eb8769f03a556ce4e198b13a028821961c3f686cfbf639afda5651de696102.txt`
- `tests/fixtures/skill-releases/blobs/8a50dd8a1ea1afadffaf8b56f86d0031ada23ecc1f72569df3c3ff24642448b4.txt`
- `tests/fixtures/skill-releases/blobs/8b9b582018094745034c033327df0093787c23c1b092c8cd706863012f748421.txt`
- `tests/fixtures/skill-releases/blobs/8ca2b70f36ee162638cafd63bcc97e8c9ed688b59c2b67b98d61ba3be1d28f37.txt`
- `tests/fixtures/skill-releases/blobs/92a9e61abe6011473f526868e731dc003b81835db720f6f4d61d704d92c22d73.txt`
- `tests/fixtures/skill-releases/blobs/970d5a96be20ac737f8e1c9650bc327355792b3ee4b2e525acbbd7dc14c7ad30.txt`
- `tests/fixtures/skill-releases/blobs/a159d2c4846ca0441337be0716fdd7c7fd96da27e7aea0b0b6e3dfb3b0f9696f.txt`
- `tests/fixtures/skill-releases/blobs/a44f6812276d05332b69bac0cff237485c6aefc1973cddd77226c0ff97c66e83.txt`
- `tests/fixtures/skill-releases/blobs/ac787118e30b11a8b5eb1429108bf1ac42911767b1ff33c56d68e925c84f11a3.txt`
- `tests/fixtures/skill-releases/blobs/b3edaa0e8a58e944ab9ba82455750f8d27018ae6ca9b4b2d738a013e13ef3eed.txt`
- `tests/fixtures/skill-releases/blobs/b7371621899a8926447873b1f80db3f64cc51361544251099793a1e1f79760e3.txt`
- `tests/fixtures/skill-releases/blobs/ba9e89ee55dd51f7ac477a52c27ce1d641272a1fabc9be3fce150fa6321f47b8.txt`
- `tests/fixtures/skill-releases/blobs/bb5d91c18199599e10195d457da0157ebcbfcb9595568b3cad743c8fc1f00908.txt`
- `tests/fixtures/skill-releases/blobs/be351d5c3a8c65bd46f4784875b0118f82e5ba4a5b97a7d364d9856636d3b711.txt`
- `tests/fixtures/skill-releases/blobs/ce8f63741058cf79afa275e01fb7c6ab0c38dc242c35c87a8628c808bf386239.txt`
- `tests/fixtures/skill-releases/blobs/d2a185514ebe6f7428ed2d31c946126e6201bfd1e0feec5e0dccd1250474aa9c.txt`
- `tests/fixtures/skill-releases/blobs/df33dfa9aa3db1355b47ef47a653e75ff4ae42a8c1de096bcd1718f6a4287c17.txt`
- `tests/fixtures/skill-releases/blobs/df42d472ae25b0ed07ad3beb1257ce77c3fe4e159feefe8e70e7c18ec03a6945.txt`
- `tests/fixtures/skill-releases/blobs/e232af339e12254fcc735ffa87b72c36d0e80a8b3efc585cb7be627973d42235.txt`
- `tests/fixtures/skill-releases/blobs/e61ad96fe5e2933c538d1ac775e7541a025d81fcd02804dc7c4d0fd215ed10f0.txt`
- `tests/fixtures/skill-releases/blobs/e92e5a8fdc2e7b02ac6e01f34e15ee66501f7e3a312970069abb9e409aca167a.txt`
- `tests/fixtures/skill-releases/blobs/ef831afa1094da91e5766c2d9597d6c933abe11b4d1a50913893415bd0d511a4.txt`
- `tests/fixtures/skill-releases/blobs/fcbc759ab4605a6dd8fffd4af4498783186e308c6d771284bb9a2626f6e0b4e8.txt`
- `specs/cli/cli.spec.md`
- `specs/cli/requirements.md`
- `specs/cli/tasks.md`
- `specs/cli/testing.md`
- `specs/cli/cli.3md`
- `CHANGELOG.md`
- `knowledge/INDEX.md`
- `.chough/cases/cli.released-skills-102.cases.toml`
- `.chough/sessions/issue-102-released-skills.json`

## Pinned source releases

- v0.1.0 commit `c03d41956ef1d9f8380f02034f03d42bf52bc174` archive SHA256 `1859f69abd8762826e5012cb4f85920f502ee383b45e78c0458f50a9a3bf48e9`.
- v0.2.0 commit `edbc3c1adecebf6a1378a731965c3bfe8c8217df` archive SHA256 `7120fd59d75d37e8e7f08874a0d38d323b08dfd2708fdade6e962bc476c1aadd`.
- v0.3.0 commit `4d65542ff583ee2478f7d98c1dd4434a960e9dc1` archive SHA256 `fd653290aef505e2fcdad9135581076e2a8d56d91d10237ef9d117661e55bca0`.
- v0.4.0 commit `69950c7a9253230aeeee3bac0d0d6b580e291aad` archive SHA256 `bbbea74e358a1ffd113932b6121831bf7576b1b5fc7108caca2b927b8b82d36c`.
- v0.5.0 commit `455cb643cd91a9c804afdf5566e3889abf1bde49` archive SHA256 `e3b211832d6f978248591d9854cf6f83e43d7d4b2ff3c00d648e91693cf16719`.

## Complete immutable fixture ownership map

Manifest `tests/fixtures/skill-releases/manifest.json` contains all120 release/file mappings. The following51 exact deduplicated files are future-owned; none is created during preparation. No network lookup occurs in product tests.

| Future path | Exact SHA256 | Bytes | Release/file sources |
|---|---|---|---|
| `tests/fixtures/skill-releases/blobs/00b3c3f4ffaaeeaaca6d829ff0277c9eb354ce0911f66f41a55639d27fc7d35d.txt` | `00b3c3f4ffaaeeaaca6d829ff0277c9eb354ce0911f66f41a55639d27fc7d35d` | 9452 | v0.2.0:references/checkpoint.md; v0.3.0:references/checkpoint.md |
| `tests/fixtures/skill-releases/blobs/1022db57d0c114effc8669ede9ebb453f932109f608f883712725e57ba53551d.txt` | `1022db57d0c114effc8669ede9ebb453f932109f608f883712725e57ba53551d` | 2365 | v0.1.0:references/plan.md |
| `tests/fixtures/skill-releases/blobs/10cd3cbe76a8a56e6e931bed15a42d4ec1a10ccc8a9091fac3f60ca507856069.txt` | `10cd3cbe76a8a56e6e931bed15a42d4ec1a10ccc8a9091fac3f60ca507856069` | 2424 | v0.1.0:references/intent.md; v0.2.0:references/intent.md; v0.3.0:references/intent.md; v0.4.0:references/intent.md; v0.5.0:references/intent.md |
| `tests/fixtures/skill-releases/blobs/1492e0c5368df8762f09b2bb590a049f6dc88649c6003686951dddf6fe34b2bf.txt` | `1492e0c5368df8762f09b2bb590a049f6dc88649c6003686951dddf6fe34b2bf` | 760 | v0.1.0:templates/frd.md; v0.2.0:templates/frd.md; v0.3.0:templates/frd.md; v0.4.0:templates/frd.md; v0.5.0:templates/frd.md |
| `tests/fixtures/skill-releases/blobs/18299d9d1adb30fb87a167a1a59ecd8595d1f53ec73522ccee257ea817641701.txt` | `18299d9d1adb30fb87a167a1a59ecd8595d1f53ec73522ccee257ea817641701` | 5534 | v0.5.0:references/accept.md |
| `tests/fixtures/skill-releases/blobs/187a887f2999e2d54350a56059a739f8379a7d77305ab53652da3b3eed8caa8d.txt` | `187a887f2999e2d54350a56059a739f8379a7d77305ab53652da3b3eed8caa8d` | 3755 | v0.5.0:references/plan.md |
| `tests/fixtures/skill-releases/blobs/1a0a324a83658a1466ee6fe295ccfe9e980275f7abca809ef2927e71fb912663.txt` | `1a0a324a83658a1466ee6fe295ccfe9e980275f7abca809ef2927e71fb912663` | 3354 | v0.3.0:references/plan.md; v0.4.0:references/plan.md |
| `tests/fixtures/skill-releases/blobs/1c9ef93185390ee98aa4b4f2f9fbf2c442a003ad22097d0e74e4676b42404909.txt` | `1c9ef93185390ee98aa4b4f2f9fbf2c442a003ad22097d0e74e4676b42404909` | 7235 | v0.3.0:references/lifecycle.md; v0.4.0:references/lifecycle.md |
| `tests/fixtures/skill-releases/blobs/23dcd8140f0d5bbbf77ea9a20ce37c43fde91345bf1cddf40b9336797a920a57.txt` | `23dcd8140f0d5bbbf77ea9a20ce37c43fde91345bf1cddf40b9336797a920a57` | 6134 | v0.5.0:SKILL.md |
| `tests/fixtures/skill-releases/blobs/28eccd903a89ff815e71e0cc3f81250e6c63bb986d93b0ee76b323180c4d92bf.txt` | `28eccd903a89ff815e71e0cc3f81250e6c63bb986d93b0ee76b323180c4d92bf` | 3967 | v0.1.0:references/checkpoint.md |
| `tests/fixtures/skill-releases/blobs/2b1aabe936ec051c040820625b7665b5d26706fa5e9396ddb4bfba8425ede78a.txt` | `2b1aabe936ec051c040820625b7665b5d26706fa5e9396ddb4bfba8425ede78a` | 1094 | v0.1.0:templates/bug-report.md; v0.2.0:templates/bug-report.md; v0.3.0:templates/bug-report.md; v0.4.0:templates/bug-report.md; v0.5.0:templates/bug-report.md |
| `tests/fixtures/skill-releases/blobs/2e26c47b349c4138d50eb78aa557822909c586802a9dbb12a46a466438d89b3b.txt` | `2e26c47b349c4138d50eb78aa557822909c586802a9dbb12a46a466438d89b3b` | 8348 | v0.3.0:references/testing.md; v0.4.0:references/testing.md; v0.5.0:references/testing.md |
| `tests/fixtures/skill-releases/blobs/366259957ea1289ac4c9046a4c65b42cb4a7228ab93006ca9396bc5840d612e3.txt` | `366259957ea1289ac4c9046a4c65b42cb4a7228ab93006ca9396bc5840d612e3` | 2060 | v0.1.0:templates/prototype/index.html; v0.2.0:templates/prototype/index.html; v0.3.0:templates/prototype/index.html; v0.4.0:templates/prototype/index.html; v0.5.0:templates/prototype/index.html |
| `tests/fixtures/skill-releases/blobs/367a9873293e79ca40c9634e2b2a12c3223074aaec161c7444690c2d02e12199.txt` | `367a9873293e79ca40c9634e2b2a12c3223074aaec161c7444690c2d02e12199` | 4874 | v0.1.0:SKILL.md |
| `tests/fixtures/skill-releases/blobs/370386488c8e5ff6c974ae9d3bd51998de31183868dfe48af44c0935a3019170.txt` | `370386488c8e5ff6c974ae9d3bd51998de31183868dfe48af44c0935a3019170` | 10335 | v0.4.0:references/checkpoint.md |
| `tests/fixtures/skill-releases/blobs/3da19999ee4830a1fe1e0af63b13efc934f90d05678bbf70c3d79f39103c1150.txt` | `3da19999ee4830a1fe1e0af63b13efc934f90d05678bbf70c3d79f39103c1150` | 4860 | v0.4.0:references/accept.md |
| `tests/fixtures/skill-releases/blobs/4013dc5ac116f3f32cffeb89ffcef957c3b402236000ea54c130ae41a02a96b5.txt` | `4013dc5ac116f3f32cffeb89ffcef957c3b402236000ea54c130ae41a02a96b5` | 3009 | v0.2.0:references/validate.md; v0.3.0:references/validate.md; v0.4.0:references/validate.md; v0.5.0:references/validate.md |
| `tests/fixtures/skill-releases/blobs/472396403a4f716f0183ec2c3495ac0ecf0dc412c1b61bb850cee767765e1f6a.txt` | `472396403a4f716f0183ec2c3495ac0ecf0dc412c1b61bb850cee767765e1f6a` | 4722 | v0.3.0:references/accept.md |
| `tests/fixtures/skill-releases/blobs/4995658517849e80e567a73eaf0399ec51123549a7e94649a17afa35ec40f20d.txt` | `4995658517849e80e567a73eaf0399ec51123549a7e94649a17afa35ec40f20d` | 1690 | v0.1.0:references/roles.md; v0.2.0:references/roles.md; v0.3.0:references/roles.md; v0.4.0:references/roles.md; v0.5.0:references/roles.md |
| `tests/fixtures/skill-releases/blobs/4d0bf24ddd5e55493658339c76750a639a15997c4290a492cacda3550874f035.txt` | `4d0bf24ddd5e55493658339c76750a639a15997c4290a492cacda3550874f035` | 2198 | v0.2.0:references/build.md |
| `tests/fixtures/skill-releases/blobs/4e085b88eaa66a59fd6c8ce7d3acb1201b46292603a2cf13ab5333fc5ea85d86.txt` | `4e085b88eaa66a59fd6c8ce7d3acb1201b46292603a2cf13ab5333fc5ea85d86` | 3059 | v0.1.0:references/accept.md |
| `tests/fixtures/skill-releases/blobs/51281ff6c9af4b939105cdf6ecf80d9dd11080dc8d682ecbba9d5b5c17d75c42.txt` | `51281ff6c9af4b939105cdf6ecf80d9dd11080dc8d682ecbba9d5b5c17d75c42` | 2511 | v0.1.0:references/validate.md |
| `tests/fixtures/skill-releases/blobs/5d7f85aa72b3bba8e5d83b593556bcdee244939b31038ed3e6000783aac5c4ea.txt` | `5d7f85aa72b3bba8e5d83b593556bcdee244939b31038ed3e6000783aac5c4ea` | 6136 | v0.2.0:SKILL.md |
| `tests/fixtures/skill-releases/blobs/5f839a81742b9dc3df3b1278e340061a08e95e2a00e203f0f2554eab3aa3fed9.txt` | `5f839a81742b9dc3df3b1278e340061a08e95e2a00e203f0f2554eab3aa3fed9` | 2251 | v0.1.0:templates/cases.toml |
| `tests/fixtures/skill-releases/blobs/658ce798826c4d685753af0735e5fc6a53685f1205b9b9902d625c63c71d277f.txt` | `658ce798826c4d685753af0735e5fc6a53685f1205b9b9902d625c63c71d277f` | 7221 | v0.1.0:references/stages.md; v0.2.0:references/stages.md; v0.3.0:references/stages.md; v0.4.0:references/stages.md |
| `tests/fixtures/skill-releases/blobs/733346e29c7a57461926a6c7d297face74fc291a2e5c7f17d4d57dcf3945ec99.txt` | `733346e29c7a57461926a6c7d297face74fc291a2e5c7f17d4d57dcf3945ec99` | 6141 | v0.3.0:SKILL.md; v0.4.0:SKILL.md |
| `tests/fixtures/skill-releases/blobs/772b6900528a84ad0c76638ad91536865ab7d62883d05eb49a8f7841896c2740.txt` | `772b6900528a84ad0c76638ad91536865ab7d62883d05eb49a8f7841896c2740` | 3385 | v0.2.0:references/librarian.md; v0.3.0:references/librarian.md |
| `tests/fixtures/skill-releases/blobs/816e40ecd47f39ac62e8f0f5e84a9e6277e3470221df1d9c80fff9d6bccb378e.txt` | `816e40ecd47f39ac62e8f0f5e84a9e6277e3470221df1d9c80fff9d6bccb378e` | 617 | v0.5.0:templates/brief.md |
| `tests/fixtures/skill-releases/blobs/86eb8769f03a556ce4e198b13a028821961c3f686cfbf639afda5651de696102.txt` | `86eb8769f03a556ce4e198b13a028821961c3f686cfbf639afda5651de696102` | 3968 | v0.2.0:references/bugs.md; v0.3.0:references/bugs.md; v0.4.0:references/bugs.md; v0.5.0:references/bugs.md |
| `tests/fixtures/skill-releases/blobs/8a50dd8a1ea1afadffaf8b56f86d0031ada23ecc1f72569df3c3ff24642448b4.txt` | `8a50dd8a1ea1afadffaf8b56f86d0031ada23ecc1f72569df3c3ff24642448b4` | 681 | v0.1.0:templates/review-findings.md; v0.2.0:templates/review-findings.md; v0.3.0:templates/review-findings.md; v0.4.0:templates/review-findings.md; v0.5.0:templates/review-findings.md |
| `tests/fixtures/skill-releases/blobs/8b9b582018094745034c033327df0093787c23c1b092c8cd706863012f748421.txt` | `8b9b582018094745034c033327df0093787c23c1b092c8cd706863012f748421` | 2646 | v0.2.0:templates/cases.toml; v0.3.0:templates/cases.toml; v0.4.0:templates/cases.toml; v0.5.0:templates/cases.toml |
| `tests/fixtures/skill-releases/blobs/8ca2b70f36ee162638cafd63bcc97e8c9ed688b59c2b67b98d61ba3be1d28f37.txt` | `8ca2b70f36ee162638cafd63bcc97e8c9ed688b59c2b67b98d61ba3be1d28f37` | 5659 | v0.1.0:references/testing.md |
| `tests/fixtures/skill-releases/blobs/92a9e61abe6011473f526868e731dc003b81835db720f6f4d61d704d92c22d73.txt` | `92a9e61abe6011473f526868e731dc003b81835db720f6f4d61d704d92c22d73` | 7439 | v0.5.0:references/stages.md |
| `tests/fixtures/skill-releases/blobs/970d5a96be20ac737f8e1c9650bc327355792b3ee4b2e525acbbd7dc14c7ad30.txt` | `970d5a96be20ac737f8e1c9650bc327355792b3ee4b2e525acbbd7dc14c7ad30` | 2477 | v0.3.0:references/build.md; v0.4.0:references/build.md; v0.5.0:references/build.md |
| `tests/fixtures/skill-releases/blobs/a159d2c4846ca0441337be0716fdd7c7fd96da27e7aea0b0b6e3dfb3b0f9696f.txt` | `a159d2c4846ca0441337be0716fdd7c7fd96da27e7aea0b0b6e3dfb3b0f9696f` | 3315 | v0.2.0:references/plan.md |
| `tests/fixtures/skill-releases/blobs/a44f6812276d05332b69bac0cff237485c6aefc1973cddd77226c0ff97c66e83.txt` | `a44f6812276d05332b69bac0cff237485c6aefc1973cddd77226c0ff97c66e83` | 788 | v0.1.0:templates/prototype/COMPANION.md; v0.2.0:templates/prototype/COMPANION.md; v0.3.0:templates/prototype/COMPANION.md; v0.4.0:templates/prototype/COMPANION.md; v0.5.0:templates/prototype/COMPANION.md |
| `tests/fixtures/skill-releases/blobs/ac787118e30b11a8b5eb1429108bf1ac42911767b1ff33c56d68e925c84f11a3.txt` | `ac787118e30b11a8b5eb1429108bf1ac42911767b1ff33c56d68e925c84f11a3` | 8025 | v0.5.0:references/lifecycle.md |
| `tests/fixtures/skill-releases/blobs/b3edaa0e8a58e944ab9ba82455750f8d27018ae6ca9b4b2d738a013e13ef3eed.txt` | `b3edaa0e8a58e944ab9ba82455750f8d27018ae6ca9b4b2d738a013e13ef3eed` | 1375 | v0.1.0:references/release.md; v0.2.0:references/release.md; v0.3.0:references/release.md; v0.4.0:references/release.md; v0.5.0:references/release.md |
| `tests/fixtures/skill-releases/blobs/b7371621899a8926447873b1f80db3f64cc51361544251099793a1e1f79760e3.txt` | `b7371621899a8926447873b1f80db3f64cc51361544251099793a1e1f79760e3` | 11839 | v0.5.0:references/checkpoint.md |
| `tests/fixtures/skill-releases/blobs/ba9e89ee55dd51f7ac477a52c27ce1d641272a1fabc9be3fce150fa6321f47b8.txt` | `ba9e89ee55dd51f7ac477a52c27ce1d641272a1fabc9be3fce150fa6321f47b8` | 5280 | v0.4.0:references/librarian.md; v0.5.0:references/librarian.md |
| `tests/fixtures/skill-releases/blobs/bb5d91c18199599e10195d457da0157ebcbfcb9595568b3cad743c8fc1f00908.txt` | `bb5d91c18199599e10195d457da0157ebcbfcb9595568b3cad743c8fc1f00908` | 2114 | v0.1.0:references/design.md; v0.2.0:references/design.md; v0.3.0:references/design.md; v0.4.0:references/design.md; v0.5.0:references/design.md |
| `tests/fixtures/skill-releases/blobs/be351d5c3a8c65bd46f4784875b0118f82e5ba4a5b97a7d364d9856636d3b711.txt` | `be351d5c3a8c65bd46f4784875b0118f82e5ba4a5b97a7d364d9856636d3b711` | 6688 | v0.2.0:references/lifecycle.md |
| `tests/fixtures/skill-releases/blobs/ce8f63741058cf79afa275e01fb7c6ab0c38dc242c35c87a8628c808bf386239.txt` | `ce8f63741058cf79afa275e01fb7c6ab0c38dc242c35c87a8628c808bf386239` | 1356 | v0.1.0:templates/prototype/styles.css; v0.2.0:templates/prototype/styles.css; v0.3.0:templates/prototype/styles.css; v0.4.0:templates/prototype/styles.css; v0.5.0:templates/prototype/styles.css |
| `tests/fixtures/skill-releases/blobs/d2a185514ebe6f7428ed2d31c946126e6201bfd1e0feec5e0dccd1250474aa9c.txt` | `d2a185514ebe6f7428ed2d31c946126e6201bfd1e0feec5e0dccd1250474aa9c` | 4628 | v0.2.0:references/accept.md |
| `tests/fixtures/skill-releases/blobs/df33dfa9aa3db1355b47ef47a653e75ff4ae42a8c1de096bcd1718f6a4287c17.txt` | `df33dfa9aa3db1355b47ef47a653e75ff4ae42a8c1de096bcd1718f6a4287c17` | 1576 | v0.1.0:references/librarian.md |
| `tests/fixtures/skill-releases/blobs/df42d472ae25b0ed07ad3beb1257ce77c3fe4e159feefe8e70e7c18ec03a6945.txt` | `df42d472ae25b0ed07ad3beb1257ce77c3fe4e159feefe8e70e7c18ec03a6945` | 3739 | v0.1.0:references/bugs.md |
| `tests/fixtures/skill-releases/blobs/e232af339e12254fcc735ffa87b72c36d0e80a8b3efc585cb7be627973d42235.txt` | `e232af339e12254fcc735ffa87b72c36d0e80a8b3efc585cb7be627973d42235` | 4950 | v0.1.0:references/lifecycle.md |
| `tests/fixtures/skill-releases/blobs/e61ad96fe5e2933c538d1ac775e7541a025d81fcd02804dc7c4d0fd215ed10f0.txt` | `e61ad96fe5e2933c538d1ac775e7541a025d81fcd02804dc7c4d0fd215ed10f0` | 334 | v0.1.0:agents/openai.yaml; v0.2.0:agents/openai.yaml; v0.3.0:agents/openai.yaml; v0.4.0:agents/openai.yaml; v0.5.0:agents/openai.yaml |
| `tests/fixtures/skill-releases/blobs/e92e5a8fdc2e7b02ac6e01f34e15ee66501f7e3a312970069abb9e409aca167a.txt` | `e92e5a8fdc2e7b02ac6e01f34e15ee66501f7e3a312970069abb9e409aca167a` | 1483 | v0.1.0:references/build.md |
| `tests/fixtures/skill-releases/blobs/ef831afa1094da91e5766c2d9597d6c933abe11b4d1a50913893415bd0d511a4.txt` | `ef831afa1094da91e5766c2d9597d6c933abe11b4d1a50913893415bd0d511a4` | 7168 | v0.2.0:references/testing.md |
| `tests/fixtures/skill-releases/blobs/fcbc759ab4605a6dd8fffd4af4498783186e308c6d771284bb9a2626f6e0b4e8.txt` | `fcbc759ab4605a6dd8fffd4af4498783186e308c6d771284bb9a2626f6e0b4e8` | 619 | v0.1.0:templates/brief.md; v0.2.0:templates/brief.md; v0.3.0:templates/brief.md; v0.4.0:templates/brief.md |

## From the change's design.md

# Design

A data-only predecessor catalog correction, not a UI/architecture redesign. file_state continues to compare current bytes first, then an exact (filename,SHA256) predecessor pair; skill update adds missing/current-pristine files and preserves every edited file. Only six catalog rows are added. Standalone tests consume fixed local bytes and cannot execute downloaded scripts or network lookups. Manifest binds5tag commits,24files each and51 exact SHA-named blobs. All production methods, data schemas, host routing and legacy policy stay unchanged.

The complete implementation plan below is deliberately included in this selected artifact so its scope is digest-bound even though the native interview did not select standalone plan.md after architecture_risk=no. Standalone plan.md remains identical supporting preparation, not an inferred completed gate.


# Plan

## Approach

Treat published source bytes as exact predecessor data. Add only the six proven missing filename/SHA256 pairs to KNOWN_PREVIOUS; never change file_state/update/host/legacy policy. Commit the offline five-release manifest and51 deduplicated source-byte blobs with a new standalone runtime test. REQ-cli-016 keeps all existing wording and gains explicit pinned-release coverage/edit-preservation acceptance criteria. No new IDs or public command shapes.

## Out of scope

Future/moving release recognition, blanket overwrite, hash ignore/normalization, byte-newline conversion, replacing edited bytes/modes, changing host routes or legacy removal, init/adopt/migration, other production/test files, inherited lifecycle records, network-executed fixtures, actor/approval policy, release/install/hosted actions.

## Steps

1. Freeze complete preparation/digest and hand sole metadata writer to root for actual scope record.
2. After root receipt and a separate focused build lease, commit exact offline fixtures and discriminating standalone tests; execute real compiled RED on unchanged production. Keep setup/compile failures separate.
3. Add only six KNOWN_PREVIOUS rows; SAME tests GREEN. Exercise all pinned files and edited/mode/name-digest/current/missing/host/legacy boundaries; run existing focused controls/fmt/lint only as leased.
4. Finish actual owned case/evidence/local bug/companion/one-change-log/index records; ordinary own native strict materialization/current freeze, then other-author review/human gates remain separate. No inherited native refresh by this owner.

## Risks

Incomplete predecessor catalogs silently preserve stale pristine content; overly broad matching overwrites real edits. Assert exact path+digest and offline immutable corpus identity. Six-host fixtures are convention/routing checks, not proof that each host discovers the skill. Existing frozen runtime relevant-input equivalence is not a fresh full-main build. New lesson file must preserve old context.md and all inherited sessions; no automatic archive drift may survive.

## Constraints consulted

Actual AGENTS.md and workflow skill; cli contract/REQ-cli-016 and companions; HI FLOW-1/LIFE-4/LIFE-12..14; root external preparation allocation; source archives/tag commits and323-receipt discovery packet; current main4bbd; own inherited typed snapshot. Fledge first, Rust/HI/Fledge/SpecSync pins follow CI only after lease. Other agents own other branches and all original seals remain unchanged.

## From the change's testing.md

# Testing

## Requirement evidence

| Requirement | Planned check | Evidence boundary |
|---|---|---|
| REQ-cli-016 | New tests/skill_releases.rs; offline5tags×24files, all6host routes; byte/mode-edited and wrong-filename digest controls; current/missing/legacy original controls | Future compiled RED/SAME GREEN required; existing private skill control has54 failed pristine rows and720 preserved edited rows, not implementation proof |

## Runtime regression plan

Load only the pinned120-mapping manifest and51 exact byte blobs, verify SHA/length/tag commit identity before invoking the tested binary in private roots. Assert every pristine released file is current/known-outdated, then byte-equals the embedded current copy after update. Single-byte/appended-text/CRLF mutations and known bytes at the wrong filename remain edited and preserve bytes/modes; no digest-only or newline normalization loophole. Preserve shared Claude/OpenAI and otherhost routing, current/missing behavior and explicit legacy handling. Original focused skill controls remain unchanged.

## Commands and limits

Future leased Fledge run test -- --locked --test skill_releases, followed by existing focused skill tests and declared fmt/lint. Not run during preparation. Fledge/native structural checks are not product assertions. No moving-network test, script execution, global install, fresh host discovery, Windows, full/Trust/site/hosted acceptance claim. All failures/raw identities retained.

## Retained actual execution results

t2ns at6bb513b: seven ran,5pass/2fail,54 pristine failures in each intended recognition/update test. t2p7 atd9a9d924: SAME7/7, existing optional4/4, cli3/3, package16/16, then fmt0/lint0. Actual final compiler end2026-11-09T05:05:12.506196Z; lease released before bookkeeping. Complete source/input equality established. No new test or case run occurs while recording these retained receipts. Original setup probe failure remains separate from product RED; normal subprocess restore_signals controls verified. Full/Trust/site/hosted/human gates remain unrun.

## Where these lessons go

- `specs/cli/cli.3md`
