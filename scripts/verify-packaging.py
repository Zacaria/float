"""Fail-closed assertions on final release containers; native commands run only in CLI."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import plistlib
import struct
import subprocess
import sys
import tempfile

ROOT = Path(__file__).resolve().parents[1]


def require(condition, message):
    if not condition:
        raise ValueError(message)


def sha(data):
    return hashlib.sha256(data).hexdigest()


def run(*args):
    try:
        return subprocess.run([str(arg) for arg in args], check=True, capture_output=True).stdout
    except subprocess.CalledProcessError as error:
        sys.stderr.write(error.stdout.decode(errors='replace') + error.stderr.decode(errors='replace'))
        raise


def approved(relative):
    manifest = json.loads((ROOT / 'tests/branding-exports.json').read_text())
    expected = next(item['sha256'] for item in manifest if item['path'] == relative)
    data = (ROOT / relative).read_bytes()
    require(sha(data) == expected, f'Unapproved source: {relative}')
    return data


def verify_app(app, version):
    require(app.name == 'Float.app' and not app.is_symlink(), 'Expected exact Float.app')
    info = plistlib.loads((app / 'Contents/Info.plist').read_bytes())
    for key in ('CFBundleShortVersionString', 'CFBundleVersion'):
        require(info.get(key) == version, f'Wrong {key}: {info.get(key)}')
    require(info.get('CFBundleIdentifier') == 'com.havesomecode.float', 'Wrong bundle identifier')
    name = info.get('CFBundleIconFile', '')
    require(name and Path(name).name == name, 'Invalid CFBundleIconFile')
    if not name.endswith('.icns'):
        name += '.icns'
    icon = app / 'Contents/Resources' / name
    data = icon.read_bytes()
    source = approved('src-tauri/icons/icon.icns')
    require(data == source, 'Packaged ICNS differs from approved source')
    # Tauri normally compiles web assets into the executable. Do not pretend
    # that searching compressed executable bytes verifies those assets.
    web = {}
    for source_file in sorted((ROOT / 'dist').rglob('*')):
        if not source_file.is_file():
            continue
        relative = source_file.relative_to(ROOT).as_posix()
        matches = list((app / 'Contents/Resources').rglob(source_file.name))
        for match in matches:
            require(match.read_bytes() == source_file.read_bytes(), f'Stale packaged web asset: {match}')
        web[relative] = {'status': 'verified' if matches else 'not-accessible-compiled-into-executable',
                         'copies': len(matches)}
    return icon, {'source_icon_sha256': sha(source), 'packaged_icon_sha256': sha(data), 'web_assets': web}


def verify_volume_icon(mount):
    data = (mount / '.VolumeIcon.icns').read_bytes()
    require(data == approved('src-tauri/icons/icon.icns'), 'DMG volume icon differs from approved source')
    return sha(data)


def ico_frames(data):
    require(len(data) >= 6, 'Truncated ICO')
    reserved, kind, count = struct.unpack_from('<HHH', data)
    require(reserved == 0 and kind == 1 and count > 0, 'Invalid ICO header')
    require(len(data) >= 6 + 16 * count, 'Truncated ICO directory')
    frames = []
    for index in range(count):
        offset = 6 + 16 * index
        length, start = struct.unpack_from('<II', data, offset + 8)
        require(length > 0 and start >= 6 + 16 * count and start + length <= len(data), 'Invalid ICO frame')
        frames.append((data[offset:offset + 8], data[start:start + length]))
    return frames


def verify_group(group, resources, source_frames, language):
    require(len(group) >= 6, 'Truncated icon group')
    reserved, kind, count = struct.unpack_from('<HHH', group)
    require((reserved, kind, count) == (0, 1, len(source_frames)), 'Wrong icon group frame count/header')
    require(len(group) == 6 + count * 14, 'Invalid icon group length')
    actual = []
    for index in range(count):
        offset = 6 + 14 * index
        length, resource_id = struct.unpack_from('<IH', group, offset + 8)
        payload = resources.get((resource_id, language))
        require(payload is not None and len(payload) == length, 'Missing/truncated icon resource')
        actual.append((group[offset:offset + 8], payload))
    require(sorted(actual) == sorted(source_frames), 'Embedded PE icon differs from approved ICO')
    return [sha(payload) for _, payload in actual]


def verify_pe(path, version=None):
    import pefile  # pinned in CI; no dependency needed for macOS
    source = approved('src-tauri/icons/icon.ico')
    frames = ico_frames(source)
    with pefile.PE(str(path)) as pe:
        require(hasattr(pe, 'DIRECTORY_ENTRY_RESOURCE'), 'PE has no resources')
        icons, groups = {}, []
        for resource_type in pe.DIRECTORY_ENTRY_RESOURCE.entries:
            if resource_type.id not in (3, 14):
                continue
            for resource in resource_type.directory.entries:
                for language in resource.directory.entries:
                    entry = language.data.struct
                    payload = pe.get_data(entry.OffsetToData, entry.Size)
                    require(len(payload) == entry.Size, 'Truncated PE resource')
                    if resource_type.id == 3:
                        icons[(resource.id, language.id)] = payload
                    else:
                        groups.append((resource.id, language.id, payload))
        require(groups, 'PE has no icon groups')
        hashes = [verify_group(data, icons, frames, lang) for _, lang, data in groups]
        if version is not None:
            require(getattr(pe, 'VS_FIXEDFILEINFO', None), 'Missing PE version')
            expected = tuple(map(int, version.split('.'))) + (0,)
            for info in pe.VS_FIXEDFILEINFO:
                for prefix in ('FileVersion', 'ProductVersion'):
                    high, low = getattr(info, prefix + 'MS'), getattr(info, prefix + 'LS')
                    require((high >> 16, high & 65535, low >> 16, low & 65535) == expected,
                            f'Wrong PE {prefix}')
    return {'file': path.name, 'sha256': sha(path.read_bytes()), 'source_icon_sha256': sha(source),
            'packaged_icon_frame_sha256': hashes,
            'source_icon_frame_sha256': [sha(payload) for _, payload in frames]}


def verify_dmg(artifact, version, temp_root):
    run('xcrun', 'stapler', 'validate', artifact)
    run('spctl', '-a', '-vvv', '-t', 'open', '--context', 'context:primary-signature', artifact)
    with tempfile.TemporaryDirectory(prefix='float-dmg-', dir=temp_root) as staging:
        mount = Path(staging) / 'mount'
        mount.mkdir()
        try:
            run('hdiutil', 'attach', '-readonly', '-nobrowse', '-mountpoint', mount, artifact)
            apps = list(mount.glob('*.app'))
            require(len(apps) == 1 and apps[0].name == 'Float.app', 'DMG must contain exactly Float.app')
            app = apps[0]
            icon, evidence = verify_app(app, version)
            evidence['volume_icon_sha256'] = verify_volume_icon(mount)
            run('codesign', '--verify', '--deep', '--strict', '--verbose=2', app)
            run('spctl', '-a', '-vvv', '-t', 'execute', app)
            run('iconutil', '--convert', 'iconset', '--output', Path(staging) / 'decoded.iconset', icon)
            require(list((Path(staging) / 'decoded.iconset').glob('*.png')), 'ICNS decode produced no images')
            return evidence
        finally:
            # Also attempt detach after a partially successful attach. Never hide failure.
            run('hdiutil', 'detach', mount)


def verify_windows(artifact, version, temp_root):
    built = ROOT / 'src-tauri/target/release/float-tauri.exe'
    evidence = {'application': verify_pe(built, version), 'installer': verify_pe(artifact, version)}
    with tempfile.TemporaryDirectory(prefix='float-nsis-', dir=temp_root) as staging:
        destination = Path(staging) / 'installed'
        # NSIS /D must be last and unquoted; subprocess list2cmdline would quote
        # paths with spaces. Pass a raw Windows command line deliberately.
        command = f'"{artifact}" /S /NS /D={destination}'
        subprocess.run(command, check=True, timeout=180)
        installed = destination / 'float-tauri.exe'
        require(installed.is_file(), 'NSIS did not install float-tauri.exe')
        evidence['installed_application'] = verify_pe(installed, version)
        evidence['uninstaller'] = verify_pe(destination / 'uninstall.exe', version)
        require(installed.read_bytes() == built.read_bytes(), 'Installed EXE differs from built EXE')
    return evidence


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('platform', choices=['macos', 'windows'])
    parser.add_argument('artifact', type=Path)
    parser.add_argument('report', type=Path)
    args = parser.parse_args()
    version = json.loads((ROOT / 'src-tauri/tauri.conf.json').read_text())['version']
    reference = os.environ.get('GITHUB_REF', '')
    if reference.startswith('refs/tags/'):
        require(reference == f'refs/tags/v{version}', 'Release tag/config version mismatch')
    artifact = args.artifact.resolve()
    temp_root = os.environ.get('RUNNER_TEMP', str(ROOT))
    verify = verify_dmg if args.platform == 'macos' else verify_windows
    evidence = verify(artifact, version, temp_root)
    report = {'source_commit': run('git', '-C', ROOT, 'rev-parse', 'HEAD').decode().strip(),
              'package_version': version, 'platform': args.platform, 'artifact': artifact.name,
              'approved_master_sha256': sha(approved('src-tauri/icons/icon_base_1024.png')),
              'artifact_sha256': sha(artifact.read_bytes()), 'verification': evidence}
    args.report.write_text(json.dumps(report, separators=(',', ':')) + '\n')
    print(json.dumps(report, separators=(',', ':')))


if __name__ == '__main__':
    main()
