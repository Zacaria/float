import { test, expect } from '@playwright/test';
import path from 'path';

/**
 * Frontend-only smoke test: load dist/index.html with a mocked TAURI invoke implementation
 * to verify the Open flow updates the UI.
 */
test('open flow updates UI with mocked tauri', async ({ page }) => {
  const distPath = path.resolve(__dirname, '..', 'dist', 'index.html');
  const mockPath = '/tmp/icon.png';

  await page.addInitScript(({ mockPath }) => {
    // Minimal __TAURI__ mock to satisfy invoke calls
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    (window as any).__TAURI__ = {
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
      invoke: (cmd: string, args: any = {}) => {
        if (cmd === 'choose_file') return Promise.resolve(mockPath);
        if (cmd === 'fit_now') return Promise.resolve();
        if (cmd === 'quick_look') return Promise.resolve();
        if (cmd === 'get_settings') return Promise.resolve({ aspect_lock: false, fit_window: true });
        if (cmd === 'set_settings') {
          return Promise.resolve({
            aspect_lock: !!args.update?.aspect_lock,
            fit_window: args.update?.fit_window ?? true,
          });
        }
        return Promise.resolve();
      },
    };
  }, { mockPath });

  await page.goto(`file://${distPath}`);

  await page.click('#openBtn');
  await expect(page.locator('#fileName')).toHaveText('icon.png');
  await expect(page.locator('#status')).toHaveText('');
});
