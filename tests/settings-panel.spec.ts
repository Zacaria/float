import { test, expect } from '@playwright/test';
import path from 'path';

test('opens settings and persists updated preferences with mocked tauri', async ({ page }) => {
  const distPath = path.resolve(__dirname, '..', 'dist', 'settings.html');

  await page.addInitScript(() => {
    const listeners: Record<string, Array<(event: { payload?: unknown }) => void>> = {};
    const invocations: Array<{ cmd: string; args: any }> = [];
    let currentSettings = {
      aspect_lock: false,
      click_through: false,
      slideshow_enabled: false,
      slideshow_interval_ms: 5000,
      opacity_percent: 80,
      blur_enabled: false,
      blur_supported: false,
    };

    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    (window as any).__TAURI__ = {
      core: {
        invoke: (cmd: string, args: any = {}) => {
          invocations.push({ cmd, args });
          if (cmd === 'get_settings') {
            return Promise.resolve(currentSettings);
          }
          if (cmd === 'set_settings') {
            currentSettings = { ...currentSettings, ...(args.update || {}) };
            return Promise.resolve(currentSettings);
          }
          if (cmd === 'load_image_data') {
            return Promise.resolve(
              'data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAQAAAC1HAwCAAAAC0lEQVR42mP8/x8AAusB9sYpWJ0AAAAASUVORK5CYII=',
            );
          }
          return Promise.resolve(null);
        },
        convertFileSrc: () => 'file:///definitely-missing.png',
      },
      event: {
        listen: (name: string, callback: (event: { payload?: unknown }) => void) => {
          listeners[name] ||= [];
          listeners[name].push(callback);
          return Promise.resolve(() => {});
        },
      },
    };

    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    (window as any).__dispatchTauri = (name: string, payload?: unknown) => {
      (listeners[name] || []).forEach((callback) => callback({ payload }));
    };

    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    (window as any).__getInvocations = () => invocations;
  });

  await page.goto(`file://${distPath}`);

  await expect(page.locator('#settingsTitle')).toHaveText('Settings');
  await expect(page.locator('#settingsOpacityValue')).toHaveText('80%');

  await page.locator('#settingsAspectLock').check({ force: true });
  await page.selectOption('#settingsSlideshowInterval', '10000');

  const invocations = await page.evaluate(() => {
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    return (window as any).__getInvocations();
  });

  expect(invocations).toEqual(
    expect.arrayContaining([
      expect.objectContaining({
        cmd: 'set_settings',
        args: { update: { aspect_lock: true } },
      }),
      expect.objectContaining({
        cmd: 'set_settings',
        args: { update: { slideshow_interval_ms: 10000 } },
      }),
    ]),
  );
});
