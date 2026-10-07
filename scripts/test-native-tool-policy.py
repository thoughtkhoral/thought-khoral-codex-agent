#!/usr/bin/env python3
# SPDX-License-Identifier: Apache-2.0
"""Actual pinned CLI request capture, only isolated loopback synthetic provider.

Default runs inside a dedicated Task8 container with --network none. Never points
production worker configuration at the capture provider. --inside is container-only.
"""
import hashlib,http.server,json,os,pathlib,queue,subprocess,sys,tempfile,threading,time
ROOT=pathlib.Path(__file__).resolve().parents[1]
ASSETS=ROOT/'contracts/codex-app-server-0.160.0'

def toml(v):
 if isinstance(v,dict):return '{'+','.join(json.dumps(k)+'='+toml(x) for k,x in v.items())+'}'
 if isinstance(v,list):return '['+','.join(map(toml,v))+']'
 return json.dumps(v)

def wire_effort(descriptor,effort):
 if effort=='persistent':return 'disabled'
 if effort!='ultra':return effort
 levels=[x['effort'] for x in descriptor['supported_reasoning_levels']]
 override=descriptor.get('multi_agent_reasoning_effort')
 if override!='ultra' and override in levels:return override
 if 'max' in levels:return 'max'
 return next((x for x in reversed(levels) if x!='ultra'),'medium')

class Capture(http.server.BaseHTTPRequestHandler):
 requests=[]
 inject=None
 injected=False
 def log_message(self,*args):pass
 def do_POST(self):
  raw=self.rfile.read(int(self.headers['Content-Length']))
  if self.headers.get('Content-Encoding'): raise AssertionError('compressed request cannot be inspected')
  data=json.loads(raw);self.requests.append(data)
  message={'id':'msg_synthetic','type':'message','role':'assistant','status':'completed','content':[{'type':'output_text','text':'Synthetic tool-policy proof.','annotations':[]}]}
  response={'id':'resp_synthetic','object':'response','status':'completed','output':[message],'usage':{'input_tokens':1,'output_tokens':1,'total_tokens':2}}
  if self.inject and not type(self).injected:
   type(self).injected=True
   message={'id':'fc_synthetic','type':'function_call','call_id':'call_synthetic','name':self.inject,'arguments':json.dumps({'cmd':'touch /tmp/task8-tool-canary','command':['touch','/tmp/task8-tool-canary'],'patch':'*** Begin Patch\n*** Add File: /tmp/task8-tool-canary\n+unsafe\n*** End Patch','code':'throw new Error(\"task8-tool-canary\")'})}
   response['output']=[message]
  events=[{'type':'response.created','response':{**response,'status':'in_progress','output':[]}}, {'type':'response.output_item.added','output_index':0,'item':{**message,'status':'in_progress','content':[]}}, {'type':'response.output_text.delta','output_index':0,'content_index':0,'item_id':'msg_synthetic','delta':'Synthetic tool-policy proof.'}, {'type':'response.output_item.done','output_index':0,'item':message}, {'type':'response.completed','response':response}]
  body=''.join('event: '+e['type']+'\ndata: '+json.dumps(e)+'\n\n' for e in events).encode()
  self.send_response(200);self.send_header('Content-Type','text/event-stream');self.send_header('Content-Length',str(len(body)));self.end_headers();self.wfile.write(body)

def assert_no_tools(value):
 if isinstance(value,dict):
  for key,child in value.items():
   if key=='tools':assert child==[],('nonempty tool declaration',child)
   assert_no_tools(child)
 elif isinstance(value,list):
  for child in value:assert_no_tools(child)

