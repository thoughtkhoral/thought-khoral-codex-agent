#!/usr/bin/env python3
# SPDX-License-Identifier: Apache-2.0
"""Offline tests: real admission/evidence/cleanup logic, fake external Docker only."""
import copy
import hashlib
import importlib.util
import json
import os
from pathlib import Path
import shutil
import subprocess
import tempfile
import unittest
from unittest.mock import patch

ROOT = Path(__file__).resolve().parents[1]
ASSETS = Path('contracts/codex-app-server-0.160.0')
MARKERS = ('tool policy verified; exposed tools: []\n'
           'worker package verified; codex-cli 0.160.0; uid/gid 10003; no inference\n')


class DockerDouble:
    """Only external Docker is replaced; files, git and validators stay real."""
    def __init__(self, root, arch):
        self.root, self.arch = root, arch
        self.host_arch = 'x86_64' if arch == 'amd64' else 'aarch64'
        self.engine_arch = self.host_arch
        self.image_arch = arch
        self.cli = b'synthetic executable for offline tests only'
        self.proof = (root / ASSETS / f'tool-policy-proof-{self.host_arch}.json').read_bytes()
        self.containers, self.tags = {}, set()
        self.fail_at = None
        self.bad_markers = False
        self.accept_tamper = False
        self.cleanup_fails = False
        self.builds = 0
        self.started = 0
        self.tamper_probe = 0
        self.tamper_outcome = None
        self.bad_state = None
        self.states = {}

    def __call__(self, argv, cwd=None, timeout=120):
        if argv[0] != 'docker':
            return subprocess.run(argv, cwd=cwd, text=True, capture_output=True, timeout=timeout)
        args = argv[1:]
        out, code = '', 0
        if args[0] == 'info':
            out = json.dumps({'OSType': 'linux', 'Architecture': self.engine_arch,
                              'ServerVersion': 'offline-double'})
        elif args[0] == 'version':
            out = json.dumps({'Client': {'Version': 'offline-double'},
                              'Server': {'Version': 'offline-double'}})
        elif args[0] == 'build':
            self.builds += 1
            self.tags.add(args[args.index('--tag') + 1])
            Path(args[args.index('--iidfile') + 1]).write_text('sha256:' + str(self.builds) * 64)
        elif args[:2] == ['image', 'inspect']:
            out = json.dumps([{'Id': args[-1], 'Os': 'linux', 'Architecture': self.image_arch,
                               'Config': {'User': '10003:10003'}}])
        elif args[0] == 'inspect':
            out = json.dumps(self.states[args[-1]])
        elif args[0] in ('run', 'create'):
            name = args[args.index('--name') + 1]
            label = args[args.index('--label') + 1]
            self.containers[name] = label
            if self.fail_at == 'allocation':
                raise subprocess.TimeoutExpired(argv, timeout)
            if args[0] == 'create':
                out = 'created-id'
            else:
                self.started += 1
                # Catch a broken isolation boundary rather than source spelling.
                for flag, value in [('--network', 'none'), ('--user', '10003:10003'),
                                    ('--cap-drop', 'ALL')]:
                    if args[args.index(flag) + 1] != value:
                        raise RuntimeError('unsafe container boundary')
                if '--read-only' not in args:
                    raise RuntimeError('writable container root')
                mounts = [args[i + 1] for i, value in enumerate(args) if value == '--volume']
                if '--inside' in args:
                    mount = next(m for m in mounts if m.endswith(':/result:rw'))
                    Path(mount.removesuffix(':/result:rw'), 'proof.json').write_bytes(self.proof)
                elif any(':/opt/thought-khoral-codex/' in m for m in mounts):
                    out, code = (MARKERS, 0) if self.accept_tamper else ('runtime_unavailable\n', 1)
                    if name.endswith('tamper-' + str(self.tamper_probe)) and self.tamper_outcome:
                        code, out = self.tamper_outcome
                else:
                    out = 'only one marker\n' if self.bad_markers else MARKERS
                self.states[name] = {'Status': 'exited', 'Running': False, 'OOMKilled': False,
                                     'Error': '', 'ExitCode': code,
                                     'StartedAt': '2026-11-19T12:00:00.000000000Z'}
                if self.bad_state:
                    self.states[name].update(self.bad_state)
        elif args[0] == 'cp':
            Path(args[-1]).write_bytes(self.cli)
        elif args[0] == 'rm':
            if self.cleanup_fails:
                code = 1
            else:
                for name in args[2:]:
                    self.containers.pop(name, None)
        elif args[0] == 'ps':
            label = args[args.index('--filter') + 1].removeprefix('label=')
            out = '\n'.join(name for name, value in self.containers.items() if value == label)
        elif args[:2] == ['image', 'rm']:
            self.tags.discard(args[-1])
        else:
            raise AssertionError(f'unexpected Docker boundary: {args}')
        if self.fail_at == args[0]:
            code = 1
        return subprocess.CompletedProcess(argv, code, out, '')


