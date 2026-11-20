---
kind: receipt
id: 2026-11-20-from-0-5
plan_digest: 1f80b80aae0935ca531b11632394ccb9079f6cc12fa7efff15d32af2c478fcc8
applied: 2026-11-20T02:19:49Z
consent: "--consent (a relayed claim): SYNTHETIC-human said migrate it (fixture)"
removed_at_commit: 241497c3a7f0339777e932852a70729efa4e81b6
---
# Migration from Chough 0.5.x

Git history still holds what earlier commits recorded; this migration does not rewrite it.

## Before

- `.chough/history/cycles/CYC-35b47ded993bd490850611ba/EVT-4edf61033f58689200e93314.json` f1bc75e5e8d6ada4f9d9e100c15e419fbdef51809bb5ab25ac1a0fb7f5ceee1c
- `.chough/history/cycles/CYC-42680cc9b00fa16e09f7b1a5/EVT-0cdfd4d090ee4a3389ad309f.json` acf090d9b33ce7632d06143d384555994b1cd442ed07273403aa8ae4a2d0aa85
- `.chough/history/cycles/CYC-42680cc9b00fa16e09f7b1a5/EVT-da5fff4240599edcb3d6502e.json` e6545f4ab98ca946dd0741a619cc80330a2ccc38f9988d0c57d0e8ed373b1052
- `.chough/history/cycles/CYC-5a72dc37cbbdb146702c0e2a/EVT-ce87b276f18c71b7feab3cfb.json` eaace03f5d8643d87968b94e629e85244c44b917a10cb91b246b0c62ef54e406
- `.chough/history/cycles/CYC-621399035e1dfe1261b5ce4a/EVT-188a807240ab4176f0ec460e.json` f7cab9d85cc08f05a7f23dd6b9a0ad88e43d1ca5499afd46172150608bce63cd
- `.chough/history/cycles/CYC-621399035e1dfe1261b5ce4a/EVT-281a5600cf62a0bc2dc15ca6.json` 199eacf317750c6e9e45dd88469d64f72f0aa83d4242dc2d24cfc4d7a488fc63
- `.chough/history/cycles/CYC-621399035e1dfe1261b5ce4a/EVT-462d4ec830878b10804dfb40.json` e59f48dc4f2780bf62f0a6895e8905378ad10f50888e3159908c9c4dbfe20e4a
- `.chough/history/cycles/CYC-621399035e1dfe1261b5ce4a/EVT-72523ee7b9ed7ef5bf1f5cfc.json` a7db0f87ddcbdee4c49ad531e9aa1732208fb442849a8fec00752aa103877cd0
- `.chough/history/cycles/CYC-621399035e1dfe1261b5ce4a/EVT-f14410095b1d4e3f9b1117cc.json` 021d55e6084c57ec483f52c5a953f00b2eb3803b87470c977fe217edfec1f224
- `.chough/history/cycles/CYC-621399035e1dfe1261b5ce4a/EVT-fea1c6bea657eb5fc5925a67.json` d470618154d786978fe25ceb2e31f6b77af2cc738ab2daaecfb2295d96432048
- `.chough/history/cycles/CYC-6aa09e1da93e854627beb733/EVT-908e95dc0772292bb0c59567.json` 9e4c32345aaadfa99a0b3421d5b88a45c179e141d0abbf003eeb28928349a198
- `.chough/history/cycles/CYC-9fa9e31652575eb199c5ad11/EVT-f2b9b6b9907720e91cd4e0fe.json` afa8598a3f5d5db3bef63fa8d15521cdcf3d65d132147b9ac148c6e0c91e4ab2
- `.chough/history/cycles/CYC-a8df9ab1c24719335a8a4b34/EVT-4281177d85d90b750a4f130f.json` 9f284357df3878e3991b86f04e8c7e406328d8f0b24e3e96395a46456dc96a02
- `.chough/history/cycles/CYC-bb5c089179aed3227e15b0fd/EVT-134ea977c7a62e1800b8138f.json` b7fb0092a18482025622b69791014fdcd1382850f425ee3c41b448f4e4b8d09b
- `.chough/history/cycles/CYC-dfc86094bb233196c40755ce/EVT-c0f51c81b37cf4d6b39ebcea.json` 2955bc1eb63d0e9e4d1a2ba986a24c361e54c653679c83585696e81e7c8713b5
- `.chough/sessions/.gitignore` 33887a1ea91c394fc4c263371641dba44db652b9defb90020f46783706a04024
- `.chough/sessions/archive/adopter.json` 654aab60f3afc2bdd49f6b62b6dc8118ae40c9db5ffc2be1d297ccba69862bb6
- `.chough/sessions/archive/archived-2.json` a78ff0649086de8a003989b43ed504230b5cad3261ccbbdcb2f4f8aea3ec3a9e
- `.chough/sessions/archive/archived-3.json` 94291d721a025008389f5c9a796fad1f00a05b5015008ec33297cefdf21d180e
- `.chough/sessions/archive/archived-4.json` d1393d283a4b8689023e0213f5b044d0b1f48ddf1f68545645e5b1041c5f3bb7
- `.chough/sessions/archive/archived-5.json` e6620a71e49c8ac9af867b516d0944c2d9d32eab139e1c8e1dc83b6e3c576603
- `.chough/sessions/archive/archived-6.json` 35bab1c297c6454d4b7ec7a5f969be6e85e94ab83b5795c46a0561f089a3b28b
- `.chough/sessions/archive/finisher.json` 3067cf1a4adaaaf5c366e18cfaf365321dcac9b72be4677b8e53fbf250be27b0
- `.chough/sessions/open-one.json` 42af8dbcdb4657035bd5240a874594bec3c3e1a44ce5f6de4e1cc09b94a8ef72
- `.chough/sessions/open-two.inbox/INB-0590e45a5d1e2245e91e1121.json` f381c1b0e0c0f1f23293174c09ce524013848071645bbf44f502effe33442fab
- `.chough/sessions/open-two.inbox/INB-5daf329377423eec43892694.json` cb7e2cdda547753f2e3c3cd5291080c86b09216cc8941735ecf0eff8e4253d5e
- `.chough/sessions/open-two.json` 28c1a659a4d14a3d5b64d36661914824165552b260f58fa53790111cf468c1eb
- `.chough/transitions.jsonl` 58a5ec984be66265ca7aa56758d64dce814300a676b63f9a5ee34104c1dd2b8f
- `knowledge/INDEX.md` edc58133029af0bea91952ac137de6d9a1a5659432413fb512021911265bf913
- `knowledge/README.md` 0518cde607c068100782610deab3cac3f705ad040062b066d344bfef2f6f0e85
- `knowledge/attachments/archived-1/sessions/archived-1.json` f628b62740fcab7478a28e0747b8ff5a436c404328631e6a35553404e538c4b5
- `knowledge/rules.md` e2f46a9fd7fd3a6b70183e265378af101493c75d465ba9167360fcd1721224bc
- `knowledge/sessions/archived-1.md` b29df7d036e3dfa2a0886c36efe68c7a5048561aa461f0d041be6198ff590655
- `knowledge/timeline.md` 1927b176e8229dd757bed0fc6fc6e661de0b93bdc886fbf1e5456224dc4b65fb

