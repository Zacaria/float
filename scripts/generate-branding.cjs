// Export the approved art only. No image generation, recoloring, masking or flattening.
const assert = require('node:assert/strict');
const { createHash } = require('node:crypto');
const { execFileSync } = require('node:child_process');
const { mkdtempSync, readFileSync, writeFileSync, rmSync } = require('node:fs');
const os = require('node:os');
const path = require('node:path');
const manifest = require('../tests/branding-exports.json');

const root = path.resolve(__dirname, '..');
const source = path.resolve(process.argv[2] || path.join(root, 'src-tauri/icons/icon_base_1024.png'));
const master = readFileSync(source);
const hash = (bytes) => createHash('sha256').update(bytes).digest('hex');
assert.equal(hash(master), 'c17820f006724e0c375a6e0d10c13aa41aa45bbb49868af8169ece8f6f53dc81', 'Unapproved source; exports were not changed');
const staging = mkdtempSync(path.join(os.tmpdir(), 'float-branding-'));

try {
    const input = path.join(staging, 'approved.png');
    writeFileSync(input, master);
    const generate = (args) => execFileSync(process.platform === 'win32' ? 'npm.cmd' : 'npm', [
        'exec', '--yes', '--package=@tauri-apps/cli@2.8.4', '--', 'tauri', 'icon', input, ...args,
    ], { cwd: root, stdio: 'inherit' });
    generate(['--output', path.join(staging, 'icons'), '--ios-color', '#00000000']);
    generate(['--output', path.join(staging, 'web'), '--png', '180']);

    // Tauri's ICNS representations have nondeterministic ordering. Sort the
    // container entries only; preserve every encoded image/mask byte verbatim.
    const icnsPath = path.join(staging, 'icons/icon.icns');
    const icns = readFileSync(icnsPath);
    const chunks = [];
    for (let offset = 8; offset < icns.length;) {
        const length = icns.readUInt32BE(offset + 4);
        assert.ok(length > 8 && offset + length <= icns.length);
        chunks.push(icns.subarray(offset, offset + length));
        offset += length;
    }
    chunks.sort((left, right) => Buffer.compare(left.subarray(0, 4), right.subarray(0, 4)));
    writeFileSync(icnsPath, Buffer.concat([icns.subarray(0, 8), ...chunks]));

    // Validate all staged outputs before touching the repository. The manifest
    // is a reviewed regression fixture, never rewritten by this command.
    const exports = manifest.map((entry) => {
        let bytes;
        if (entry.path === 'src-tauri/icons/icon_base_1024.png' || entry.path === 'site/assets/float-icon.png') {
            bytes = master;
        } else if (entry.path === 'dist/favicon.ico') {
            bytes = readFileSync(path.join(staging, 'icons/icon.ico'));
        } else if (entry.path === 'dist/apple-touch-icon.png') {
            bytes = readFileSync(path.join(staging, 'web/180x180.png'));
        } else {
            bytes = readFileSync(path.join(staging, entry.path.replace(/^src-tauri\//, '')));
        }
        assert.equal(hash(bytes), entry.sha256, `Unexpected export: ${entry.path}`);
        return { name: path.join(root, entry.path), bytes };
    });
    for (const { name, bytes } of exports) writeFileSync(name, bytes);
    console.log(`Verified and wrote ${exports.length} branding files using Tauri CLI 2.8.4.`);
} finally {
    rmSync(staging, { recursive: true, force: true });
}
