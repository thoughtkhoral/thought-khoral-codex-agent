#!/usr/bin/env python3
# SPDX-License-Identifier: Apache-2.0
"""Native Linux acceptance. Governing How: .ai/specs/how/linux-worker-acceptance.md.

Docker is CI-only. Offline tests replace the external command boundary; they do
not count as native execution. Output must be empty and outside the checkout.
"""
import argparse
import hashlib
import json
import os
from pathlib import Path
import platform
import re
import signal
import subprocess
import tempfile
import uuid

ROOT = Path(__file__).resolve().parents[1]
ASSETS = Path('contracts/codex-app-server-0.160.0')
TARGETS = {'amd64': ('x86_64', 'X64'), 'arm64': ('aarch64', 'ARM64')}
MARKERS = ('tool policy verified; exposed tools: []',
           'worker package verified; codex-cli 0.160.0; uid/gid 10003; no inference')
LABEL = 'org.thoughtkhoral.linux-acceptance'
REQUIRED = {'source', 'native', 'worker', 'admission', 'tamper', 'cli', 'capture', 'proof', 'sourceFinal'}


class AcceptanceError(Exception):
    """A required acceptance check failed."""


def require(condition, message):
    if not condition:
        raise AcceptanceError(message)


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def execute(argv, cwd=None, timeout=120):
    # Spool large build/test output to disk; retain a bounded diagnostic tail.
    with tempfile.TemporaryFile() as log:
        result = subprocess.run(argv, cwd=cwd, stdout=log, stderr=subprocess.STDOUT, timeout=timeout)
        log.seek(0, 2)
        log.seek(max(0, log.tell() - 16384))
        output = log.read().decode(errors='replace')
    if result.returncode:
        print(output, end='', flush=True)
    return subprocess.CompletedProcess(argv, result.returncode, output, '')


def checked(run, argv, cwd=None, timeout=120):
    result = run(argv, cwd=cwd, timeout=timeout)
    require(result.returncode == 0, f'{argv[0]} {argv[1]} failed (exit {result.returncode})')
    return result.stdout.strip()


def write_json(path, data):
    temporary = path.with_suffix('.tmp')
    temporary.write_text(json.dumps(data, sort_keys=True, indent=2) + '\n')
    temporary.replace(path)


def prepare_output(root, output):
    root, output = root.resolve(), output.resolve()
    require(not output.is_relative_to(root) and not root.is_relative_to(output),
            'evidence output must be outside the checkout')
    require(not output.exists() or (output.is_dir() and not any(output.iterdir())),
            'evidence output must be empty')
    output.mkdir(parents=True, exist_ok=True)
    return root, output


def source(root, commit, run):
    require(re.fullmatch(r'[0-9a-f]{40}', commit), 'invalid event commit')
    actual = checked(run, ['git', 'rev-parse', 'HEAD'], cwd=root)
    require(actual == commit, 'checkout differs from event commit')
    require(not checked(run, ['git', 'status', '--porcelain', '--untracked-files=all'], cwd=root),
            'checkout is dirty')
    lock = digest(root / 'Cargo.lock')
    reviewed = json.loads((root / 'docs/dependency-evidence.json').read_text())
    require(lock == reviewed['cargoLockSha256'], 'Cargo.lock differs from reviewed dependency inventory')
    return {'commit': actual, 'clean': True, 'cargoLockSha256': lock,
            'containerfileSha256': digest(root / 'Containerfile')}


def native(arch):
    require(arch in TARGETS, 'unsupported target architecture')
    machine, runner = TARGETS[arch]
    require(platform.system() == 'Linux' and platform.machine() == machine,
            'host must be native Linux for requested architecture')
    require(os.environ.get('RUNNER_ARCH') == runner, 'runner architecture differs from requested native target')
    return {'platform': 'linux/' + arch, 'system': platform.system(),
            'machine': platform.machine(), 'runnerArch': runner}


def validate_proof(actual, expected):
    require(actual == expected, 'captured proof differs from committed target bytes')
    try:
        proof = json.loads(actual)
        cases = proof['cases']
        require(len(cases) == 44 and len({(c['model'], c['effort']) for c in cases}) == 44,
                'proof requires 44 unique model/effort cases')
        require(len({c['model'] for c in cases}) == 8, 'proof requires eight models')
        require(all(c['tools'] == [] for c in cases), 'proof contains tools')
        require(proof['resume']['tools'] == [] and
                (proof['resume']['model'], proof['resume']['effort']) in
                {(c['model'], c['effort']) for c in cases}, 'proof requires a valid empty-tools resume')
        require(proof['toolCallRejections'] ==
                ['apply_patch', 'exec_command', 'shell', 'exec', 'client_tool', 'mcp__task8__canary'],
                'proof requires all six unsolicited-tool refusals')
        require(proof['providerInference'] is False and proof['cliVersion'] == '0.160.0',
                'proof version/provider boundary differs')
        for key in ('cliSha256', 'controlsSha256', 'catalogSha256'):
            require(re.fullmatch(r'[0-9a-f]{64}', proof[key]), 'invalid proof digest')
    except (KeyError, TypeError, ValueError) as error:
        raise AcceptanceError('incomplete or malformed proof') from error
    return {'sha256': hashlib.sha256(actual).hexdigest(), 'cases': 44, 'models': 8, 'refusals': 6,
            'resume': True, 'providerInference': False}