## After

- `.chough/README.md` 1335 bytes c9c7d63e0d158fcf77ab8d6214c5d6d942251b17823d97d9dacb134c04bf21db
- `.chough/archive/sessions/2026-11-11-adopter.md` 1519 bytes 62a208d6db83b262c6131b4c68163480a79771f2aacf72219421bd94c476a4b9
- `.chough/archive/sessions/2026-11-11-archived-1.md` 886 bytes d4e35b1cf62769864566ff79043de4e4173c5e1808e0f3548cee603f0e90400d
- `.chough/archive/sessions/2026-11-11-archived-2.md` 886 bytes 3eac45f0446ae010692716655859ecf9995d92fe00969a2c409d0249a00f38cd
- `.chough/archive/sessions/2026-11-11-archived-3.md` 886 bytes f170a9d15213d8e7cf05993c183908281786ab0482c286f3953bbc2eb7a52f90
- `.chough/archive/sessions/2026-11-11-archived-4.md` 886 bytes 0934b7037a5ac1decf50522cc5c2d79719700f432613e965116da6dbb682eb31
- `.chough/archive/sessions/2026-11-11-archived-5.md` 886 bytes 607ff18506ed0ca33dec0f8bb36be3d68f21095d1c6c73638058cdaffba8a62a
- `.chough/archive/sessions/2026-11-11-archived-6.md` 886 bytes 05c8e3639c796144bcbae9be6e8e08e468b2a6e37b81e1cf47ba5512e6c0bbaf
- `.chough/archive/sessions/2026-11-11-finisher.md` 1630 bytes ba916ed32aa1fe1556d938f6d879866084f4810b331bdd0375cfb39f6feb87dc
- `.chough/sessions/2026-11-11-open-one/session.md` 741 bytes e8826939feec867664b81172203002361c25900849efe23fba44ce8cf50019e6
- `.chough/sessions/2026-11-20-open-two/agent/inbox/2026-11-11-keep-the-second-run.md` 221 bytes 0c6de76ff0d70aa9efab3d61c86d8f0b7d25eab0b9afe0275e5e69448add5e91
- `.chough/sessions/2026-11-20-open-two/agent/inbox/2026-11-11-use-the-smaller-fixture.md` 231 bytes aa3b2c055ba949e85a1e33fe9aada963db3c4fca585b46ac3096c68b7d7a1ea5
- `.chough/sessions/2026-11-20-open-two/session.md` 578 bytes 5fa94161f18b3673ab6dc9ee3179dd0fd08cac1b623e90647b76d91389b9829a
- `knowledge/index.md` 1161 bytes 3fcad588b6a5efae63181e5af457070c6feb826a9ed42e749dc2bd72a3d37270
- `knowledge/journal/feed.xml` 261 bytes 9b35e960a135cdee3ef05378f5c1556d26453243a2c3bfe8a5d77fa5be12f26f

