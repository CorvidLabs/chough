import pathlib,subprocess,json,re,hashlib,os,collections,tomllib
root=pathlib.Path('/tmp/chough-tickets-20261102-JAIhPp');os.chdir(root)
capture=pathlib.Path(pathlib.Path('/tmp/chough-pr46-final44-integration-dir.txt').read_text().strip());env=dict(os.environ,GIT_OPTIONAL_LOCKS='0')
def git(*a):return subprocess.check_output(['git',*a],env=env)
pat=re.compile(r'^<<<<<<< [^\n]*\n(.*?)^=======\n(.*?)^>>>>>>> [^\n]*\n',re.M|re.S)
for name in ['docs/FRD.md','specs/cli/context.md']:
 p=root/name;s=p.read_text();s,n=pat.subn(lambda m:m.group(1).rstrip()+'\n\n'+m.group(2).lstrip(),s);assert n>0 and '<<<<<<<' not in s;p.write_text(s)
p=root/'.chough/cases/cli.cases.toml';s=p.read_text();s,n=pat.subn(lambda m:m.group(1).rstrip()+'\n\n[[case]]\n'+m.group(2).lstrip(),s);assert n==1;p.write_text(s)
merged_cases={c['id']:c for c in tomllib.loads(s)['case']};assert len(merged_cases)==len(tomllib.loads(s)['case'])
for ref in ['HEAD','122b8e9e8b913b49e618c59c2906e9e26613fd7e']:
 for case in tomllib.loads(git('show',ref+':.chough/cases/cli.cases.toml').decode())['case']:assert merged_cases[case['id']]==case,case['id']
blocks={};preamble=None
for ref in ['HEAD','122b8e9e8b913b49e618c59c2906e9e26613fd7e']:
 chunks=re.split(r'(?=^### REQ-cli-\d+)',git('show',ref+':specs/cli/requirements.md').decode(),flags=re.M)
 if preamble is None:preamble=chunks[0]
 for b in chunks[1:]:
  ident=re.match(r'### (REQ-cli-\d+)',b).group(1)
  if ident in blocks:assert blocks[ident].strip()==b.strip(),ident
  blocks[ident]=b
(root/'specs/cli/requirements.md').write_text(preamble+''.join(blocks[k].rstrip()+'\n\n' for k in sorted(blocks,key=lambda k:int(k.rsplit('-',1)[1]))).rstrip()+'\n')
p=root/'specs/cli/cli.spec.md';s=p.read_text();m=pat.search(s);assert m and '| 23 |' in m.group(1) and '| 23 |' in m.group(2)
s=pat.sub(lambda m:m.group(2)+m.group(1).replace('| 23 |','| 24 |',1),s);s,n=re.subn(r'^version: 23$', 'version: 24',s,flags=re.M);assert n==1;p.write_text(s)
base=json.loads((capture/'base.session.json').read_text());ours=json.loads((capture/'ours.session.json').read_text());theirs=json.loads((capture/'theirs44.session.json').read_text())
fields=['objective','intent_scope','modules','stage','native_change','next_action','run_consents','blockers','awaiting_human','history_cursor'];adjusted=dict(theirs)
for f in fields:
 if f in base:adjusted[f]=base[f]
 else:adjusted.pop(f,None)
adj=capture/'theirs44.owner-selected.session.json';adj.write_text(json.dumps(adjusted,indent=2)+'\n');out=capture/'merged44.session.json';out.write_bytes((capture/'ours.session.json').read_bytes())
r=subprocess.run([str(root/'target/debug/chough'),'checkpoint','--merge',str(capture/'base.session.json'),str(out),str(adj)],stdout=subprocess.PIPE,stderr=subprocess.STDOUT);(capture/'checkpoint44-merge.log').write_bytes(r.stdout);assert r.returncode==0,r.stdout.decode()
merged=json.loads(out.read_text());assert all(merged.get(f)==ours.get(f) for f in fields)
def arrays(d):
 for key in ['decisions','open_questions','completed','observations','bugs']:yield key,d[key]
 for key,value in d['references'].items():yield 'references.'+key,value
marr=dict(arrays(merged));barr=dict(arrays(base))
for name,parent in [('ours',ours),('theirs44',theirs)]:
 for key,values in arrays(parent):assert set(values)-set(barr[key]) <= set(marr[key]),(name,key)