class OwnedDocker:
    def __init__(self, output, run):
        self.output, self.run = output, run
        self.token = uuid.uuid4().hex
        self.prefix = 'tk-linux-' + self.token + '-'
        self.tags = []
        self.save()

    def save(self):
        write_json(self.output / 'resources.json', {'token': self.token, 'tags': self.tags})

    def command(self, args, timeout=120):
        return checked(self.run, ['docker', *args], timeout=timeout)

    def image(self, image, arch):
        require(re.fullmatch(r'sha256:[0-9a-f]{64}', image), 'build must yield an immutable image ID')
        info = json.loads(self.command(['image', 'inspect', image]))
        require(len(info) == 1 and info[0]['Id'] == image and info[0]['Os'] == 'linux'
                and info[0]['Architecture'] == arch, 'image differs from requested native platform')
        require(info[0]['Config']['User'] == '10003:10003', 'image must use packaged UID/GID 10003')
        return image

    def build(self, context, containerfile, kind, arch):
        tag = self.prefix + kind
        self.tags.append(tag)
        self.save()  # Before allocation, including failures/timeouts.
        iid = self.output / (kind + '.iid')
        self.command(['build', '--platform', 'linux/' + arch, '--label', LABEL + '=' + self.token,
                      '--iidfile', str(iid), '--tag', tag, '--file', str(containerfile), str(context)], timeout=2400)
        return self.image(iid.read_text().strip(), arch)

    def container(self, kind, image, arguments, volumes=(), entrypoint=None):
        name = self.prefix + kind
        args = ['run', '--name', name, '--label', LABEL + '=' + self.token,
                '--platform', 'linux/' + self.arch, '--network', 'none', '--read-only',
                '--cap-drop', 'ALL', '--security-opt', 'no-new-privileges', '--user', '10003:10003']
        if entrypoint:
            args += ['--tmpfs', '/tmp:rw,nosuid,nodev,mode=1777', '--entrypoint', entrypoint]
        for mount in volumes:
            args += ['--volume', mount]
        result = self.run(['docker', *args, image, *arguments], timeout=1200)
        state = json.loads(self.command(['inspect', '--format', '{{json .State}}', name]))
        require(state['Status'] == 'exited' and state['Running'] is False and
                state['OOMKilled'] is False and state['Error'] == '' and
                state['ExitCode'] == result.returncode and
                isinstance(state['StartedAt'], str) and
                re.match(r'[1-9][0-9]{3}-', state['StartedAt']),
                'container did not complete a normally launched process')
        self.command(['rm', '--force', name])
        return result


def cleanup_output(output, run=execute):
    """Retry cleanup after signals/cancellation, restricted to a validated owner."""
    output = Path(output)
    path = output / 'resources.json'
    if not path.exists():
        return 0
    try:
        data = json.loads(path.read_text())
        token, tags = data['token'], data['tags']
        require(re.fullmatch(r'[0-9a-f]{32}', token), 'invalid cleanup owner')
        prefix = 'tk-linux-' + token + '-'
        require(isinstance(tags, list) and all(t in (prefix + 'worker', prefix + 'capture') for t in tags),
                'invalid cleanup tags')
        query = ['docker', 'ps', '--all', '--filter', 'label=' + LABEL + '=' + token,
                 '--format', '{{.Names}}']
        names = checked(run, query).splitlines()
        require(all(re.fullmatch(re.escape(prefix) + r'[a-z0-9-]+', n) for n in names),
                'unexpected owned-container name')
        if names:
            run(['docker', 'rm', '--force', *names], timeout=120)
        require(not checked(run, query), 'owned containers remain after cleanup')
        for tag in tags:
            result = run(['docker', 'image', 'rm', tag], timeout=120)
            if result.returncode:
                require(run(['docker', 'image', 'inspect', tag], timeout=120).returncode != 0,
                        'temporary image tag remains after cleanup')
        write_json(output / 'cleanup.json', {'passed': True, 'ownedContainersRemaining': 0})
        return 0
    except (AcceptanceError, OSError, ValueError, KeyError, TypeError, subprocess.SubprocessError):
        write_json(output / 'cleanup.json', {'passed': False})
        manifest_path = output / 'manifest.json'
        if manifest_path.exists():
            manifest = json.loads(manifest_path.read_text())
            manifest.update(status='failed', cleanup={'passed': False}, error='owned-resource cleanup failed')
            write_json(manifest_path, manifest)
        return 1