class LinuxAcceptance(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        path = ROOT / 'scripts/verify-linux-package.py'
        if not path.is_file():
            raise unittest.SkipTest('helper implementation pending')
        spec = importlib.util.spec_from_file_location('linux_acceptance', path)
        cls.helper = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(cls.helper)

    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory(prefix='linux-offline-')
        self.addCleanup(self.tmp.cleanup)
        base = Path(self.tmp.name)
        self.root, self.output = base / 'repo', base / 'evidence'
        self.root.mkdir()
        for relative in ['Cargo.lock', 'Containerfile', 'docs/dependency-evidence.json',
                         *(str(p.relative_to(ROOT)) for p in (ROOT / ASSETS).glob('*.json'))]:
            destination = self.root / relative
            destination.parent.mkdir(parents=True, exist_ok=True)
            shutil.copyfile(ROOT / relative, destination)
        # Synthetic fixture CLI hash is reviewed independently from helper logic.
        digest = hashlib.sha256(b'synthetic executable for offline tests only').hexdigest()
        for path in (self.root / ASSETS).glob('tool-policy-proof-*.json'):
            proof = json.loads(path.read_text())
            proof['cliSha256'] = digest
            path.write_text(json.dumps(proof, sort_keys=True, indent=2) + '\n')
        pins = self.root / 'scripts/fixtures/linux-acceptance-pins.json'
        pins.parent.mkdir(parents=True)
        pins.write_text(json.dumps({'pythonIndex': 'docker.io/library/python@sha256:' + 'a' * 64,
                                   'pythonManifests': {'amd64': 'sha256:' + 'b' * 64,
                                                       'arm64': 'sha256:' + 'c' * 64}}))
        for args in [['init', '--quiet'], ['add', '.'],
                     ['-c', 'user.name=Offline test', '-c', 'user.email=offline@example.invalid',
                      'commit', '--quiet', '-m', 'synthetic fixture']]:
            subprocess.run(['git', *args], cwd=self.root, check=True, capture_output=True)
        self.commit = subprocess.check_output(['git', 'rev-parse', 'HEAD'], cwd=self.root, text=True).strip()

    def invoke(self, engine, commit=None, machine=None):
        machine = machine or engine.host_arch
        with patch.object(self.helper.platform, 'system', return_value='Linux'), \
             patch.object(self.helper.platform, 'machine', return_value=machine), \
             patch.dict(os.environ, {'RUNNER_ARCH': 'X64' if engine.arch == 'amd64' else 'ARM64'}):
            result = self.helper.verify_package(self.root, self.output, engine.arch,
                                                commit or self.commit, run=engine)
        manifest = json.loads((self.output / 'manifest.json').read_text())
        return result, manifest

    def test_both_native_targets_emit_complete_evidence_and_remove_owned_resources(self):
        for arch in ('amd64', 'arm64'):
            with self.subTest(arch=arch):
                self.output = Path(self.tmp.name) / arch
                engine = DockerDouble(self.root, arch)
                # Unrelated containers must survive cleanup.
                engine.containers['unrelated'] = 'unrelated=owner'
                result, manifest = self.invoke(engine)
                self.assertEqual(result, 0, manifest)
                self.assertEqual(manifest['status'], 'passed')
                self.assertEqual(manifest['source']['commit'], self.commit)
                self.assertEqual(manifest['proof']['cases'], 44)
                self.assertEqual(manifest['proof']['models'], 8)
                self.assertEqual(manifest['proof']['refusals'], 6)
                self.assertTrue(manifest['cleanup']['passed'])
                self.assertEqual(engine.containers, {'unrelated': 'unrelated=owner'})
                self.assertEqual(engine.tags, set())
                self.assertEqual((self.output / 'proof.json').read_bytes(), engine.proof)

    def test_wrong_native_architecture_cannot_allocate_containers(self):
        for kind in ('host', 'engine', 'image'):
            with self.subTest(kind=kind):
                self.output = Path(self.tmp.name) / kind
                engine = DockerDouble(self.root, 'amd64')
                if kind == 'engine':
                    engine.engine_arch = 'aarch64'
                if kind == 'image':
                    engine.image_arch = 'arm64'
                result, manifest = self.invoke(engine, machine='aarch64' if kind == 'host' else None)
                self.assertEqual(result, 1)
                self.assertEqual(manifest['status'], 'failed')
                self.assertEqual(engine.started, 0)
                self.assertFalse(engine.containers)

    def test_wrong_or_dirty_source_is_rejected_before_build(self):
        for dirty in (False, True):
            with self.subTest(dirty=dirty):
                self.output = Path(self.tmp.name) / str(dirty)
                if dirty:
                    (self.root / 'unexpected.txt').write_text('dirty')
                engine = DockerDouble(self.root, 'amd64')
                result, manifest = self.invoke(engine, commit=self.commit if dirty else 'f' * 40)
                self.assertEqual(result, 1)
                self.assertEqual(manifest['status'], 'failed')
                self.assertEqual(engine.builds, 0)

    def test_stale_reviewed_lock_is_rejected_even_in_clean_commit(self):
        inventory = self.root / 'docs/dependency-evidence.json'
        data = json.loads(inventory.read_text())
        data['cargoLockSha256'] = '0' * 64
        inventory.write_text(json.dumps(data))
        subprocess.run(['git', 'add', '.'], cwd=self.root, check=True)
        subprocess.run(['git', '-c', 'user.name=Offline', '-c', 'user.email=offline@example.invalid',
                        'commit', '--quiet', '-m', 'stale inventory'], cwd=self.root, check=True)
        self.commit = subprocess.check_output(['git', 'rev-parse', 'HEAD'], cwd=self.root, text=True).strip()
        engine = DockerDouble(self.root, 'amd64')
        result, manifest = self.invoke(engine)
        self.assertEqual(result, 1)
        self.assertIn('lock', manifest['error'].lower())
        self.assertEqual(engine.builds, 0)

    def test_verifier_markers_and_tamper_refusal_are_required(self):
        for kind in ('bad_markers', 'accept_tamper'):
            with self.subTest(kind=kind):
                self.output = Path(self.tmp.name) / kind
                engine = DockerDouble(self.root, 'amd64')
                setattr(engine, kind, True)
                result, manifest = self.invoke(engine)
                self.assertEqual(result, 1)
                self.assertEqual(manifest['status'], 'failed')
                self.assertFalse(engine.containers)

    def test_every_tamper_probe_rejects_launch_kill_and_unrelated_failures(self):
        outcomes = [(125, 'runtime_unavailable\n'), (126, 'runtime_unavailable\n'),
                    (127, 'runtime_unavailable\n'), (137, 'runtime_unavailable\n'),
                    (1, 'forbidden\n'), (1, 'Docker launch failed\n')]
        for probe in range(3):
            for number, outcome in enumerate(outcomes):
                with self.subTest(probe=probe, outcome=outcome):
                    self.output = Path(self.tmp.name) / f'probe-{probe}-{number}'
                    engine = DockerDouble(self.root, 'amd64')
                    engine.tamper_probe, engine.tamper_outcome = probe, outcome
                    code, manifest = self.invoke(engine)
                    self.assertEqual(code, 1)
                    self.assertEqual(manifest['status'], 'failed')
                    self.assertNotIn('tamper', manifest['outcomes'])
                    self.assertFalse(engine.containers)

    def test_engine_state_must_prove_successful_process_launch(self):
        for number, state in enumerate([{'OOMKilled': True}, {'Error': 'launch failure'},
                                        {'Status': 'created'}, {'Running': True},
                                        {'ExitCode': 125}, {'StartedAt': '0001-01-01T00:00:00Z'}]):
            with self.subTest(state=state):
                self.output = Path(self.tmp.name) / str(number)
                engine = DockerDouble(self.root, 'amd64')
                engine.bad_state = state
                code, manifest = self.invoke(engine)
                self.assertEqual(code, 1)
                self.assertEqual(manifest['status'], 'failed')
                self.assertFalse(engine.containers)

    def test_wrong_extracted_executable_fails_before_capture(self):
        engine = DockerDouble(self.root, 'amd64')
        engine.cli = b'wrong executable'
        result, manifest = self.invoke(engine)
        self.assertEqual(result, 1)
        self.assertEqual(engine.builds, 1)
        self.assertIn('CLI', manifest['error'])
        self.assertFalse(engine.containers)

    def test_partial_changed_and_semantically_incomplete_proofs_fail(self):
        engine = DockerDouble(self.root, 'amd64')
        original = json.loads(engine.proof)
        for kind in ('partial', 'changed', 'duplicate', 'tools', 'resume', 'refusal'):
            with self.subTest(kind=kind):
                self.output = Path(self.tmp.name) / kind
                data = copy.deepcopy(original)
                if kind == 'duplicate':
                    data['cases'][-1] = data['cases'][0]
                if kind == 'tools':
                    data['cases'][0]['tools'] = ['shell']
                if kind == 'resume':
                    data.pop('resume')
                if kind == 'refusal':
                    data['toolCallRejections'].pop()
                engine = DockerDouble(self.root, 'amd64')
                engine.proof = b'{' if kind == 'partial' else (json.dumps(data).encode() if kind == 'changed'
                               else (json.dumps(data, sort_keys=True, indent=2) + '\n').encode())
                result, manifest = self.invoke(engine)
                self.assertEqual(result, 1)
                self.assertEqual(manifest['status'], 'failed')
                self.assertFalse(engine.containers)
                if kind not in ('partial', 'changed'):
                    # Even a changed committed proof cannot weaken semantic coverage.
                    with self.assertRaises(self.helper.AcceptanceError):
                        self.helper.validate_proof(engine.proof, engine.proof)

    def test_failures_and_partial_allocation_leave_no_passed_manifest(self):
        for stage in ('build', 'run', 'cp', 'allocation'):
            with self.subTest(stage=stage):
                self.output = Path(self.tmp.name) / stage
                engine = DockerDouble(self.root, 'amd64')
                engine.fail_at = stage
                result, manifest = self.invoke(engine)
                self.assertEqual(result, 1)
                self.assertEqual(manifest['status'], 'failed')
                self.assertTrue(manifest['cleanup']['passed'])
                self.assertFalse(engine.containers)

    def test_failed_cleanup_cannot_pass_and_retry_removes_only_owned_resources(self):
        engine = DockerDouble(self.root, 'amd64')
        engine.cleanup_fails = True
        result, manifest = self.invoke(engine)
        self.assertEqual(result, 1)
        self.assertEqual(manifest['status'], 'failed')
        self.assertFalse(manifest['cleanup']['passed'])
        engine.cleanup_fails = False
        self.assertEqual(self.helper.cleanup_output(self.output, run=engine), 0)
        self.assertFalse(engine.containers)

    def test_output_inside_checkout_is_refused_without_modifying_source(self):
        self.output = self.root / 'evidence'
        engine = DockerDouble(self.root, 'amd64')
        with self.assertRaises(self.helper.AcceptanceError):
            self.invoke(engine)
        self.assertFalse(self.output.exists())
        self.assertEqual(engine.builds, 0)

    def test_runtime_versions_native_target_and_test_failure_are_required(self):
        for kind in ('passed', 'old-rust', 'old-cargo', 'cross-host', 'cross-env', 'failed-tests'):
            with self.subTest(kind=kind):
                self.output = Path(self.tmp.name) / kind
                def boundary(argv, cwd=None, timeout=120):
                    if argv == ['rustc', '-vV']:
                        version = '1.84.0' if kind == 'old-rust' else '1.85.0'
                        host = 'aarch64' if kind == 'cross-host' else 'x86_64'
                        out = f'rustc {version}\nrelease: {version}\nhost: {host}-unknown-linux-gnu\n'
                    elif argv == ['cargo', '--version']:
                        out = 'cargo 1.84.0' if kind == 'old-cargo' else 'cargo 1.85.0 (synthetic)'
                    elif argv[:2] == ['cargo', 'test']:
                        return subprocess.CompletedProcess(argv, int(kind == 'failed-tests'),
                                                           'synthetic offline test boundary\n', '')
                    else:
                        return subprocess.run(argv, cwd=cwd, text=True, capture_output=True, timeout=timeout)
                    return subprocess.CompletedProcess(argv, 0, out, '')
                with patch.object(self.helper.platform, 'system', return_value='Linux'), \
                     patch.object(self.helper.platform, 'machine', return_value='x86_64'), \
                     patch.dict(os.environ, {'RUNNER_ARCH': 'X64',
                                             'CARGO_BUILD_TARGET': 'other' if kind == 'cross-env' else ''}):
                    code = self.helper.verify_runtime(self.root, self.output, 'amd64', self.commit,
                                                       '1.85.0', run=boundary)
                manifest = json.loads((self.output / 'manifest.json').read_text())
                self.assertEqual(code, 0 if kind == 'passed' else 1)
                self.assertEqual(manifest['status'], 'passed' if kind == 'passed' else 'failed')

    def test_unrecognised_matrix_rust_version_is_refused(self):
        code = self.helper.verify_runtime(self.root, self.output, 'amd64', self.commit, '1.84.0')
        self.assertEqual(code, 1)

    def test_cleanup_retry_revokes_a_previously_passed_manifest_if_it_fails(self):
        engine = DockerDouble(self.root, 'amd64')
        code, manifest = self.invoke(engine)
        self.assertEqual(code, 0, manifest)
        data = json.loads((self.output / 'resources.json').read_text())
        engine.containers['tk-linux-' + data['token'] + '-late'] = \
            'org.thoughtkhoral.linux-acceptance=' + data['token']
        engine.cleanup_fails = True
        self.assertEqual(self.helper.cleanup_output(self.output, run=engine), 1)
        self.assertEqual(json.loads((self.output / 'manifest.json').read_text())['status'], 'failed')


if __name__ == '__main__':
    unittest.main()