class Native:
 def __init__(self,cli,controls,home,cwd,port):
  overrides=controls|{'model_provider':'task8_capture','model_providers':{'task8_capture':{'name':'Task8 synthetic capture','base_url':f'http://127.0.0.1:{port}/v1','wire_api':'responses','requires_openai_auth':False,'supports_websockets':False,'request_max_retries':0,'stream_max_retries':0}},'model_catalog_json':str(ASSETS/'restricted-models.json')}
  overrides['features']=overrides['features']|{'enable_request_compression':False,'responses_websockets':False,'responses_websockets_v2':False}
  args=[cli,'app-server','--listen','stdio://']
  for k,v in overrides.items():args+=['-c',k+'='+toml(v)]
  self.stderr=tempfile.TemporaryFile()
  self.p=subprocess.Popen(args,stdin=subprocess.PIPE,stdout=subprocess.PIPE,stderr=self.stderr,text=True,cwd=cwd,env={'HOME':str(home),'CODEX_HOME':str(home),'PATH':'/usr/bin:/bin','LANG':'C.UTF-8'})
  self.q=queue.Queue();self.next_id=1;self.pending=[]
  def read():
   for line in self.p.stdout:
    try:self.q.put(json.loads(line))
    except Exception:self.q.put({'badLine':line[:80]})
  threading.Thread(target=read,daemon=True).start()
  self.rpc('initialize',{'clientInfo':{'name':'task8-tool-proof','version':'1'},'capabilities':{'experimentalApi':False}})
  self.send({'method':'initialized'})
 def send(self,v):self.p.stdin.write(json.dumps(v)+'\n');self.p.stdin.flush()
 def event(self):
  try:return self.q.get(timeout=20)
  except queue.Empty:
   self.stderr.seek(0);raise AssertionError('native response timeout: '+self.stderr.read()[-1800:].decode(errors='replace'))
 def rpc(self,method,params):
  id=self.next_id;self.next_id+=1;self.send({'id':id,'method':method,'params':params})
  while True:
   v=self.event()
   if v.get('id')==id:
    assert 'error' not in v,(method,v);return v['result']
   self.pending.append(v)
 def close(self):
  self.p.terminate()
  try:self.p.wait(timeout=3)
  except subprocess.TimeoutExpired:self.p.kill();self.p.wait()
  self.stderr.close()


def inside(output):
 cli='/usr/local/bin/codex';assert subprocess.check_output([cli,'--version'],text=True).strip()=='codex-cli 0.160.0'
 controls=json.loads((ASSETS/'tool-controls.json').read_text())
 with tempfile.TemporaryDirectory(prefix='task8-native-') as tmp:
  tmp=pathlib.Path(tmp);home=tmp/'native';cwd=tmp/'workspace';home.mkdir();cwd.mkdir()
  server=http.server.ThreadingHTTPServer(('127.0.0.1',0),Capture);threading.Thread(target=server.serve_forever,daemon=True).start()
  native=Native(cli,controls,home,cwd,server.server_port)
  results=[];first_thread=None;first_selection=None
  try:
   models=native.rpc('model/list',{'limit':100,'includeHidden':False})
   assert models['nextCursor'] is None
   descriptors={m['slug']:m for m in json.loads((ASSETS/'restricted-models.json').read_text())['models']}
   reviewed=set(descriptors)
   for m in models['data']:
    assert m['model'] in reviewed
    for option in m['supportedReasoningEfforts']:
     effort=option['reasoningEffort'];before=len(Capture.requests)
     params={'model':m['model'],'approvalPolicy':'never','sandbox':'read-only','cwd':str(cwd),'config':{'model_reasoning_effort':effort},'baseInstructions':'Reply with synthetic text only. No tools.','ephemeral':False}
     started=native.rpc('thread/start',params);thread=started['thread']['id']
     assert started['model']==m['model'];assert started['reasoningEffort']==effort
     native.rpc('turn/start',{'threadId':thread,'model':m['model'],'effort':effort,'approvalPolicy':'never','sandboxPolicy':{'type':'readOnly','networkAccess':False},'cwd':str(cwd),'input':[{'type':'text','text':'Synthetic task8 proof; reply OK.','text_elements':[]}]})
     while True:
      event=native.event()
      assert 'id' not in event,('unexpected client request',event)
      if event.get('method')=='turn/completed':
       assert event['params']['turn']['status']=='completed',event;break
     assert len(Capture.requests)==before+1,(m['model'],Capture.requests[before:])
     payload=Capture.requests[-1]
     assert_no_tools(payload)
     assert payload['model']==m['model']
     # Pinned protocol/openai_models/reasoning_effort.rs resolves Ultra using
     # multi_agent_reasoning_effort; native app-server setting remains Ultra.
     expected_wire=wire_effort(descriptors[m['model']],effort)
     assert payload.get('reasoning',{}).get('effort')==expected_wire,payload
     results.append({'model':m['model'],'effort':effort,'wireEffort':expected_wire,'tools':[]})
     if first_thread is None:first_thread=thread;first_selection=(m['model'],effort)
     print('captured',m['model'],effort,'tools=[]',flush=True)
   native.close();native=Native(cli,controls,home,cwd,server.server_port)
   model,effort=first_selection
   native.rpc('thread/resume',{'threadId':first_thread,'model':model,'approvalPolicy':'never','sandbox':'read-only','cwd':str(cwd),'config':{'model_reasoning_effort':effort},'excludeTurns':True})
   before=len(Capture.requests)
   native.rpc('turn/start',{'threadId':first_thread,'model':model,'effort':effort,'approvalPolicy':'never','sandboxPolicy':{'type':'readOnly','networkAccess':False},'cwd':str(cwd),'input':[{'type':'text','text':'Synthetic resume proof.','text_elements':[]}]})
   while True:
    event=native.event()
    if event.get('method')=='turn/completed':assert event['params']['turn']['status']=='completed';break
   assert len(Capture.requests)==before+1
   assert_no_tools(Capture.requests[-1])
   rejected=[]
   for tool in ['apply_patch','exec_command','shell','exec','client_tool','mcp__task8__canary']:
    Capture.inject=tool;Capture.injected=False;before=len(Capture.requests)
    native.rpc('turn/start',{'threadId':first_thread,'model':model,'effort':effort,'approvalPolicy':'never','sandboxPolicy':{'type':'readOnly','networkAccess':False},'cwd':str(cwd),'input':[{'type':'text','text':'Synthetic invalid tool response probe.','text_elements':[]}]})
    while True:
     event=native.event()
     assert 'id' not in event,('unexpected client tool request',event)
     if event.get('method')=='turn/completed':break
    requests=Capture.requests[before:]
    assert len(requests)>=2,('native did not reject unknown tool with response',tool)
    for payload in requests:assert_no_tools(payload)
    errors=[]
    def collect_tool_outputs(value):
     if isinstance(value,dict):
      if value.get('call_id')=='call_synthetic' and 'output' in value:errors.append(str(value['output']))
      for child in value.values():collect_tool_outputs(child)
     elif isinstance(value,list):
      for child in value:collect_tool_outputs(child)
    collect_tool_outputs(requests[-1])
    assert errors,('missing native rejection output',tool,requests[-1])
    rejected_text=errors[-1].lower()
    assert ('unknown tool' in rejected_text or 'unrecognized' in rejected_text or 'unsupported' in rejected_text) and tool in rejected_text,(tool,errors[-1])
    assert not pathlib.Path('/tmp/task8-tool-canary').exists()
    rejected.append(tool)
    print('rejected unsolicited tool',tool,flush=True)
   Capture.inject=None
   proof={'toolCallRejections':rejected,'cliVersion':'0.160.0','cliSha256':hashlib.sha256(pathlib.Path(cli).read_bytes()).hexdigest(),'catalogSha256':hashlib.sha256((ASSETS/'restricted-models.json').read_bytes()).hexdigest(),'controlsSha256':hashlib.sha256((ASSETS/'tool-controls.json').read_bytes()).hexdigest(),'cases':results,'resume':{'model':model,'effort':effort,'tools':[]},'providerInference':False}
   pathlib.Path(output).write_text(json.dumps(proof,sort_keys=True,indent=2)+'\n')
  finally:native.close();server.shutdown()

