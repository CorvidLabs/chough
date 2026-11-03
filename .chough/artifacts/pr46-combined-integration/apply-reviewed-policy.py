import pathlib,subprocess,json,hashlib,os,collections,shutil,re,tomllib
root=pathlib.Path('/tmp/chough-tickets-20261102-JAIhPp');os.chdir(root)
capture=pathlib.Path(pathlib.Path('/tmp/chough-pr46-final44-integration-dir.txt').read_text().strip());policy=pathlib.Path('/var/folders/5k/616qy7sj28l0d53k4c6w_4x00000gn/T/chough-pr46-bounded-refs-m67rz7_b');env=dict(os.environ,GIT_OPTIONAL_LOCKS='0')
def git(*a):return subprocess.check_output(['git',*a],env=env)
raw=(policy/'derived/ours.json').read_bytes();assert hashlib.sha256(raw).hexdigest()=='e9786a91a451f0e251ed6835254fc6384abdf0f3542c7e122e11c7ec7099adac'
merged=json.loads(raw);ours=json.loads((capture/'ours.session.json').read_text());base=json.loads((capture/'base.session.json').read_text());theirs=json.loads((capture/'theirs44.session.json').read_text());selection=json.loads((policy/'policy.json').read_text());retired=selection['retired_from_active_index']
assert len(merged['observations'])==178 and len(merged['references']['files'])==36 and len(retired)==24
fields=['objective','intent_scope','modules','stage','native_change','next_action','run_consents','blockers','awaiting_human','history_cursor'];assert all(merged.get(k)==ours.get(k) for k in fields)
additive=set(ours['observations']+theirs['observations'])-set(base['observations']);assert len(additive)==65 and additive<=set(merged['observations'])
for record in retired:
 assert record['id'] not in merged['observations'] and record['record'] in merged['references']['files']
 assert hashlib.sha256((root/record['record']).read_bytes()).hexdigest()==record['sha256']
 for path,digest in record['artifacts'].items():assert hashlib.sha256((root/path).read_bytes()).hexdigest()==digest,path
index=root/'.chough/artifacts/pr46-checkpoint-reference-index';assert not index.exists();index.mkdir()
for p in policy.iterdir():
 if p.is_file():shutil.copyfile(p,index/p.name)
shutil.copytree(policy/'derived',index/'derived')
rawdir=index/'raw-parent-operands';rawdir.mkdir()
for name in ['base.session.json','ours.session.json','theirs44.session.json','base.transitions.jsonl','ours.transitions.jsonl','theirs44.transitions.jsonl','base.proof-inventory.txt','ours.proof-inventory.txt','theirs44.proof-inventory.txt','refs.json']:
 shutil.copyfile(capture/name,rawdir/name)
assert (rawdir/'theirs44.session.json').read_bytes()==git('show','122b8e9e8b913b49e618c59c2906e9e26613fd7e:.chough/session.json')
(index/'operand-provenance.json').write_text(json.dumps({'raw_parent_operands':'raw-parent-operands holds exact Git parent inputs. In particular theirs44.session.json is the original final44 checkpoint, not the owner-selected input.','derived_operands':'derived/base.json and derived/theirs.json are root-selected helper inputs with superseded active references removed; derived/theirs.json also aligns the complete incoming current-cycle group to base. derived/ours.json is the supported helper output selected by root. These derived inputs are not historical parent bytes.','reviewed_candidate_sha256':'e9786a91a451f0e251ed6835254fc6384abdf0f3542c7e122e11c7ec7099adac','scope':'Retire only superseded active pointers while retaining all underlying records/artifacts and direct checkpoint file protection. No product bound or cleanup behavior changes.'},indent=2)+'\n')
(root/'.chough/session.json').write_bytes(raw)
b=(capture/'base.transitions.jsonl').read_bytes();o=(capture/'ours.transitions.jsonl').read_bytes();t=(capture/'theirs44.transitions.jsonl').read_bytes();bc=collections.Counter(b.splitlines());oc=collections.Counter(o.splitlines());tc=collections.Counter(t.splitlines());assert not (bc-oc) and not (bc-tc);assert not ((oc-bc)&(tc-bc))
remaining=bc.copy();adds=[];mapping=[]
for n,line in enumerate(t.splitlines(),1):
 if remaining[line]:remaining[line]-=1;mapping.append({'incoming_line':n,'source':'shared-base'})
 else:adds.append(line);mapping.append({'incoming_line':n,'source':'incoming44','merged_line':len(o.splitlines())+len(adds)})
