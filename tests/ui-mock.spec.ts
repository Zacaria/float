import { test, expect } from '@playwright/test';
import path from 'path';

test('bootstrap shows the default placeholder with mocked tauri settings', async ({ page }) => {
  const distPath = path.resolve(__dirname, '..', 'dist', 'index.html');

  await page.addInitScript(() => {
    const listeners: Record<string, Array<(event: { payload?: unknown }) => void>> = {};

    // Minimal __TAURI__ v2 mock to satisfy frontend bootstrap and image fallback calls.
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    (window as any).__TAURI__ = {
      core: {
        // eslint-disable-next-line @typescript-eslint/no-explicit-any
        invoke: (cmd: string, _args: any = {}) => {
          if (cmd === 'current_window_label') {
            return Promise.resolve('main');
          }
          if (cmd === 'get_settings') {
            return Promise.resolve({
              aspect_lock: false,
              click_through: false,
              slideshow_enabled: false,
              slideshow_interval_ms: 5000,
              opacity_percent: 100,
              blur_enabled: false,
              blur_supported: false,
            });
          }
          if (cmd === 'previous_file' || cmd === 'next_file') {
            return Promise.resolve();
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
  });

  await page.goto(`file://${distPath}`);

  await expect(page.locator('#fileInfo')).toHaveText('No file selected');
  await expect(page.locator('#status')).toHaveText('Idle');
  await expect(page.locator('#placeholderTitle')).toHaveText('No image loaded');
  await expect(page.locator('#imageContainer')).toHaveClass(/placeholder/);
  await expect(page.locator('#controls')).toBeHidden();
});

test('active-file-changed renders the selected image with mocked tauri', async ({ page }) => {
  const distPath = path.resolve(__dirname, '..', 'dist', 'index.html');
  const mockPath = '/tmp/icon.png';
  const pixelDataUrl =
    'data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAQAAAC1HAwCAAAAC0lEQVR42mP8/x8AAusB9sYpWJ0AAAAASUVORK5CYII=';

  await page.addInitScript(({ pixelDataUrl }) => {
    const listeners: Record<string, Array<(event: { payload?: unknown }) => void>> = {};

    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    (window as any).__TAURI__ = {
      core: {
        // eslint-disable-next-line @typescript-eslint/no-explicit-any
        invoke: (cmd: string, _args: any = {}) => {
          if (cmd === 'current_window_label') {
            return Promise.resolve('main');
          }
          if (cmd === 'get_settings') {
            return Promise.resolve({
              aspect_lock: false,
              click_through: false,
              slideshow_enabled: false,
              slideshow_interval_ms: 5000,
              opacity_percent: 100,
              blur_enabled: false,
              blur_supported: false,
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
  }, { pixelDataUrl });

  await page.goto(`file://${distPath}`);
  await page.evaluate((mockPath) => {
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    (window as any).__dispatchTauri('active-file-changed:main', { path: mockPath, index: 0, total: 1 });
  }, mockPath);

  await expect(page.locator('#fileInfo')).toHaveText('icon.png');
  await expect(page.locator('#status')).toHaveText('Single image');
  await expect(page.locator('#imageContainer')).not.toHaveClass(/placeholder/);
  await expect(page.locator('#controls')).toBeHidden();
});

test('active-file-changed renders a missing-file state when the image is gone', async ({ page }) => {
  const distPath = path.resolve(__dirname, '..', 'dist', 'index.html');
  const missingPath = '/tmp/missing-shot.png';

  await page.addInitScript(() => {
    const listeners: Record<string, Array<(event: { payload?: unknown }) => void>> = {};

    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    (window as any).__TAURI__ = {
      core: {
        // eslint-disable-next-line @typescript-eslint/no-explicit-any
        invoke: (cmd: string, _args: any = {}) => {
          if (cmd === 'current_window_label') {
            return Promise.resolve('main');
          }
          if (cmd === 'get_settings') {
            return Promise.resolve({
              aspect_lock: false,
              click_through: false,
              slideshow_enabled: false,
              slideshow_interval_ms: 5000,
              opacity_percent: 100,
              blur_enabled: false,
              blur_supported: false,
            });
          }
          if (cmd === 'load_image_data') {
            return Promise.reject('file does not exist');
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
  });

  await page.goto(`file://${distPath}`);
  await page.evaluate((missingPath) => {
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    (window as any).__dispatchTauri('active-file-changed:main', { path: missingPath, index: 0, total: 1 });
  }, missingPath);

  await expect(page.locator('#imageContainer')).toHaveClass(/placeholder/);
  await expect(page.locator('#fileInfo')).toHaveText('missing-shot.png');
  await expect(page.locator('#status')).toHaveText('Missing file');
  await expect(page.locator('#placeholderTitle')).toHaveText('Image not found');
  await expect(page.locator('#placeholderAction')).toContainText('choose it again');
});

test('active-file-changed renders a load-failed state when the image payload is unreadable', async ({ page }) => {
  const distPath = path.resolve(__dirname, '..', 'dist', 'index.html');
  const brokenPath = '/tmp/broken-shot.png';

  await page.addInitScript(() => {
    const listeners: Record<string, Array<(event: { payload?: unknown }) => void>> = {};

    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    (window as any).__TAURI__ = {
      core: {
        // eslint-disable-next-line @typescript-eslint/no-explicit-any
        invoke: (cmd: string, _args: any = {}) => {
          if (cmd === 'current_window_label') {
            return Promise.resolve('main');
          }
          if (cmd === 'get_settings') {
            return Promise.resolve({
              aspect_lock: false,
              click_through: false,
              slideshow_enabled: false,
              slideshow_interval_ms: 5000,
              opacity_percent: 100,
              blur_enabled: false,
              blur_supported: false,
            });
          }
          if (cmd === 'load_image_data') {
            return Promise.resolve('data:image/png;base64,not-valid-image-data');
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
  });

  await page.goto(`file://${distPath}`);
  await page.evaluate((brokenPath) => {
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    (window as any).__dispatchTauri('active-file-changed:main', { path: brokenPath, index: 0, total: 1 });
  }, brokenPath);

  await expect(page.locator('#imageContainer')).toHaveClass(/placeholder/);
  await expect(page.locator('#fileInfo')).toHaveText('broken-shot.png');
  await expect(page.locator('#status')).toHaveText('Load failed');
  await expect(page.locator('#placeholderTitle')).toHaveText('Could not open image');
  await expect(page.locator('#placeholderAction')).toContainText('choose a different file');
});
