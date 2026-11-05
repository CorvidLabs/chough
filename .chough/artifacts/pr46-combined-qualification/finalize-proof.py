import pathlib,json,subprocess,os,hashlib,gzip
root=pathlib.Path('/tmp/chough-tickets-20261102-JAIhPp');os.chdir(root);env=dict(os.environ,GIT_OPTIONAL_LOCKS='0');dest=root/'.chough/artifacts/pr46-combined-qualification'
queries={e:pathlib.Path(p) for e,p in [('local-macos-arm64','/tmp/chough-pr46-combined-final-status-local.json'),('local-macos-arm64-rust-1.88.0','/tmp/chough-pr46-combined-final-status-msrv.json'),('repository-text','/tmp/chough-pr46-combined-final-status-repository.json')]};data={e:json.loads(p.read_text()) for e,p in queries.items()};ids=json.loads((dest/'final-observation-ids.json').read_text());current=[]
for ident in ids:
 record=json.loads((root/'.chough/evidence'/(ident+'.json')).read_text());e=record['environment'];match=next(x for x in data[e]['observations'] if x['record']['id']==ident);assert match['freshness']=='current-observation',match
 current.append({'observation':ident,'environment':e,'freshness':match['freshness']})
raw_queries=[]
for e,p in queries.items():
 payload=p.read_bytes();target=dest/(p.name+'.gz');assert not target.exists();target.write_bytes(gzip.compress(payload,mtime=0));raw_queries.append({'environment':e,'raw_sha256':hashlib.sha256(payload).hexdigest(),'artifact':str(target.relative_to(root))})
(dest/'final-freshness.json').write_text(json.dumps({'observations':current,'all18_current_in_recorded_environments':True,'raw_queries':raw_queries,'note':'Native verification was refreshed after these record writes; this file records product observation freshness, not human acceptance.'},indent=2)+'\n')
def git(*a):return subprocess.check_output(['git',*a],env=env)
proof={}
for ref in ['HEAD','16fe55da9ee900edf34a4d3ded517f36854a46d9']:
 for line in git('ls-tree','-r',ref).decode().splitlines():
  info,path=line.split('\t',1)
  if not path.startswith(('.specsync/archive/','.chough/evidence/','.chough/artifacts/','docs/exercises/handoff-cold-start/')):continue
  mode,kind,oid=info.split();assert kind=='blob'
  if path in proof:assert proof[path]==(mode,oid),path
  proof[path]=(mode,oid)
for path,(mode,oid) in proof.items():
 content=(root/path).read_bytes();assert hashlib.sha1(b'blob '+str(len(content)).encode()+b'\0'+content).hexdigest()==oid,path
for kind in ['verify','msrv']:
 r=json.loads((dest/(kind+'-result.json')).read_text())
 for path,digest in r['input']['source_sha256'].items():assert hashlib.sha256((root/path).read_bytes()).hexdigest()==digest,path
 assert hashlib.sha256((root/'.chough/cases/cli.cases.toml').read_bytes()).hexdigest()==r['input']['cases_sha256']
checkpoint=json.loads((root/'.chough/session.json').read_text());assert checkpoint['stage']['current']=='accept' and len(checkpoint['observations'])==196 and len((root/'.chough/session.json').read_bytes())<=65536
policy=json.loads((root/'.chough/artifacts/pr46-checkpoint-reference-index/policy.json').read_text())
for retired in policy['retired_from_active_index']:
 assert retired['id'] not in checkpoint['observations'] and retired['record'] in checkpoint['references']['files']
 assert hashlib.sha256((root/retired['record']).read_bytes()).hexdigest()==retired['sha256']
 for path,digest in retired['artifacts'].items():assert hashlib.sha256((root/path).read_bytes()).hexdigest()==digest,path
preservation={'protected_committed_parent_blobs':len(proof),'mismatches':0,'source_test_skill_case_identities_unchanged':True,'canonical_version':24,'active_observations':196,'checkpoint_bytes':len((root/'.chough/session.json').read_bytes()),'file_references':len(checkpoint['references']['files']),'retired_record_pins':24,'retired_record_and_artifact_hashes_unchanged':True,'retained_prequalification_transition_occurrences':218,'new_legitimate_stage_occurrences':2,'current_transition_occurrences':len((root/'.chough/transitions.jsonl').read_bytes().splitlines()),'all18_new_observations_current_in_their_recorded_environments':True,'no_live_adoption_or_cleanup':True,'no_release_deployment_or_hosted_new_tip_claim':True};assert preservation['current_transition_occurrences']==220
(dest/'final-preservation.json').write_text(json.dumps(preservation,indent=2)+'\n');(dest/'finalize-proof.py').write_bytes(pathlib.Path('/tmp/chough-pr46-finalize-proof.py').read_bytes());print(json.dumps(preservation,indent=2))
# Only new records/artifacts/checkpoint updates and native CLI bookkeeping may differ.
changed=git('status','--porcelain','--untracked-files=all').decode().splitlines();assert all(x[3:].startswith(('.chough/','.specsync/changes/integrate-accepted-history-and-workflow-changes-into-pr-46-report-reads/')) for x in changed),changed
subprocess.run(['git','diff','--check','--','src/','tests/','skills/','.agents/skills/','.claude/skills/','specs/','docs/','CHANGELOG.md','.chough/session.json','.chough/transitions.jsonl','.chough/cases/'],check=True)
subprocess.run(['git','add','--','.chough','.specsync/changes/integrate-accepted-history-and-workflow-changes-into-pr-46-report-reads'],check=True)
subprocess.run(['git','commit','-m','test(chough): qualify combined history workflow and report reads'],check=True)
assert not git('status','--porcelain').strip();print('QUALIFICATION_HEAD',git('rev-parse','HEAD').decode().strip())
