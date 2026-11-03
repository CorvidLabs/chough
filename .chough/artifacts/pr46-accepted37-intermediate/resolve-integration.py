import pathlib,subprocess,json,re,hashlib,os,collections
root=pathlib.Path('/tmp/chough-tickets-20261102-JAIhPp');os.chdir(root)
capture=pathlib.Path('/tmp/chough-pr46-accepted-integration-dir.txt').read_text().strip();capture=pathlib.Path(capture)
env=dict(os.environ,GIT_OPTIONAL_LOCKS='0')
def git(*a):return subprocess.check_output(['git',*a],env=env)
def union_conflicts(path):
 p=root/path;s=p.read_text()
 if '<<<<<<<' not in s:return
 pattern=re.compile(r'^<<<<<<< [^\n]*\n(.*?)^=======\n(.*?)^>>>>>>> [^\n]*\n',re.M|re.S)
 s,n=pattern.subn(lambda m:m.group(1).rstrip()+'\n\n'+m.group(2).lstrip(),s)
 assert n>0,path
 assert '<<<<<<<' not in s,path
 p.write_text(s)
for p in ['CHANGELOG.md','docs/architecture.md','specs/cli/context.md']:union_conflicts(p)
# Preserve exact requirement bodies from both accepted parents and order by stable ID.
refs=['HEAD','a788869b32b8442cbc8d7be2bf19c19a922954a8'];blocks={};preamble=None
for ref in refs:
 s=git('show',ref+':specs/cli/requirements.md').decode();chunks=re.split(r'(?=^### REQ-cli-\d+)',s,flags=re.M)
 if preamble is None:preamble=chunks[0]
 for b in chunks[1:]:
  ident=re.match(r'### (REQ-cli-\d+)',b).group(1)
  if ident in blocks:assert blocks[ident].strip()==b.strip(),ident
  blocks[ident]=b
(root/'specs/cli/requirements.md').write_text(preamble+''.join(blocks[k].rstrip()+'\n\n' for k in sorted(blocks,key=lambda k:int(k.rsplit('-',1)[1]))).rstrip()+'\n')
# Main owns canonical22; existing report changes follow at23 until final44 reconciliation.
p=root/'specs/cli/cli.spec.md';s=p.read_text()
pattern=re.compile(r'^<<<<<<< [^\n]*\n(.*?)^=======\n(.*?)^>>>>>>> [^\n]*\n',re.M|re.S)
m=pattern.search(s)
if m:
 assert '| 22 |' in m.group(1) and '| 22 |' in m.group(2)
 s=pattern.sub(lambda m:m.group(2)+m.group(1).replace('| 22 |','| 23 |',1),s)
 s,n=re.subn(r'^version: 22$', 'version: 23',s,flags=re.M);assert n==1
else:assert 'version: 23' in s and '| 23 |' in s
p.write_text(s)
# Keep PR46's complete current-cycle group. Only the disposable incoming copy is aligned
# to base to permit the supported helper to merge additive collections.
base=json.loads((capture/'base.session.json').read_text());ours=json.loads((capture/'prepared46.session.json').read_text());theirs=json.loads((capture/'main37.session.json').read_text())
fields=['objective','intent_scope','modules','stage','native_change','next_action','run_consents','blockers','awaiting_human','history_cursor']
assert all('history_cursor' not in x for x in (base,ours,theirs))
adjusted=dict(theirs)
for f in fields:
 if f in base:adjusted[f]=base[f]
 else:adjusted.pop(f,None)
adj=capture/'main37.owner-selected.session.json';adj.write_text(json.dumps(adjusted,indent=2)+'\n')
out=capture/'merged37.session.json';out.write_bytes((capture/'prepared46.session.json').read_bytes())
r=subprocess.run([str(root/'target/debug/chough'),'checkpoint','--merge',str(capture/'base.session.json'),str(out),str(adj)],stdout=subprocess.PIPE,stderr=subprocess.STDOUT)
(capture/'checkpoint37-merge.log').write_bytes(r.stdout);assert r.returncode==0,r.stdout.decode()
merged=json.loads(out.read_text())
assert all(merged.get(f)==ours.get(f) for f in fields)
def arrays(d):
 for key in ['decisions','open_questions','completed','observations','bugs']:yield key,d[key]
 for key,value in d['references'].items():yield 'references.'+key,value
marr=dict(arrays(merged));barr=dict(arrays(base))
for name,parent in [('ours',ours),('main37',theirs)]:
 for key,values in arrays(parent):
  assert set(values)-set(barr[key]) <= set(marr[key]),(name,key)
(root/'.chough/session.json').write_bytes(out.read_bytes())
b=(capture/'base.transitions.jsonl').read_bytes();o=(capture/'prepared46.transitions.jsonl').read_bytes();t=(capture/'main37.transitions.jsonl').read_bytes()
bc=collections.Counter(b.splitlines());oc=collections.Counter(o.splitlines());tc=collections.Counter(t.splitlines())
assert not (bc-oc) and not (bc-tc)
assert not ((oc-bc)&(tc-bc)), 'shared transported additions require occurrence reconciliation'
remaining=bc.copy();additions=[]
for line in t.splitlines():
 if remaining[line]:remaining[line]-=1
 else:additions.append(line)
merged_transitions=o+b''.join(line+b'\n' for line in additions)
assert collections.Counter(merged_transitions.splitlines())==oc+tc-bc
assert all(json.loads(x) for x in merged_transitions.splitlines())
(root/'.chough/transitions.jsonl').write_bytes(merged_transitions)
# Verify every imported proof blob and reject any conflicting same-path identity.
proof={}
for parent in ['ours','main37']:
 for line in (capture/(parent+'.proof-inventory.txt')).read_text().splitlines():
  info,path=line.split('\t',1);mode,kind,oid=info.split();assert kind=='blob'
  if path in proof:assert proof[path]==(mode,oid),path
  proof[path]=(mode,oid)
mismatches=[]
for path,(mode,oid) in proof.items():
 p=root/path
 if not p.is_file():mismatches.append(path+':missing');continue
 b=p.read_bytes();actual=hashlib.sha1(b'blob '+str(len(b)).encode()+b'\0'+b).hexdigest()
 if actual!=oid:mismatches.append(path+':identity')
assert not mismatches,mismatches
report={'ours_head':git('rev-parse','HEAD').decode().strip(),'main37':'a788869b32b8442cbc8d7be2bf19c19a922954a8','proof_blobs':len(proof),'proof_mismatches':mismatches,'atomic_cycle_preserved':True,'additive_collections_preserved':True,'base_transition_lines':len(b.splitlines()),'ours_appended_occurrences':sum((oc-bc).values()),'main37_appended_occurrences':sum((tc-bc).values()),'merged_transition_lines':len(merged_transitions.splitlines()),'canonical_version':23,'requirement_count':len(blocks)}
(capture/'reconciliation37.json').write_text(json.dumps(report,indent=2)+'\n');print(json.dumps(report,indent=2))