## Redactions

- `.chough/sessions/2026-11-11-open-one/session.md` body: home-path

## Removed

- `.chough/history/cycles/CYC-35b47ded993bd490850611ba/EVT-4edf61033f58689200e93314.json`: carried into `.chough/sessions/2026-11-11-open-one/session.md`; held by commit 241497c3a7f0339777e932852a70729efa4e81b6
- `.chough/history/cycles/CYC-42680cc9b00fa16e09f7b1a5/EVT-0cdfd4d090ee4a3389ad309f.json`: carried into `.chough/archive/sessions/2026-11-11-adopter.md`; held by commit 241497c3a7f0339777e932852a70729efa4e81b6
- `.chough/history/cycles/CYC-42680cc9b00fa16e09f7b1a5/EVT-da5fff4240599edcb3d6502e.json`: carried into `.chough/archive/sessions/2026-11-11-adopter.md`; held by commit 241497c3a7f0339777e932852a70729efa4e81b6
- `.chough/history/cycles/CYC-5a72dc37cbbdb146702c0e2a/EVT-ce87b276f18c71b7feab3cfb.json`: carried into `.chough/archive/sessions/2026-11-11-archived-1.md`; held by commit 241497c3a7f0339777e932852a70729efa4e81b6
- `.chough/history/cycles/CYC-621399035e1dfe1261b5ce4a/EVT-188a807240ab4176f0ec460e.json`: carried into `.chough/archive/sessions/2026-11-11-finisher.md`; held by commit 241497c3a7f0339777e932852a70729efa4e81b6
- `.chough/history/cycles/CYC-621399035e1dfe1261b5ce4a/EVT-281a5600cf62a0bc2dc15ca6.json`: carried into `.chough/archive/sessions/2026-11-11-finisher.md`; held by commit 241497c3a7f0339777e932852a70729efa4e81b6
- `.chough/history/cycles/CYC-621399035e1dfe1261b5ce4a/EVT-462d4ec830878b10804dfb40.json`: carried into `.chough/archive/sessions/2026-11-11-finisher.md`; held by commit 241497c3a7f0339777e932852a70729efa4e81b6
- `.chough/history/cycles/CYC-621399035e1dfe1261b5ce4a/EVT-72523ee7b9ed7ef5bf1f5cfc.json`: carried into `.chough/archive/sessions/2026-11-11-finisher.md`; held by commit 241497c3a7f0339777e932852a70729efa4e81b6
- `.chough/history/cycles/CYC-621399035e1dfe1261b5ce4a/EVT-f14410095b1d4e3f9b1117cc.json`: carried into `.chough/archive/sessions/2026-11-11-finisher.md`; held by commit 241497c3a7f0339777e932852a70729efa4e81b6
- `.chough/history/cycles/CYC-621399035e1dfe1261b5ce4a/EVT-fea1c6bea657eb5fc5925a67.json`: carried into `.chough/archive/sessions/2026-11-11-finisher.md`; held by commit 241497c3a7f0339777e932852a70729efa4e81b6
- `.chough/history/cycles/CYC-6aa09e1da93e854627beb733/EVT-908e95dc0772292bb0c59567.json`: carried into `.chough/archive/sessions/2026-11-11-archived-3.md`; held by commit 241497c3a7f0339777e932852a70729efa4e81b6
- `.chough/history/cycles/CYC-9fa9e31652575eb199c5ad11/EVT-f2b9b6b9907720e91cd4e0fe.json`: carried into `.chough/archive/sessions/2026-11-11-archived-4.md`; held by commit 241497c3a7f0339777e932852a70729efa4e81b6
- `.chough/history/cycles/CYC-a8df9ab1c24719335a8a4b34/EVT-4281177d85d90b750a4f130f.json`: carried into `.chough/archive/sessions/2026-11-11-archived-2.md`; held by commit 241497c3a7f0339777e932852a70729efa4e81b6
- `.chough/history/cycles/CYC-bb5c089179aed3227e15b0fd/EVT-134ea977c7a62e1800b8138f.json`: carried into `.chough/archive/sessions/2026-11-11-archived-5.md`; held by commit 241497c3a7f0339777e932852a70729efa4e81b6
- `.chough/history/cycles/CYC-dfc86094bb233196c40755ce/EVT-c0f51c81b37cf4d6b39ebcea.json`: carried into `.chough/archive/sessions/2026-11-11-archived-6.md`; held by commit 241497c3a7f0339777e932852a70729efa4e81b6
- `.chough/sessions/.gitignore`: 0.5.x temporary-file ignore; 0.6.0 writes temporaries in .chough/local/; held by commit 241497c3a7f0339777e932852a70729efa4e81b6
- `.chough/sessions/archive/adopter.json`: carried into `.chough/archive/sessions/2026-11-11-adopter.md`; held by commit 241497c3a7f0339777e932852a70729efa4e81b6
- `.chough/sessions/archive/archived-2.json`: carried into `.chough/archive/sessions/2026-11-11-archived-2.md`; held by commit 241497c3a7f0339777e932852a70729efa4e81b6
- `.chough/sessions/archive/archived-3.json`: carried into `.chough/archive/sessions/2026-11-11-archived-3.md`; held by commit 241497c3a7f0339777e932852a70729efa4e81b6
- `.chough/sessions/archive/archived-4.json`: carried into `.chough/archive/sessions/2026-11-11-archived-4.md`; held by commit 241497c3a7f0339777e932852a70729efa4e81b6
- `.chough/sessions/archive/archived-5.json`: carried into `.chough/archive/sessions/2026-11-11-archived-5.md`; held by commit 241497c3a7f0339777e932852a70729efa4e81b6
- `.chough/sessions/archive/archived-6.json`: carried into `.chough/archive/sessions/2026-11-11-archived-6.md`; held by commit 241497c3a7f0339777e932852a70729efa4e81b6
- `.chough/sessions/archive/finisher.json`: carried into `.chough/archive/sessions/2026-11-11-finisher.md`; held by commit 241497c3a7f0339777e932852a70729efa4e81b6
- `.chough/sessions/open-one.json`: carried into `.chough/sessions/2026-11-11-open-one/session.md`; held by commit 241497c3a7f0339777e932852a70729efa4e81b6
- `.chough/sessions/open-two.inbox/INB-0590e45a5d1e2245e91e1121.json`: carried into `.chough/sessions/2026-11-20-open-two/agent/inbox/2026-11-11-keep-the-second-run.md`; held by commit 241497c3a7f0339777e932852a70729efa4e81b6
- `.chough/sessions/open-two.inbox/INB-5daf329377423eec43892694.json`: carried into `.chough/sessions/2026-11-20-open-two/agent/inbox/2026-11-11-use-the-smaller-fixture.md`; held by commit 241497c3a7f0339777e932852a70729efa4e81b6
- `.chough/sessions/open-two.json`: carried into `.chough/sessions/2026-11-20-open-two/session.md`; held by commit 241497c3a7f0339777e932852a70729efa4e81b6
- `.chough/transitions.jsonl`: carried into `.chough/archive/sessions/2026-11-11-adopter.md`; held by commit 241497c3a7f0339777e932852a70729efa4e81b6
- `knowledge/INDEX.md`: the index 0.5.x `chough library index` generated; 0.6.0 writes index.md, the vault's reading guide, instead (REQ-cli-043); held by commit 241497c3a7f0339777e932852a70729efa4e81b6
- `knowledge/README.md`: the README an earlier release generated, unedited; index.md is the vault's reading guide now (REQ-cli-044); held by commit 241497c3a7f0339777e932852a70729efa4e81b6
- `knowledge/attachments/archived-1/sessions/archived-1.json`: carried into `.chough/archive/sessions/2026-11-11-archived-1.md`; held by commit 241497c3a7f0339777e932852a70729efa4e81b6
- `knowledge/rules.md`: a note 0.5.x `chough library index` generated; 0.6.0 generates no rules.md or timeline.md and links the sources from index.md (REQ-cli-044); held by commit 241497c3a7f0339777e932852a70729efa4e81b6
- `knowledge/sessions/archived-1.md`: a note 0.5.x `chough library index` generated; 0.6.0 generates no per-change, module, intent, bug, session or timeline note and links the sources from index.md (REQ-cli-044); held by commit 241497c3a7f0339777e932852a70729efa4e81b6
- `knowledge/timeline.md`: a note 0.5.x `chough library index` generated; 0.6.0 generates no rules.md or timeline.md and links the sources from index.md (REQ-cli-044); held by commit 241497c3a7f0339777e932852a70729efa4e81b6

