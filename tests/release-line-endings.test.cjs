const { test } = require('node:test');
const assert = require('node:assert/strict');
const { mkdtempSync, mkdirSync, readFileSync, writeFileSync, rmSync } = require('node:fs');
const { spawnSync } = require('node:child_process');
const { tmpdir } = require('node:os');
const path = require('node:path');

// Exercise the real release checks against a Windows-style text checkout,
// even when this regression runs on Linux or macOS. No native build is mocked.
test('release checks accept CRLF checkouts without weakening assertions', () => {
    const root = path.resolve(__dirname, '..');
    const temporary = mkdtempSync(path.join(tmpdir(), 'float-crlf-'));
    const inputs = [
        'Cargo.toml', 'Cargo.lock', 'src-tauri/Cargo.toml', 'src-tauri/Cargo.lock',
        'src-tauri/tauri.conf.json', 'package.json', 'package-lock.json',
        'site/index.html', 'CHANGELOG.md', '.github/workflows/release-bundles.yml',
        'scripts/nsis-branding.nsh',
    ];
    try {
        for (const relative of inputs) {
            const target = path.join(temporary, relative);
            mkdirSync(path.dirname(target), { recursive: true });
            writeFileSync(target, readFileSync(path.join(root, relative), 'utf8').replace(/\r?\n/g, '\r\n'));
        }
        const env = { ...process.env };
        delete env.NODE_TEST_CONTEXT; // Run a separate test runner, not a nested child.
        const check = () => spawnSync(process.execPath, ['--test', path.join(__dirname, 'release.test.cjs')], {
            cwd: temporary, env, encoding: 'utf8', timeout: 30000,
        });
        const valid = check();
        assert.ifError(valid.error);
        assert.equal(valid.status, 0, valid.stdout + valid.stderr);

        // A wrong version must still fail; normalization must not bypass checks.
        const configPath = path.join(temporary, 'src-tauri/tauri.conf.json');
        const config = JSON.parse(readFileSync(configPath, 'utf8'));
        config.version = '0.0.0';
        writeFileSync(configPath, JSON.stringify(config));
        const invalid = check();
        assert.ifError(invalid.error);
        assert.notEqual(invalid.status, 0, 'wrong package version unexpectedly accepted\n' + invalid.stdout + invalid.stderr);
    } finally {
        rmSync(temporary, { recursive: true, force: true });
    }
});
