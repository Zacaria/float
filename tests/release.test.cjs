const { test } = require('node:test');
const assert = require('node:assert/strict');
const { readFileSync } = require('node:fs');
const read = (p) => readFileSync(p, 'utf8');
const json = (p) => JSON.parse(read(p));

test('all own versions and live download copy are 0.2.1', () => {
    const version = '0.2.1';
    for (const [file, name] of [['Cargo.toml', 'float'], ['Cargo.lock', 'float'], ['src-tauri/Cargo.toml', 'float-tauri'], ['src-tauri/Cargo.lock', 'float-tauri']]) {
        assert.equal(read(file).match(new RegExp(`name = "${name}"\\nversion = "([^"]+)"`))[1], version, file);
    }
    assert.equal(json('src-tauri/tauri.conf.json').version, version);
    assert.equal(json('package.json').version, version);
    assert.equal(json('package-lock.json').version, version);
    assert.equal(json('package-lock.json').packages[''].version, version);
    assert.match(read('site/index.html'), /Latest: v0\.2\.1/);
    assert.match(read('CHANGELOG.md'), /## 0\.2\.1 - \d{4}-\d{2}-\d{2}/);
});

test('legacy bundle uses canonical approved ICNS', () => {
    assert.match(read('Cargo.toml'), /icon = \["src-tauri\/icons\/icon.icns"\]/);
});

test('both release runners verify final artifacts before upload and publication', () => {
    const workflow = read('.github/workflows/release-bundles.yml');
    for (const platform of ['macos', 'windows']) {
        const job = workflow.split(`  build-${platform}-public:`)[1].split(/\n  [a-z]+[\w-]*:/)[0];
        assert.match(job, /npm run test:branding/);
        assert.match(job, /npm run test:release/);
        assert.match(job, /npm exec --yes --package=@tauri-apps\/cli@2\.8\.4 -- tauri build/);
        assert.ok(job.indexOf(`verify-packaging.py ${platform}`) > job.indexOf('Create stable'));
        assert.ok(job.indexOf(`verify-packaging.py ${platform}`) < job.indexOf(`name: Upload ${platform === 'macos' ? 'macOS' : 'Windows'} release artifact`));
        assert.match(job, /name: float-.*-verification/);
    }
    assert.doesNotMatch(workflow, /cargo install tauri-cli|continue-on-error/);
    assert.match(workflow, /permissions:\n  contents: read/);
    assert.match(workflow, /needs:\n      - build-macos-public\n      - build-windows-public/);
    assert.match(workflow, /release:\n[\s\S]*permissions:\n      contents: write/);
});


test('NSIS installer and uninstaller both use approved ICO', () => {
    const nsis = json('src-tauri/tauri.conf.json').bundle.windows.nsis;
    assert.equal(nsis.installerIcon, 'icons/icon.ico');
    assert.equal(nsis.installerHooks, '../scripts/nsis-branding.nsh');
    assert.match(read('scripts/nsis-branding.nsh'), /!define MUI_UNICON/);
    assert.match(read('scripts/nsis-branding.nsh'), /icon\.ico/);
});