(root/'.chough/session.json').write_bytes(out.read_bytes())
b=(capture/'base.transitions.jsonl').read_bytes();o=(capture/'ours.transitions.jsonl').read_bytes();t=(capture/'theirs44.transitions.jsonl').read_bytes();bc=collections.Counter(b.splitlines());oc=collections.Counter(o.splitlines());tc=collections.Counter(t.splitlines())
assert not (bc-oc) and not (bc-tc);assert not ((oc-bc)&(tc-bc)), 'shared transported additions require explicit mapping'
remaining=bc.copy();additions=[];mapping=[]
for n,line in enumerate(t.splitlines(),1):
 if remaining[line]:remaining[line]-=1;mapping.append({'incoming_line':n,'source':'shared-base'})
 else:additions.append(line);mapping.append({'incoming_line':n,'source':'theirs44-addition','merged_line':len(o.splitlines())+len(additions)})
joined=o+b''.join(line+b'\n' for line in additions);assert collections.Counter(joined.splitlines())==oc+tc-bc
assert all(json.loads(x) for x in joined.splitlines());(root/'.chough/transitions.jsonl').write_bytes(joined);(capture/'transition-occurrence-map.json').write_text(json.dumps(mapping,indent=2)+'\n')
proof={}
for parent in ['ours','theirs44']:
 for line in (capture/(parent+'.proof-inventory.txt')).read_text().splitlines():
  info,path=line.split('\t',1);mode,kind,oid=info.split();assert kind=='blob'
  if path in proof:assert proof[path]==(mode,oid),path
  proof[path]=(mode,oid)
mismatches=[]
for path,(mode,oid) in proof.items():
 p=root/path
 if not p.is_file():mismatches.append(path+':missing');continue
 content=p.read_bytes();actual=hashlib.sha1(b'blob '+str(len(content)).encode()+b'\0'+content).hexdigest()
 if actual!=oid:mismatches.append(path+':identity')
assert not mismatches,mismatches
# Verify union of predecessor identifiers and exact runtime behavior imports.
for path in ['src/history.rs','src/checkpoint.rs','src/stage.rs','src/timing.rs','tests/history.rs','tests/timing.rs','tests/common/mod.rs']:
 assert (root/path).read_bytes()==git('show','a788869b32b8442cbc8d7be2bf19c19a922954a8:'+path),path
for path in ['src/cases.rs','src/evidence.rs','src/main.rs','src/records.rs','tests/next.rs']:
 assert (root/path).read_bytes()==git('show','HEAD:'+path),path
for path in ['src/skill.rs','tests/workflow_authority.rs','skills/chough-workflow/SKILL.md','skills/chough-workflow/references/plan.md','skills/chough-workflow/references/accept.md','skills/chough-workflow/references/lifecycle.md']:
 assert (root/path).read_bytes()==git('show','122b8e9e8b913b49e618c59c2906e9e26613fd7e:'+path),path
oldhashes=set(re.findall(r'"([0-9a-f]{64})"',git('show','HEAD:src/skill.rs').decode()));newhashes=set(re.findall(r'"([0-9a-f]{64})"',(root/'src/skill.rs').read_text()));assert oldhashes<=newhashes
report={'ours_head':git('rev-parse','HEAD').decode().strip(),'incoming44':'122b8e9e8b913b49e618c59c2906e9e26613fd7e','qualified44_source':'769b5269b87d2f822a1b41a4276eccd7d0fa3770','proof_blobs':len(proof),'proof_mismatches':mismatches,'atomic_cycle_preserved':True,'additive_collections_preserved':True,'base_transition_occurrences':sum(bc.values()),'ours_added_occurrences':sum((oc-bc).values()),'incoming44_added_occurrences':sum((tc-bc).values()),'merged_transition_occurrences':len(joined.splitlines()),'transition_order_note':'Preserved existing ours order and appended unmatched incoming occurrences with explicit source mapping; not an inferred global causal order.','canonical_version':24,'requirement_count':len(blocks),'case_count':len(merged_cases),'predecessor_hash_union_preserved':True,'behavior_import_identities_checked':True}
(capture/'reconciliation44.json').write_text(json.dumps(report,indent=2)+'\n');print(json.dumps(report,indent=2))