def verify_package(root, output, arch, commit, run=execute):
    root, output = prepare_output(Path(root), Path(output))
    manifest = {'status': 'failed', 'kind': 'native-package', 'outcomes': {}}
    owner = None
    try:
        manifest['source'] = source(root, commit, run)
        manifest['outcomes']['source'] = True
        manifest['native'] = native(arch)
        info = json.loads(checked(run, ['docker', 'info', '--format', '{{json .}}']))
        require(info['OSType'] == 'linux' and info['Architecture'] in (arch, TARGETS[arch][0]),
                'Docker engine differs from requested native architecture')
        versions = json.loads(checked(run, ['docker', 'version', '--format', '{{json .}}']))
        manifest['docker'] = {side.lower(): versions[side]['Version'] for side in ('Client', 'Server')}
        manifest['outcomes']['native'] = True
        assets = root / ASSETS
        machine = TARGETS[arch][0]
        expected = (assets / f'tool-policy-proof-{machine}.json').read_bytes()
        validate_proof(expected, expected)
        proof = json.loads(expected)
        require(proof['catalogSha256'] == digest(assets / 'restricted-models.json') and
                proof['controlsSha256'] == digest(assets / 'tool-controls.json'), 'proof asset hashes differ')
        manifest['policy'] = {k: proof[k] for k in ('cliSha256', 'catalogSha256', 'controlsSha256')}
        owner = OwnedDocker(output, run)
        owner.arch = arch
        worker = owner.build(root, root / 'Containerfile', 'worker', arch)
        manifest['workerImage'] = worker
        manifest['outcomes']['worker'] = True
        result = owner.container('verify', worker, ['--verify-package'])
        require(result.returncode == 0 and all(m in result.stdout.splitlines() for m in MARKERS),
                'package verifier failed or missing success markers')
        manifest['outcomes']['admission'] = True
        other = 'aarch64' if arch == 'amd64' else 'x86_64'
        with tempfile.TemporaryDirectory(prefix='linux-package-') as temporary:
            tmp = Path(temporary)
            mutations = [(assets / f'tool-policy-proof-{other}.json', f'tool-policy-proof-{machine}.json')]
            for name in ('tool-controls.json', 'restricted-models.json'):
                path = tmp / name
                path.write_bytes((assets / name).read_bytes() + b'\n')
                mutations.append((path, name))
            for number, (path, name) in enumerate(mutations):
                result = owner.container('tamper-' + str(number), worker, ['--verify-package'],
                                         [str(path) + ':/opt/thought-khoral-codex/' + name + ':ro'])
                require(result.returncode == 1 and result.stdout.strip() == 'runtime_unavailable',
                        'tampered package was not refused cleanly')
            manifest['outcomes']['tamper'] = True
            extraction = owner.prefix + 'extract'
            owner.command(['create', '--name', extraction, '--label', LABEL + '=' + owner.token,
                           '--platform', 'linux/' + arch, worker])
            cli = tmp / 'codex'
            owner.command(['cp', extraction + ':/usr/local/bin/codex', str(cli)])
            owner.command(['rm', '--force', extraction])
            require(digest(cli) == proof['cliSha256'], 'extracted CLI digest differs from target proof')
            cli.chmod(0o755)
            manifest['outcomes']['cli'] = True
            pins = json.loads((root / 'scripts/fixtures/linux-acceptance-pins.json').read_text())
            require(re.fullmatch(r'docker.io/library/python@sha256:[0-9a-f]{64}', pins['pythonIndex']),
                    'invalid Python index pin')
            pin = pins['pythonManifests'][arch]
            require(re.fullmatch(r'sha256:[0-9a-f]{64}', pin), 'invalid Python platform pin')
            manifest['python'] = {'index': pins['pythonIndex'], 'manifest': pin}
            # Only the exact CLI and this generated file enter the build context.
            containerfile = tmp / 'Containerfile'
            containerfile.write_text('FROM docker.io/library/python@' + pin +
                                     '\nCOPY codex /usr/local/bin/codex\nUSER 10003:10003\n')
            # Tamper fixtures are no longer needed and are excluded from the context.
            for name in ('tool-controls.json', 'restricted-models.json'):
                (tmp / name).unlink()
            capture = owner.build(tmp, containerfile, 'capture', arch)
            manifest['captureImage'] = capture
            result_dir = tmp / 'result'
            result_dir.mkdir(mode=0o777)
            result_dir.chmod(0o777)  # The container writes only as UID 10003.
            # Parent traversal is needed for the runner to read output; Docker mounts the leaf.
            result = owner.container('capture', capture,
                                     ['/proof/scripts/test-native-tool-policy.py', '--inside', '/result/proof.json'],
                                     [str(root / 'scripts') + ':/proof/scripts:ro',
                                      str(assets) + ':/proof/' + str(ASSETS) + ':ro',
                                      str(result_dir) + ':/result:rw'], 'python3')
            require(result.returncode == 0, 'native CLI capture failed')
            manifest['outcomes']['capture'] = True
            actual = (result_dir / 'proof.json').read_bytes()
            manifest['proof'] = validate_proof(actual, expected)
            (output / 'proof.json').write_bytes(actual)
            manifest['outcomes']['proof'] = True
        require(source(root, commit, run) == manifest['source'], 'source changed during acceptance')
        manifest['outcomes']['sourceFinal'] = True
    except (AcceptanceError, OSError, ValueError, KeyError, TypeError, subprocess.SubprocessError) as error:
        manifest['error'] = str(error)[:240] if isinstance(error, AcceptanceError) else type(error).__name__
    finally:
        cleaned = cleanup_output(output, run) == 0 if owner else True
        manifest['cleanup'] = {'passed': cleaned}
        if not cleaned:
            manifest['error'] = 'owned-resource cleanup failed'
        if not manifest.get('error') and REQUIRED <= manifest['outcomes'].keys() and cleaned:
            manifest['status'] = 'passed'
        write_json(output / 'manifest.json', manifest)
    return 0 if manifest['status'] == 'passed' else 1


