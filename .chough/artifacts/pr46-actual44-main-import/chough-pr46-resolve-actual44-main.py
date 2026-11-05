import pathlib,subprocess,os,json,hashlib,collections,shutil
root=pathlib.Path('/tmp/chough-tickets-20261102-JAIhPp');os.chdir(root);env=dict(os.environ,GIT_OPTIONAL_LOCKS='0')
def git(*a):return subprocess.check_output(['git',*a],env=env)
p=pathlib.Path(pathlib.Path('/tmp/chough-pr46-actual44-main-dir.txt').read_text().strip());actual='16fe55da9ee900edf34a4d3ded517f36854a46d9';archived='122b8e9e8b913b49e618c59c2906e9e26613fd7e'
assert git('rev-parse','MERGE_HEAD').decode().strip()==actual;assert git('rev-parse',actual+'^{tree}')==git('rev-parse',archived+'^{tree}');subprocess.run(['git','merge-base','--is-ancestor',archived,'HEAD'],check=True)
conflicts=git('diff','--name-only','--diff-filter=U').decode().splitlines();expected={'.chough/cases/cli.cases.toml','.chough/session.json','.chough/transitions.jsonl','docs/FRD.md','specs/cli/cli.spec.md','specs/cli/context.md','specs/cli/requirements.md'};assert set(conflicts)==expected,conflicts
for path in conflicts:
 if path not in ['.chough/session.json','.chough/transitions.jsonl']:(root/path).write_bytes(git('show','HEAD:'+path))
base=json.loads((p/'base.session.json').read_text());ours=json.loads((p/'ours.session.json').read_text());incoming=json.loads((p/'actual44main.session.json').read_text());policy=json.loads((root/'.chough/artifacts/pr46-checkpoint-reference-index/policy.json').read_text());retired=policy['retired_from_active_index'];ids={x['id'] for x in retired}
incoming['observations']=[x for x in incoming['observations'] if x not in ids]
for x in retired:
 if x['record'] not in incoming['references']['files']:incoming['references']['files'].append(x['record'])
index='.chough/artifacts/pr46-checkpoint-reference-index/policy.json'
if index not in incoming['references']['files']:incoming['references']['files'].append(index)
fields=['objective','intent_scope','modules','stage','native_change','next_action','run_consents','blockers','awaiting_human','history_cursor']
for f in fields:
 if f in base:incoming[f]=base[f]
 else:incoming.pop(f,None)
(p/'actual44main.projected-owner-selected.session.json').write_text(json.dumps(incoming,indent=2)+'\n');out=p/'merged.session.json';out.write_bytes((p/'ours.session.json').read_bytes())
args=[str(root/'target/debug/chough'),'checkpoint','--merge',str(p/'base.session.json'),str(out),str(p/'actual44main.projected-owner-selected.session.json')];r=subprocess.run(args,stdout=subprocess.PIPE,stderr=subprocess.STDOUT);(p/'checkpoint-merge.log').write_bytes(r.stdout);assert r.returncode==0,r.stdout.decode()
merged=json.loads(out.read_text());assert merged==ours,'Redundant squash must not add/replay checkpoint state';assert out.read_bytes()==(p/'ours.session.json').read_bytes(),'Expected byte-identical selected checkpoint'
(root/'.chough/session.json').write_bytes(out.read_bytes())
ourslog=(p/'ours.transitions.jsonl').read_bytes();mainlog=(p/'actual44main.transitions.jsonl').read_bytes();assert mainlog==(p/'archived44.transitions.jsonl').read_bytes();assert not (collections.Counter(mainlog.splitlines())-collections.Counter(ourslog.splitlines()))
(root/'.chough/transitions.jsonl').write_bytes(ourslog)
proof={}
for name in ['ours','actual44main']:
 for line in (p/(name+'.proof-inventory.txt')).read_text().splitlines():
  info,path=line.split('\t',1);mode,kind,oid=info.split();assert kind=='blob'
  if path in proof:assert proof[path]==(mode,oid),path
  proof[path]=(mode,oid)
for path,(mode,oid) in proof.items():
 b=(root/path).read_bytes();assert hashlib.sha1(b'blob '+str(len(b)).encode()+b'\0'+b).hexdigest()==oid,path
for kind in ['verify','msrv']:
 d=json.loads(pathlib.Path('/tmp/chough-pr46-combined-'+kind+'-result.json').read_text())
 for path,digest in d['input']['source_sha256'].items():assert hashlib.sha256((root/path).read_bytes()).hexdigest()==digest,path
 assert hashlib.sha256((root/'.chough/cases/cli.cases.toml').read_bytes()).hexdigest()==d['input']['cases_sha256']
for path in conflicts:
 if path not in ['.chough/session.json','.chough/transitions.jsonl']:assert (root/path).read_bytes()==git('show','HEAD:'+path),path
report={'actual_main':actual,'archived44':archived,'whole_tree_identical':True,'tree':git('rev-parse',actual+'^{tree}').decode().strip(),'archived44_already_ancestor_of_qualified_source':True,'qualified_source':'b04a8d517837c28e0912bb8af832845fc9a5bfaa','qualified_source_tests_skills_cases_unchanged':True,'canonical_version_unchanged':24,'checkpoint_byte_identical':True,'active_observations':len(merged['observations']),'retired_projection_same24':True,'all24direct_file_pins_preserved':all(x['record'] in merged['references']['files'] for x in retired),'transitions_byte_identical':True,'transition_occurrences':len(ourslog.splitlines()),'replayed_occurrences':0,'protected_parent_blobs':len(proof),'proof_mismatches':0,'rerun_decision':'No additional full product rerun: the actual squash whole tree equals its premerged archived tip, and qualified source/test/skill/case identities are unchanged.','operand_note':'actual44main.session.json is the exact historical squash checkpoint. actual44main.projected-owner-selected.session.json is the disclosed derived input with the same24retired-pointer projection/direct pins and base-aligned atomic group.'}
(p/'equivalence-and-preservation.json').write_text(json.dumps(report,indent=2)+'\n');print(json.dumps(report,indent=2))
artifact=root/'.chough/artifacts/pr46-actual44-main-import';assert not artifact.exists();artifact.mkdir()
for file in p.iterdir():
 if file.is_file():shutil.copyfile(file,artifact/file.name)
for script in ['/tmp/chough-pr46-import-actual44-main.py','/tmp/chough-pr46-resolve-actual44-main.py']:shutil.copyfile(script,artifact/pathlib.Path(script).name)
subprocess.run(['git','add','--',*conflicts,'.chough/artifacts/pr46-actual44-main-import','.specsync/changes/integrate-accepted-history-and-workflow-changes-into-pr-46-report-reads'],check=True)
assert not git('diff','--name-only','--diff-filter=U').strip()
paths=git('diff','HEAD','--name-only').decode().splitlines();assert all(x.startswith(('.chough/artifacts/pr46-actual44-main-import/','.specsync/changes/integrate-accepted-history-and-workflow-changes-into-pr-46-report-reads/')) for x in paths),paths
subprocess.run(['git','commit','-m','Merge actual PR 44 squash without replaying integrated records'],check=True)
assert not git('status','--porcelain').strip();print('HEAD',git('rev-parse','HEAD').decode().strip())