joined=o+b''.join(x+b'\n' for x in adds);assert collections.Counter(joined.splitlines())==oc+tc-bc;assert all(json.loads(x) for x in joined.splitlines());(root/'.chough/transitions.jsonl').write_bytes(joined)
(capture/'transition-occurrence-map.json').write_text(json.dumps(mapping,indent=2)+'\n')
proof={}
for parent in ['ours','theirs44']:
 for line in (capture/(parent+'.proof-inventory.txt')).read_text().splitlines():
  info,path=line.split('\t',1);mode,kind,oid=info.split();assert kind=='blob'
  if path in proof:assert proof[path]==(mode,oid),path
  proof[path]=(mode,oid)
for path,(mode,oid) in proof.items():
 content=(root/path).read_bytes();assert hashlib.sha1(b'blob '+str(len(content)).encode()+b'\0'+content).hexdigest()==oid,path
oldhashes=set(re.findall(r'"([0-9a-f]{64})"',git('show','HEAD:src/skill.rs').decode()));newhashes=set(re.findall(r'"([0-9a-f]{64})"',(root/'src/skill.rs').read_text()));assert oldhashes<=newhashes
case_count=len(tomllib.loads((root/'.chough/cases/cli.cases.toml').read_text())['case'])
report={'ours_head':git('rev-parse','HEAD').decode().strip(),'incoming44':'122b8e9e8b913b49e618c59c2906e9e26613fd7e','qualified44_source':'769b5269b87d2f822a1b41a4276eccd7d0fa3770','proof_blobs':len(proof),'proof_mismatches':0,'atomic_cycle_preserved':True,'all65additive_observation_ids_retained':True,'root_selected_retired_pointers':24,'direct_protected_record_references':24,'active_observations_before_final_runs':178,'checkpoint_files':36,'checkpoint_bytes':len(raw),'base_transition_occurrences':sum(bc.values()),'ours_added_occurrences':sum((oc-bc).values()),'incoming44_added_occurrences':sum((tc-bc).values()),'merged_transition_occurrences':len(joined.splitlines()),'transition_order_note':'Existing ours order retained and unmatched incoming occurrences appended with explicit source mapping. No global causal ordering is inferred.','canonical_version':24,'case_count':case_count,'predecessor_hash_union_preserved':True}
(capture/'reconciliation44.json').write_text(json.dumps(report,indent=2)+'\n');print(json.dumps(report,indent=2))
artifact=root/'.chough/artifacts/pr46-combined-integration';assert not artifact.exists();artifact.mkdir()
for name in ['reconciliation44.json','transition-occurrence-map.json','source-stable-identities.json','merge44.log','checkpoint44-merge.log']:
 shutil.copyfile(capture/name,artifact/name)
shutil.copyfile('/tmp/chough-pr46-resolve-final44.py',artifact/'resolve-final44.py');shutil.copyfile('/tmp/chough-pr46-apply-reviewed44-policy.py',artifact/'apply-reviewed-policy.py')
# These completed boxes cover source/proof preparation, not product qualification.
tasks=root/'.specsync/changes/integrate-accepted-history-and-workflow-changes-into-pr-46-report-reads/tasks.md';s=tasks.read_text();assert s.count('- [ ]')==3;tasks.write_text(s.replace('- [ ]','- [x]'))
