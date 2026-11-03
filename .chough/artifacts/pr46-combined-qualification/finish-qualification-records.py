import pathlib,subprocess,os,json,hashlib
root=pathlib.Path('/tmp/chough-tickets-20261102-JAIhPp');os.chdir(root);env=dict(os.environ);env['PATH']='/Users/leif/.cargo/bin:'+env['PATH'];cli=str(root/'target/debug/chough');dest=root/'.chough/artifacts/pr46-combined-qualification'
cases=json.loads(pathlib.Path('/tmp/chough-pr46-combined-cases.json').read_text());assert len(cases)==16
index=[];raw=dest/'raw-case-results';assert not raw.exists();raw.mkdir()
for c in cases:
 r=c['result'];assert r['outcome']=='passed';ident=r.get('observation') or pathlib.Path(r['recorded']).stem
 record=json.loads((root/'.chough/evidence'/ (ident+'.json')).read_text());assert record['outcome']=='passed'
 index.append({'case':c['case'],'observation':ident,'outcome':record['outcome'],'environment':record['environment'],'method':record['method']})
 for suffix in ['.json','.stderr']:
  p=pathlib.Path('/tmp/chough-pr46-combined-'+c['case']+suffix);(raw/p.name).write_bytes(p.read_bytes())
(dest/'cases.json').write_bytes(pathlib.Path('/tmp/chough-pr46-combined-cases.json').read_bytes());(dest/'case-observation-index.json').write_text(json.dumps(index,indent=2)+'\n')
(dest/'manual-response-wrapper-note.json').write_text(json.dumps({'phase':'after all16cases were recorded passed','failure':'The wrapper attempted result.observation for the agent-assisted record response, which uses recorded:path instead. It stopped after saving the16thpassed result.','resolution':'Index the already existing recorded path. Do not rerun or duplicate the case.','raw_manual_result':'raw-case-results/chough-pr46-combined-TC-cli-skill-orders-the-change-check-before-new-requirement-cases.json','original_wrapper':'chough-pr46-run-final-cases-v2.py','product_or_expected_outcome_changes':False},indent=2)+'\n')
(dest/'finish-qualification-records.py').write_bytes(pathlib.Path('/tmp/chough-pr46-finish-qualification-records.py').read_bytes())
ids=[x['observation'] for x in index]
for name in ['verify','msrv']:
 d=json.loads((dest/(name+'-result.json')).read_text());assert d['exit_code']==0 and d['observed_failed_tests']==0 and d['observed_ignored_tests']==0
 if name=='verify':assert d['lane_completed']
 count=d['observed_passed_tests'];summary=(f'Final combined37/44/46 qualification: all9Fledge steps and all{count}tests passed on Rust1.95. Actual44main is an exact whole-tree match to the premerged archived44tip; qualified source/test/skill/case identities remain unchanged, with0transition replay. Original protected proof is exact.' if name=='verify' else f'Final combined37/44/46 MSRV qualification: all{count}tests passed on Rust1.88.0 in a separate target. Actual44main import preserves the exact qualified source/test/skill/case identities. Version-specific fixture binaries are identified separately.')
 args=[cli,'evidence','add','--criterion','LIFE-3','--criterion','LIFE-6','--criterion','LIFE-7','--criterion','LIFE-8','--criterion','FLOW-6','--requirement','REQ-cli-025','--requirement','REQ-cli-026','--requirement','REQ-cli-027','--requirement','REQ-cli-028','--requirement','REQ-cli-029','--contract','specs/cli/cli.spec.md','--kind','product-test','--outcome','passed','--environment',d['environment'],'--summary',summary,'--artifact',str(pathlib.Path('.chough/artifacts/pr46-combined-qualification')/(name+'.log')),'--artifact',str(pathlib.Path('.chough/artifacts/pr46-combined-qualification')/(name+'-result.json')),'--artifact','.chough/artifacts/pr46-combined-qualification/binary-identities.json','--artifact','.chough/artifacts/pr46-actual44-main-import/equivalence-and-preservation.json','--method','automated','--limitation','Local macOS qualification. Hosted checks at the final PR46tip, human implementation acceptance and native review/ship are separate. No external-agent cold-start run, release, deployment, live adoption or cleanup.']
 r=subprocess.run(args,env=env,capture_output=True,timeout=120);pathlib.Path('/tmp/chough-pr46-combined-'+name+'-observation-result.json').write_bytes(r.stdout);assert r.returncode==0,r.stderr.decode();result=json.loads(r.stdout);ident=pathlib.Path(result['recorded']).stem;ids.append(ident);print(name,ident,flush=True)
assert len(ids)==18 and len(set(ids))==18
pathlib.Path('/tmp/chough-pr46-combined-final-observation-ids.json').write_text(json.dumps(ids,indent=2)+'\n');(dest/'final-observation-ids.json').write_text(json.dumps(ids,indent=2)+'\n')
args=[cli,'checkpoint','--completed','Combined source:9Fledge steps and240tests on each Rust1.95/1.88, all16cases passed; actual44main imported with identical qualified inputs and0record replay.','--next-action','Root records authorized native review/ship and done, then pushes and observes current CI before merge.']
for ident in ids:args+=['--observation',ident]
r=subprocess.run(args,env=env,capture_output=True,timeout=120);pathlib.Path('/tmp/chough-pr46-combined-qualified-checkpoint.json').write_bytes(r.stdout);assert r.returncode==0,r.stderr.decode()
p=root/'.chough/session.json';s=json.loads(p.read_text());assert len(s['observations'])==196 and len(p.read_bytes())<=65536
policy=json.loads((root/'.chough/artifacts/pr46-checkpoint-reference-index/policy.json').read_text())
for retired in policy['retired_from_active_index']:
 assert retired['record'] in s['references']['files'] and retired['id'] not in s['observations']
 assert hashlib.sha256((root/retired['record']).read_bytes()).hexdigest()==retired['sha256']
 for file,digest in retired['artifacts'].items():assert hashlib.sha256((root/file).read_bytes()).hexdigest()==digest
print(json.dumps({'final_observations':len(ids),'active_observations':len(s['observations']),'checkpoint_bytes':len(p.read_bytes()),'record_pin_count':24,'file_reference_count':len(s['references']['files'])}),flush=True)
