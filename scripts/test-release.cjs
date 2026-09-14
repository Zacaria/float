const { spawnSync } = require('node:child_process');
for (const [command, args] of [
    [process.execPath, ['--test', 'tests/release.test.cjs', 'tests/release-line-endings.test.cjs']],
    [process.platform === 'win32' ? 'python' : 'python3', ['scripts/test-packaging.py']],
]) {
    const result = spawnSync(command, args, { stdio: 'inherit' });
    if (result.error) throw result.error;
    if (result.status !== 0) process.exit(result.status || 1);
}
