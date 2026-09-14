const assert = require('node:assert/strict');
const { createHash } = require('node:crypto');
const { readFileSync, readdirSync } = require('node:fs');
const path = require('node:path');
const { test } = require('node:test');
const exportsManifest = require('./branding-exports.json');

const root = path.resolve(__dirname, '..');
const read = (name) => readFileSync(path.join(root, name));
const hash = (bytes) => createHash('sha256').update(bytes).digest('hex');
const approvedHash = 'c17820f006724e0c375a6e0d10c13aa41aa45bbb49868af8169ece8f6f53dc81';

test('canonical and site masters preserve the exact approved PNG bytes', () => {
    const master = read('src-tauri/icons/icon_base_1024.png');
    assert.equal(hash(master), approvedHash);
    assert.deepEqual(read('site/assets/float-icon.png'), master);
    assert.deepEqual([master.readUInt32BE(16), master.readUInt32BE(20), master[24], master[25]], [1024, 1024, 8, 6]);
});

test('the manifest covers the entire existing native icon family', () => {
    const actual = readdirSync(path.join(root, 'src-tauri/icons'), { recursive: true })
        .filter((name) => /\.(png|ico|icns)$/.test(name))
        .map((name) => `src-tauri/icons/${name.split(path.sep).join('/')}`).sort();
    assert.deepEqual(actual, exportsManifest.map((entry) => entry.path).filter((name) => name.startsWith('src-tauri/')).sort());
});

for (const entry of exportsManifest) {
    test(`approved export bytes and PNG metadata: ${entry.path}`, () => {
        const bytes = read(entry.path);
        assert.equal(hash(bytes), entry.sha256);
        if (entry.png) {
            assert.deepEqual(bytes.subarray(0, 8), Buffer.from('89504e470d0a1a0a', 'hex'));
            assert.equal(bytes.toString('ascii', 12, 16), 'IHDR');
            assert.deepEqual([bytes.readUInt32BE(16), bytes.readUInt32BE(20), bytes[24], bytes[25]], entry.png);
        }
    });
}

test('web branding references resolve to the approved assets', () => {
    for (const [page, directory, expected] of [
        ['dist/index.html', 'dist', ['/favicon.ico', '/apple-touch-icon.png']],
        ['site/index.html', 'site', ['./assets/float-icon.png', './assets/float-icon.png', './assets/float-icon.png']],
    ]) {
        const tags = read(page).toString().match(/<(?:link|img)\b[^>]*>/g) || [];
        const references = tags.filter((tag) => /(?:rel="(?:icon|apple-touch-icon)"|class="brand-mark")/.test(tag))
            .map((tag) => tag.match(/(?:href|src)="([^"]+)"/)[1]);
        assert.deepEqual(references, expected, page);
        for (const reference of references) {
            const name = path.posix.join(directory, reference.replace(/^\//, ''));
            assert.ok(exportsManifest.some((entry) => entry.path === name), name);
            assert.ok(read(name).length > 0, name);
        }
    }
});

test('configured bundle paths all reference verified exports', () => {
    const config = JSON.parse(read('src-tauri/tauri.conf.json'));
    assert.equal(config.build.frontendDist, '../dist');
    assert.deepEqual(config.bundle.icon, ['icons/32x32.png', 'icons/128x128.png', 'icons/128x128@2x.png', 'icons/icon.icns', 'icons/icon.ico']);
    for (const icon of config.bundle.icon) {
        assert.ok(exportsManifest.some((entry) => entry.path === `src-tauri/${icon}`));
        assert.ok(read(`src-tauri/${icon}`).length > 0);
    }
});

test('native and webview ICO bytes match, including every frame', () => {
    const bytes = read('src-tauri/icons/icon.ico');
    assert.deepEqual(read('dist/favicon.ico'), bytes);
    assert.equal(bytes.readUInt16LE(0), 0);
    assert.equal(bytes.readUInt16LE(2), 1);
    const sizes = [];
    for (let index = 0; index < bytes.readUInt16LE(4); index++) {
        const offset = 6 + index * 16;
        const size = bytes[offset] || 256;
        assert.equal(bytes[offset + 1] || 256, size);
        const length = bytes.readUInt32LE(offset + 8);
        const start = bytes.readUInt32LE(offset + 12);
        assert.ok(length > 0 && start >= 6 + bytes.readUInt16LE(4) * 16 && start + length <= bytes.length);
        sizes.push(size);
    }
    assert.deepEqual(sizes.sort((a, b) => a - b), [16, 24, 32, 48, 64, 256]);
});

test('ICNS has the complete native representation set', () => {
    const bytes = read('src-tauri/icons/icon.icns');
    assert.equal(bytes.toString('ascii', 0, 4), 'icns');
    assert.equal(bytes.readUInt32BE(4), bytes.length);
    const types = [];
    for (let offset = 8; offset < bytes.length;) {
        const length = bytes.readUInt32BE(offset + 4);
        assert.ok(length > 8 && offset + length <= bytes.length);
        types.push(bytes.toString('ascii', offset, offset + 4));
        offset += length;
    }
    assert.deepEqual(types.sort(), ['ic07', 'ic08', 'ic09', 'ic10', 'ic11', 'ic12', 'ic13', 'ic14', 'il32', 'is32', 'l8mk', 's8mk'].sort());
});