def verify_runtime(root, output, arch, commit, toolchain, run=execute):
    root, output = prepare_output(Path(root), Path(output))
    manifest = {'status': 'failed', 'kind': 'native-runtime'}
    try:
        require(toolchain in ('1.85.0', '1.93.1'), 'unsupported Rust toolchain')
        manifest['source'] = source(root, commit, run)
        manifest['native'] = native(arch)
        rust = checked(run, ['rustc', '-vV'])
        cargo = checked(run, ['cargo', '--version'])
        fields = dict(line.split(': ', 1) for line in rust.splitlines() if ': ' in line)
        require(fields.get('release') == toolchain and cargo.split()[1] == toolchain,
                'actual Rust/Cargo version differs from matrix cell')
        require(fields.get('host') == TARGETS[arch][0] + '-unknown-linux-gnu', 'Rust host is not native')
        # Reject environment cross-target overrides, even if a native compiler exists.
        require(not os.environ.get('CARGO_BUILD_TARGET'), 'cross-target environment is forbidden')
        manifest['toolchain'] = {'requested': toolchain, 'rustc': rust, 'cargo': cargo}
        summary = checked(run, ['cargo', 'test', '--locked', '--', '--test-threads=1'], cwd=root, timeout=1800)
        (output / 'verification.txt').write_text(summary[-16384:] + '\n')
        require(source(root, commit, run) == manifest['source'], 'source changed during runtime tests')
        manifest['status'] = 'passed'
    except (AcceptanceError, OSError, ValueError, KeyError, TypeError, IndexError, subprocess.SubprocessError) as error:
        manifest['error'] = str(error)[:240] if isinstance(error, AcceptanceError) else type(error).__name__
    write_json(output / 'manifest.json', manifest)
    return 0 if manifest['status'] == 'passed' else 1


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('mode', choices=('runtime', 'package', 'cleanup'))
    parser.add_argument('--output', required=True, type=Path)
    parser.add_argument('--arch', choices=tuple(TARGETS))
    parser.add_argument('--commit')
    parser.add_argument('--toolchain', choices=('1.85.0', '1.93.1'))
    args = parser.parse_args()
    def interrupted(_signum, _frame):
        raise AcceptanceError('acceptance interrupted')
    signal.signal(signal.SIGTERM, interrupted)
    signal.signal(signal.SIGINT, interrupted)
    try:
        if args.mode == 'cleanup':
            return cleanup_output(args.output)
        require(args.arch and args.commit, 'architecture and event commit are required')
        if args.mode == 'runtime':
            return verify_runtime(ROOT, args.output, args.arch, args.commit, args.toolchain)
        return verify_package(ROOT, args.output, args.arch, args.commit)
    except AcceptanceError as error:
        print(str(error))
        return 1


if __name__ == '__main__':
    raise SystemExit(main())