## Kept with leaks

none

## Not carried

none

## Aliases

- `INB-0590e45a5d1e2245e91e1121` → `.chough/sessions/2026-11-20-open-two/agent/inbox/2026-11-11-keep-the-second-run.md`
- `INB-5daf329377423eec43892694` → `.chough/sessions/2026-11-20-open-two/agent/inbox/2026-11-11-use-the-smaller-fixture.md`
- `adopter` → `.chough/archive/sessions/2026-11-11-adopter.md`
- `archived-1` → `.chough/archive/sessions/2026-11-11-archived-1.md`
- `archived-2` → `.chough/archive/sessions/2026-11-11-archived-2.md`
- `archived-3` → `.chough/archive/sessions/2026-11-11-archived-3.md`
- `archived-4` → `.chough/archive/sessions/2026-11-11-archived-4.md`
- `archived-5` → `.chough/archive/sessions/2026-11-11-archived-5.md`
- `archived-6` → `.chough/archive/sessions/2026-11-11-archived-6.md`
- `finisher` → `.chough/archive/sessions/2026-11-11-finisher.md`
- `open-one` → `.chough/sessions/2026-11-11-open-one/session.md`
- `open-two` → `.chough/sessions/2026-11-20-open-two/session.md`

## Follow-up

Declare Chough's merge drivers so generated files and bugs merge without a hand-resolved conflict (REQ-cli-047): add `.chough/bugs/*.md merge=chough-bug`, `knowledge/index.md merge=chough-generated`, `knowledge/journal/feed.xml merge=chough-generated` to `.gitattributes` and commit it, then run `chough doctor --fix` in each clone, which sets `merge.chough-bug.driver` and `merge.chough-generated.driver`.
