"""Real byte fixtures test assertions; native signing/install commands are not simulated successes."""
import importlib.util
from pathlib import Path
import plistlib
import struct
import tempfile
import unittest
from unittest.mock import patch

spec = importlib.util.spec_from_file_location('packaging_checks', Path(__file__).with_name('verify-packaging.py'))
checks = importlib.util.module_from_spec(spec)
spec.loader.exec_module(checks)
ROOT = checks.ROOT


def pe_fixture(frames, extra_group=False, version=None):
    """Minimal non-executable PE32 with a real .rsrc tree parsed by pefile."""
    group = struct.pack('<HHH', 0, 1, len(frames))
    icons = {}
    for index, (metadata, payload) in enumerate(frames, 1):
        group += metadata + struct.pack('<IH', len(payload), index)
        icons[index] = {1033: payload}
    groups = {1: {1033: group}}
    if extra_group:
        groups[2] = {1033: struct.pack('<HHH', 0, 1, 0)}
    tree = {3: icons, 14: groups}
    if version is not None:
        major, minor, patch_version = version
        high, low = major << 16 | minor, patch_version << 16
        fixed = struct.pack('<13I', 0xfeef04bd, 0x10000, high, low, high, low, 0x3f, 0, 0x40004, 1, 0, 0, 0)
        value = bytearray(struct.pack('<HHH', 0, len(fixed), 0) + 'VS_VERSION_INFO\0'.encode('utf-16le'))
        value.extend(bytes(-len(value) % 4))
        value.extend(fixed)
        struct.pack_into('<H', value, 0, len(value))
        tree[16] = {1: {1033: bytes(value)}}
    section = bytearray()

    def append(data):
        offset = len(section)
        section.extend(data)
        return offset

    def directory(entries):
        offset = append(struct.pack('<IIHHHH', 0, 0, 0, 0, 0, len(entries)) + bytes(8 * len(entries)))
        for index, (key, value) in enumerate(sorted(entries.items())):
            if isinstance(value, dict):
                target = directory(value) | 0x80000000
            else:
                target = append(bytes(16))
                payload = append(value)
                struct.pack_into('<IIII', section, target, 0x1000 + payload, len(value), 0, 0)
                section.extend(bytes(-len(section) % 4))
            struct.pack_into('<II', section, offset + 16 + 8 * index, key, target)
        return offset

    directory(tree)
    headers = bytearray(512)
    headers[:2] = b'MZ'
    struct.pack_into('<I', headers, 0x3c, 0x80)
    headers[0x80:0x84] = b'PE\0\0'
    struct.pack_into('<HHIIIHH', headers, 0x84, 0x14c, 1, 0, 0, 0, 224, 0x102)
    optional = 0x98
    struct.pack_into('<H', headers, optional, 0x10b)
    struct.pack_into('<III', headers, optional + 28, 0x400000, 0x1000, 0x200)
    struct.pack_into('<II', headers, optional + 56, (len(section) + 0x1fff) & ~0xfff, 512)
    struct.pack_into('<I', headers, optional + 92, 16)
    struct.pack_into('<II', headers, optional + 112, 0x1000, len(section))
    raw_size = (len(section) + 511) & ~511
    struct.pack_into('<8sIIIIIIHHI', headers, optional + 224, b'.rsrc\0\0\0', len(section),
                     0x1000, raw_size, 512, 0, 0, 0, 0, 0x40000040)
    return headers + section + bytes(raw_size - len(section))


class PackagingTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory(dir=ROOT, prefix='.packaging-test-')
        self.addCleanup(self.temp.cleanup)
        self.directory = Path(self.temp.name)
        self.app = self.directory / 'Float.app'
        resources = self.app / 'Contents/Resources'
        resources.mkdir(parents=True)
        self.info = {'CFBundleShortVersionString': '0.2.1', 'CFBundleVersion': '0.2.1',
                     'CFBundleIdentifier': 'com.havesomecode.float', 'CFBundleIconFile': 'custom-name'}
        self.write_info()
        self.icon = resources / 'custom-name.icns'
        self.icon.write_bytes(checks.approved('src-tauri/icons/icon.icns'))

    def write_info(self):
        (self.app / 'Contents/Info.plist').write_bytes(plistlib.dumps(self.info))

    def test_mac_uses_plist_icon_and_checks_exposed_web_assets(self):
        favicon = self.icon.parent / 'favicon.ico'
        favicon.write_bytes(checks.approved('dist/favicon.ico'))
        icon, evidence = checks.verify_app(self.app, '0.2.1')
        self.assertEqual(icon, self.icon)
        self.assertEqual(evidence['source_icon_sha256'], evidence['packaged_icon_sha256'])
        self.assertEqual(evidence['web_assets']['dist/favicon.ico']['status'], 'verified')
        favicon.write_bytes(b'stale')
        with self.assertRaisesRegex(ValueError, 'Stale packaged web asset'):
            checks.verify_app(self.app, '0.2.1')

    def test_mac_rejects_actual_v020_icon(self):
        self.icon.write_bytes((ROOT / 'tests/fixtures/packaging/v0.2.0-icon.icns').read_bytes())
        with self.assertRaisesRegex(ValueError, 'Packaged ICNS differs'):
            checks.verify_app(self.app, '0.2.1')

    def test_mac_rejects_stale_versions_and_unsafe_icon_path(self):
        for key, value in [('CFBundleVersion', '0.2.0'), ('CFBundleShortVersionString', '0.2.0'),
                           ('CFBundleIconFile', '../icon.icns')]:
            with self.subTest(key=key):
                original = self.info[key]
                self.info[key] = value
                self.write_info()
                with self.assertRaises(ValueError):
                    checks.verify_app(self.app, '0.2.1')
                self.info[key] = original

    def test_volume_icon_rejects_old_or_missing_art(self):
        volume = self.directory / '.VolumeIcon.icns'
        with self.assertRaises(FileNotFoundError):
            checks.verify_volume_icon(self.directory)
        volume.write_bytes(checks.approved('src-tauri/icons/icon.icns'))
        self.assertEqual(checks.verify_volume_icon(self.directory), checks.sha(volume.read_bytes()))
        volume.write_bytes((ROOT / 'tests/fixtures/packaging/v0.2.0-icon.icns').read_bytes())
        with self.assertRaisesRegex(ValueError, 'DMG volume icon differs'):
            checks.verify_volume_icon(self.directory)

    def test_dmg_detaches_after_verification_failure(self):
        # This tests cleanup control flow only, not native verification.
        calls = []
        def command(*args):
            calls.append(tuple(str(arg) for arg in args))
            return b''
        with patch.object(checks, 'run', side_effect=command):
            with self.assertRaisesRegex(ValueError, 'DMG must contain exactly Float.app'):
                checks.verify_dmg(self.directory / 'test.dmg', '0.2.1', self.directory)
        self.assertEqual(calls[-1][:2], ('hdiutil', 'detach'))
        attach = next(call for call in calls if call[:2] == ('hdiutil', 'attach'))
        self.assertIn('-readonly', attach)

    def test_pe_parser_accepts_approved_frames(self):
        frames = checks.ico_frames(checks.approved('src-tauri/icons/icon.ico'))
        exe = self.directory / 'approved.exe'
        exe.write_bytes(pe_fixture(frames))
        result = checks.verify_pe(exe)
        self.assertEqual(result['packaged_icon_frame_sha256'], [result['source_icon_frame_sha256']])

    def test_pe_parser_rejects_actual_v020_icon(self):
        old = (ROOT / 'tests/fixtures/packaging/v0.2.0-icon.ico').read_bytes()
        exe = self.directory / 'stale.exe'
        exe.write_bytes(pe_fixture(checks.ico_frames(old)))
        with self.assertRaisesRegex(ValueError, 'Wrong icon group|differs from approved'):
            checks.verify_pe(exe)

    def test_pe_parser_rejects_one_stale_frame_and_extra_group(self):
        frames = checks.ico_frames(checks.approved('src-tauri/icons/icon.ico'))
        exe = self.directory / 'mixed.exe'
        for extra in (False, True):
            mixed = frames.copy()
            if not extra:
                metadata, payload = mixed[-1]
                mixed[-1] = (metadata, payload[:-1] + bytes([payload[-1] ^ 1]))
            exe.write_bytes(pe_fixture(mixed, extra_group=extra))
            with self.assertRaises(ValueError):
                checks.verify_pe(exe)

    def test_pe_parser_rejects_missing_versions(self):
        exe = self.directory / 'unversioned.exe'
        exe.write_bytes(pe_fixture(checks.ico_frames(checks.approved('src-tauri/icons/icon.ico'))))
        with self.assertRaisesRegex(ValueError, 'Missing PE version'):
            checks.verify_pe(exe, '0.2.1')

    def test_pe_versions_match_config_and_reject_old_release(self):
        exe = self.directory / 'versioned.exe'
        frames = checks.ico_frames(checks.approved('src-tauri/icons/icon.ico'))
        exe.write_bytes(pe_fixture(frames, version=(0, 2, 1)))
        checks.verify_pe(exe, '0.2.1')
        exe.write_bytes(pe_fixture(frames, version=(0, 2, 0)))
        with self.assertRaisesRegex(ValueError, 'Wrong PE FileVersion'):
            checks.verify_pe(exe, '0.2.1')

    def test_truncated_ico_is_rejected(self):
        for data in (b'', b'\0' * 6, checks.approved('src-tauri/icons/icon.ico')[:-20]):
            with self.assertRaises(ValueError):
                checks.ico_frames(data)


if __name__ == '__main__':
    unittest.main()