def container(output):
 prefix=f'task8-tool-proof-{os.getpid()}'
 actual=subprocess.check_output(['podman','image','inspect','--format','{{.Id}}','localhost/thought-khoral-codex-agent:task6-final'],text=True).strip().removeprefix('sha256:')
 assert actual=='95145520f4249ac1e843c0f13a817e0c42cfc578735abef5fa7b760333596a1b','capture CLI source image drift'
 with tempfile.TemporaryDirectory(prefix='task8-tools-') as tmp:
  tmp=pathlib.Path(tmp);tmp.chmod(0o777);(tmp/'Containerfile').write_text('FROM localhost/thought-khoral-codex-agent:task6-final AS cli\nFROM localhost/thought-khoral-codex-proxy:task8-fixture\nCOPY --from=cli /usr/local/bin/codex /usr/local/bin/codex\nUSER 10003:10003\n')
  subprocess.run(['podman','build','--quiet','-t','localhost/'+prefix,'-f',str(tmp/'Containerfile'),str(tmp)],check=True)
  try:
   subprocess.run(['podman','run','--rm','--name',prefix,'--network','none','--cap-drop','ALL','--read-only','--tmpfs','/tmp','-v',str(ROOT)+':/proof:ro','-v',str(tmp)+':/result','--entrypoint','python3','localhost/'+prefix,'/proof/scripts/test-native-tool-policy.py','--inside','/result/proof.json'],check=True)
   pathlib.Path(output).write_bytes((tmp/'proof.json').read_bytes())
  finally:subprocess.run(['podman','rm','-f',prefix],capture_output=True)

if __name__=='__main__':
 if len(sys.argv)==3 and sys.argv[1]=='--inside':inside(sys.argv[2])
 else:container(sys.argv[1] if len(sys.argv)>1 else '/tmp/task8-tool-proof.json')
