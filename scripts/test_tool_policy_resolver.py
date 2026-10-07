#!/usr/bin/env python3
import importlib.util,pathlib,unittest
path=pathlib.Path(__file__).with_name('test-native-tool-policy.py')
spec=importlib.util.spec_from_file_location('capture',path);capture=importlib.util.module_from_spec(spec);spec.loader.exec_module(capture)
class Resolver(unittest.TestCase):
 def test_pinned_ultra_resolution_uses_valid_override_then_max_then_last_then_medium(self):
  for override,levels,want in [('xhigh',['low','xhigh','max','ultra'],'xhigh'),(None,['low','max','ultra'],'max'),('invalid',['low','high','ultra'],'high'),('ultra',['ultra'],'medium')]:
   self.assertTrue(hasattr(capture,'wire_effort'),'pinned wire resolver missing')
   self.assertEqual(capture.wire_effort({'multi_agent_reasoning_effort':override,'supported_reasoning_levels':[{'effort':v} for v in levels]},'ultra'),want)
 def test_other_native_efforts_retain_values_except_persistent_wire_alias(self):
  self.assertTrue(hasattr(capture,'wire_effort'),'pinned wire resolver missing')
  self.assertEqual(capture.wire_effort({},'high'),'high')
  self.assertEqual(capture.wire_effort({},'persistent'),'disabled')
if __name__=='__main__':unittest.main()
