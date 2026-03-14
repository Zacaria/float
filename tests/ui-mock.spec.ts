import { test, expect } from '@playwright/test';
import path from 'path';

/**
 * Frontend-only smoke test: load dist/index.html with a mocked Tauri v2 API
 * and verify bootstrap renders the selected file state.
 */
test('bootstrap renders selected file with mocked tauri', async ({ page }) => {
  const distPath = path.resolve(__dirname, '..', 'dist', 'index.html');
  const mockPath = '/tmp/icon.png';
  const pixelDataUrl =
    'data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAQAAAC1HAwCAAAAC0lEQVR42mP8/x8AAusB9sYpWJ0AAAAASUVORK5CYII=';

  await page.addInitScript(({ mockPath, pixelDataUrl }) => {
    // Minimal __TAURI__ v2 mock to satisfy frontend bootstrap and image fallback calls.
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    (window as any).__TAURI__ = {
      core: {
        // eslint-disable-next-line @typescript-eslint/no-explicit-any
        invoke: (cmd: string, _args: any = {}) => {
          if (cmd === 'get_settings') {
            return Promise.resolve({
              last_file: mockPath,
              aspect_lock: false,
              fit_window: true,
            });
          }
          if (cmd === 'load_image_data') {
            return Promise.resolve(pixelDataUrl);
          }
          if (cmd === 'previous_file' || cmd === 'next_file') {
            return Promise.resolve();
          }
          return Promise.resolve(null);
        },
        convertFileSrc: () => 'file:///definitely-missing.png',
      },
      event: {
        listen: () => Promise.resolve(() => {}),
      },
    };
  }, { mockPath, pixelDataUrl });

  await page.goto(`file://${distPath}`);

  await expect(page.locator('#fileInfo')).toHaveText('icon.png');
  await expect(page.locator('#status')).toHaveText('');
  await expect(page.locator('#placeholder')).toHaveText('');
  await expect(page.locator('#imageContainer')).not.toHaveClass(/placeholder/);
});
